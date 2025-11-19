/**
 * WebGPU Standard Render Pipeline Tests
 *
 * Comprehensive tests for render pipelines, render passes, and rendering operations.
 * All APIs conform to W3C WebGPU specification.
 */

import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, GPUBufferUsage, GPUTextureUsage } from '../webgpu.js'

describe('WebGPU Standard: Render Pipeline', () => {
  let device: Awaited<ReturnType<Awaited<ReturnType<ReturnType<typeof Gpu>['requestAdapter']>>['requestDevice']>>

  beforeAll(async () => {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create basic render pipeline (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
          return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
      `
    })

    // Standard: createRenderPipeline with vertex and fragment
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [
          {
            format: 'rgba8unorm'
          }
        ]
      },
      primitive: {
        topology: 'triangle-list'
      }
    })

    expect(pipeline).toBeDefined()
  })

  test('should create pipeline with vertex buffers (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        struct VertexInput {
          @location(0) position: vec3<f32>,
          @location(1) color: vec3<f32>
        }

        @vertex
        fn vs_main(input: VertexInput) -> @builtin(position) vec4<f32> {
          return vec4<f32>(input.position, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 1.0, 1.0, 1.0);
        }
      `
    })

    // Standard: vertex.buffers array with vertex attributes
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main',
        buffers: [
          {
            arrayStride: 24, // 6 floats * 4 bytes
            attributes: [
              {
                shaderLocation: 0,
                offset: 0,
                format: 'float32x3'
              },
              {
                shaderLocation: 1,
                offset: 12,
                format: 'float32x3'
              }
            ]
          }
        ]
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      }
    })

    expect(pipeline).toBeDefined()
  })

  test('should support primitive state options (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main() -> @builtin(position) vec4<f32> {
          return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
      `
    })

    // Standard: primitive with topology, cullMode, frontFace
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      },
      primitive: {
        topology: 'triangle-strip',
        cullMode: 'back',
        frontFace: 'ccw'
      }
    })

    expect(pipeline).toBeDefined()
  })

  test('should support blend state (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main() -> @builtin(position) vec4<f32> {
          return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 0.5);
        }
      `
    })

    // Standard: fragment targets with blend
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [
          {
            format: 'rgba8unorm',
            blend: {
              color: {
                srcFactor: 'src-alpha',
                dstFactor: 'one-minus-src-alpha',
                operation: 'add'
              },
              alpha: {
                srcFactor: 'one',
                dstFactor: 'zero',
                operation: 'add'
              }
            }
          }
        ]
      }
    })

    expect(pipeline).toBeDefined()
  })

  test('should support depth stencil state (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main() -> @builtin(position) vec4<f32> {
          return vec4<f32>(0.0, 0.0, 0.5, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
      `
    })

    // Standard: depthStencil configuration
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      },
      depthStencil: {
        format: 'depth24plus',
        depthWriteEnabled: true,
        depthCompare: 'less'
      }
    })

    expect(pipeline).toBeDefined()
  })

  test('should support multisample state (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main() -> @builtin(position) vec4<f32> {
          return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
      `
    })

    // Standard: multisample with count
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      },
      multisample: {
        count: 4
      }
    })

    expect(pipeline).toBeDefined()
  })
})

