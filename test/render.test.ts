import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, bufferUsage, textureUsage } from '../index.js'

describe('GPU Render Pipeline', () => {
  let device: any
  const bufUsage = bufferUsage()
  const texUsage = textureUsage()

  beforeAll(async () => {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create render pipeline', () => {
    const shaderCode = `
      @vertex
      fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
        return vec4f(pos, 1.0);
      }

      @fragment
      fn fs_main() -> @location(0) vec4f {
        return vec4f(1.0, 0.0, 0.0, 1.0); // Red
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const pipelineLayout = device.createPipelineLayout('Layout', [])

    const pipeline = device.createRenderPipeline(
      'Render Pipeline',
      pipelineLayout,
      shader,
      'vs_main',
      ['float32x3'],
      shader,
      'fs_main',
      ['rgba8unorm'],
      null, // no depth
      null, // default blend
      null, // all channels
      null  // no MSAA
    )

    expect(pipeline).toBeDefined()
  })

  test('should render triangle', async () => {
    const width = 256
    const height = 256

    const shaderCode = `
      @vertex
      fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
        return vec4f(pos, 1.0);
      }

      @fragment
      fn fs_main() -> @location(0) vec4f {
        return vec4f(1.0, 0.0, 0.0, 1.0); // Red
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const pipelineLayout = device.createPipelineLayout('Layout', [])
    const pipeline = device.createRenderPipeline(
      'Triangle Pipeline',
      pipelineLayout,
      shader,
      'vs_main',
      ['float32x3'],
      shader,
      'fs_main',
      ['rgba8unorm'],
      null, null, null, null
    )

    // Triangle vertices
    const vertices = new Float32Array([
      0.0, 0.5, 0.0,    // top
      -0.5, -0.5, 0.0,  // bottom left
      0.5, -0.5, 0.0,   // bottom right
    ])

    const vertexBuffer = device.createBuffer(
      vertices.byteLength,
      bufUsage.vertex | bufUsage.copyDst,
      false
    )
    device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

    // Render texture
    const texture = device.createTexture({
      width,
      height,
      format: 'rgba8unorm',
      usage: texUsage.renderAttachment | texUsage.copySrc,
    })
    const textureView = texture.createView('Render Target')

    // Render
    const encoder = device.createCommandEncoder()
    encoder.renderPass(
      pipeline,
      [vertexBuffer],
      3, // vertex count
      [textureView],
      [[0.0, 0.0, 0.0, 1.0]], // black background
      null, // no bind groups
      null, // no depth
      null, // no clear depth
      null  // no resolve targets
    )

    // Read back
    const readBuffer = device.createBuffer(
      width * height * 4,
      bufUsage.copyDst | bufUsage.mapRead,
      false
    )
    device.copyTextureToBuffer(
      encoder,
      texture,
      0, // mip level
      0, 0, 0, // origin
      readBuffer,
      0, // offset
      width * 4, // bytes per row
      height,
      width,
      height,
      1 // depth
    )

    device.queueSubmit(encoder.finish())
    device.poll(true)

    const data = await readBuffer.mapRead()
    const pixels = new Uint8Array(data.buffer, data.byteOffset, data.byteLength)

    // Check center pixel (should be red from triangle)
    const centerX = Math.floor(width / 2)
    const centerY = Math.floor(height / 2)
    const offset = (centerY * width + centerX) * 4

    expect(pixels[offset]).toBe(255)     // R
    expect(pixels[offset + 1]).toBe(0)   // G
    expect(pixels[offset + 2]).toBe(0)   // B
    expect(pixels[offset + 3]).toBe(255) // A

    readBuffer.unmap()
    vertexBuffer.destroy()
    readBuffer.destroy()
    texture.destroy()
  })

  test('should render with indexed drawing', async () => {
    const shaderCode = `
      @vertex
      fn vs_main(@location(0) pos: vec2f) -> @builtin(position) vec4f {
        return vec4f(pos, 0.0, 1.0);
      }

      @fragment
      fn fs_main() -> @location(0) vec4f {
        return vec4f(0.0, 1.0, 0.0, 1.0); // Green
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const pipelineLayout = device.createPipelineLayout('Layout', [])
    const pipeline = device.createRenderPipeline(
      'Indexed Pipeline',
      pipelineLayout,
      shader,
      'vs_main',
      ['float32x2'],
      shader,
      'fs_main',
      ['rgba8unorm'],
      null, null, null, null
    )

    // Quad vertices
    const vertices = new Float32Array([
      -0.5, 0.5,   // top-left
      0.5, 0.5,    // top-right
      0.5, -0.5,   // bottom-right
      -0.5, -0.5,  // bottom-left
    ])

    const indices = new Uint16Array([0, 1, 2, 0, 2, 3])

    const vertexBuffer = device.createBuffer(vertices.byteLength, bufUsage.vertex | bufUsage.copyDst, false)
    const indexBuffer = device.createBuffer(indices.byteLength, bufUsage.index | bufUsage.copyDst, false)

    device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
    device.queueWriteBuffer(indexBuffer, 0, Buffer.from(indices.buffer))

    const texture = device.createTexture({
      width: 128,
      height: 128,
      format: 'rgba8unorm',
      usage: texUsage.renderAttachment | texUsage.copySrc,
    })
    const textureView = texture.createView('Target')

    const encoder = device.createCommandEncoder()
    encoder.renderPassIndexed(
      pipeline,
      [vertexBuffer],
      indexBuffer,
      'uint16',
      6, // index count
      [textureView],
      [[0.0, 0.0, 0.0, 1.0]],
      null, null, null, null
    )

    device.queueSubmit(encoder.finish())
    device.poll(true)

    vertexBuffer.destroy()
    indexBuffer.destroy()
    texture.destroy()
  })

  test('should render with blend modes', () => {
    const blendModes = ['replace', 'alpha', 'additive', 'premultiplied']

    blendModes.forEach(blendMode => {
      const shaderCode = `
        @vertex
        fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
          return vec4f(pos, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4f {
          return vec4f(1.0, 0.0, 0.0, 0.5);
        }
      `

      const shader = device.createShaderModule(shaderCode)
      const pipelineLayout = device.createPipelineLayout('Layout', [])

      const pipeline = device.createRenderPipeline(
        `Pipeline ${blendMode}`,
        pipelineLayout,
        shader,
        'vs_main',
        ['float32x3'],
        shader,
        'fs_main',
        ['rgba8unorm'],
        null, // no depth
        blendMode,
        null, // all channels
        null  // no MSAA
      )

      expect(pipeline).toBeDefined()
    })
  })

  test('should render with MSAA', () => {
    const sampleCounts = [1, 4] // Only test widely supported sample counts

    sampleCounts.forEach(sampleCount => {
      const shaderCode = `
        @vertex
        fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
          return vec4f(pos, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4f {
          return vec4f(1.0, 1.0, 1.0, 1.0);
        }
      `

      const shader = device.createShaderModule(shaderCode)
      const pipelineLayout = device.createPipelineLayout('Layout', [])

      const pipeline = device.createRenderPipeline(
        `MSAA ${sampleCount}x Pipeline`,
        pipelineLayout,
        shader,
        'vs_main',
        ['float32x3'],
        shader,
        'fs_main',
        ['rgba8unorm'],
        null, // no depth
        null, // default blend
        null, // all channels
        sampleCount
      )

      expect(pipeline).toBeDefined()
    })
  })

  test('should create render bundle', () => {
    const shaderCode = `
      @vertex
      fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
        return vec4f(pos, 1.0);
      }

      @fragment
      fn fs_main() -> @location(0) vec4f {
        return vec4f(1.0, 0.0, 0.0, 1.0);
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const pipelineLayout = device.createPipelineLayout('Layout', [])
    const pipeline = device.createRenderPipeline(
      'Bundle Pipeline',
      pipelineLayout,
      shader,
      'vs_main',
      ['float32x3'],
      shader,
      'fs_main',
      ['rgba8unorm'],
      null, null, null, null
    )

    const vertices = new Float32Array([0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0])
    const vertexBuffer = device.createBuffer(vertices.byteLength, bufUsage.vertex | bufUsage.copyDst, false)
    device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

    const bundle = device.createRenderBundle(
      'Test Bundle',
      pipeline,
      [vertexBuffer],
      3, // vertex count
      null, // no bind groups
      ['rgba8unorm']
    )

    expect(bundle).toBeDefined()
    expect(bundle).toHaveProperty('destroy')

    bundle.destroy()
    vertexBuffer.destroy()
  })

  test('should execute render bundles', () => {
    const shaderCode = `
      @vertex
      fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
        return vec4f(pos, 1.0);
      }

      @fragment
      fn fs_main() -> @location(0) vec4f {
        return vec4f(1.0, 1.0, 1.0, 1.0);
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const pipelineLayout = device.createPipelineLayout('Layout', [])
    const pipeline = device.createRenderPipeline(
      'Bundle Pipeline',
      pipelineLayout,
      shader,
      'vs_main',
      ['float32x3'],
      shader,
      'fs_main',
      ['rgba8unorm'],
      null, null, null, null
    )

    const vertices = new Float32Array([0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0])
    const vertexBuffer = device.createBuffer(vertices.byteLength, bufUsage.vertex | bufUsage.copyDst, false)
    device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

    const bundle = device.createRenderBundle('Bundle', pipeline, [vertexBuffer], 3, null, ['rgba8unorm'])

    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: texUsage.renderAttachment,
    })
    const textureView = texture.createView('View')

    const encoder = device.createCommandEncoder()
    encoder.renderPassBundles([bundle], [textureView], [[0.0, 0.0, 0.0, 1.0]])
    device.queueSubmit(encoder.finish())
    device.poll(true)

    bundle.destroy()
    vertexBuffer.destroy()
    texture.destroy()
  })
})