describe('WebGPU Standard: Render Pass', () => {
  let device: Awaited<ReturnType<Awaited<ReturnType<ReturnType<typeof Gpu>['requestAdapter']>>['requestDevice']>>

  beforeAll(async () => {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create render pass with color attachment (standard)', () => {
    // Create render target texture
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC
    })

    const view = texture.createView()

    const encoder = device.createCommandEncoder()

    // Standard: beginRenderPass with colorAttachments
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: view,
          clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    expect(renderPass).toBeDefined()
    renderPass.end()

    encoder.finish()
    texture.destroy()
  })

  test('should support multiple color attachments (standard)', () => {
    const texture1 = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const texture2 = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const encoder = device.createCommandEncoder()

    // Standard: Multiple color attachments (MRT)
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture1.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        },
        {
          view: texture2.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    renderPass.end()
    encoder.finish()

    texture1.destroy()
    texture2.destroy()
  })

  test('should draw in render pass (standard)', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
          var pos = array<vec2<f32>, 3>(
            vec2<f32>(0.0, 0.5),
            vec2<f32>(-0.5, -0.5),
            vec2<f32>(0.5, -0.5)
          );
          return vec4<f32>(pos[idx], 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
      `
    })

    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      }
    })

    const encoder = device.createCommandEncoder()
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    // Standard: setPipeline and draw
    renderPass.setPipeline(pipeline)
    renderPass.draw(3, 1, 0, 0)
    renderPass.end()

    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    texture.destroy()
  })

  test('should support indexed drawing (standard)', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    // Create vertex buffer
    const vertexBuffer = device.createBuffer({
      size: 48, // 4 vertices * 12 bytes (3 floats)
      usage: GPUBufferUsage.VERTEX
    })

    // Create index buffer
    const indexBuffer = device.createBuffer({
      size: 12, // 6 indices * 2 bytes (uint16)
      usage: GPUBufferUsage.INDEX
    })

    const shaderModule = device.createShaderModule({
      code: `
        @vertex
        fn vs_main(@location(0) pos: vec3<f32>) -> @builtin(position) vec4<f32> {
          return vec4<f32>(pos, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
      `
    })

    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main',
        buffers: [
          {
            arrayStride: 12,
            attributes: [
              {
                shaderLocation: 0,
                offset: 0,
                format: 'float32x3'
              }
            ]
          }
        ]
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      }
    })

    const encoder = device.createCommandEncoder()
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    renderPass.setPipeline(pipeline)
    // Standard: setVertexBuffer, setIndexBuffer, drawIndexed
    renderPass.setVertexBuffer(0, vertexBuffer)
    renderPass.setIndexBuffer(indexBuffer, 'uint16')
    renderPass.drawIndexed(6, 1, 0, 0, 0)
    renderPass.end()

    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    texture.destroy()
    vertexBuffer.destroy()
    indexBuffer.destroy()
  })

  test('should support viewport and scissor (standard)', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const encoder = device.createCommandEncoder()
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    // Standard: setViewport and setScissorRect
    renderPass.setViewport(0, 0, 256, 256, 0, 1)
    renderPass.setScissorRect(0, 0, 256, 256)
    renderPass.end()

    encoder.finish()
    texture.destroy()
  })

  test('should support blend constant (standard)', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const encoder = device.createCommandEncoder()
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    // Standard: setBlendConstant
    renderPass.setBlendConstant([1.0, 0.0, 0.0, 1.0])
    renderPass.end()

    encoder.finish()
    texture.destroy()
  })

  test('should support stencil reference (standard)', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const encoder = device.createCommandEncoder()
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    // Standard: setStencilReference
    renderPass.setStencilReference(0xFF)
    renderPass.end()

    encoder.finish()
    texture.destroy()
  })

  test('should support debug groups (standard)', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT
    })

    const encoder = device.createCommandEncoder()
    const renderPass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: texture.createView(),
          loadOp: 'clear',
          storeOp: 'store'
        }
      ]
    })

    // Standard: debug groups and markers
    renderPass.pushDebugGroup('render-group')
    renderPass.insertDebugMarker('render-marker')
    renderPass.popDebugGroup()
    renderPass.end()

    encoder.finish()
    texture.destroy()
  })
})

describe('WebGPU Standard: Render Pipeline Auto Layout', () => {
  let device: Awaited<ReturnType<Awaited<ReturnType<ReturnType<typeof Gpu>['requestAdapter']>>['requestDevice']>>

  beforeAll(async () => {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create pipeline with auto layout (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @group(0) @binding(0)
        var<uniform> color: vec4<f32>;

        @vertex
        fn vs_main() -> @builtin(position) vec4<f32> {
          return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
          return color;
        }
      `
    })

    // Standard: layout: null for automatic layout
    const pipeline = device.createRenderPipeline({
      layout: null,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main'
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      }
    })

    // Standard: getBindGroupLayout for auto-generated layouts
    const bindGroupLayout = pipeline.getBindGroupLayout(0)
    expect(bindGroupLayout).toBeDefined()
  })
})
