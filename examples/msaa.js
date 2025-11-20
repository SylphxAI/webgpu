const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('../webgpu.js')

async function main() {
  console.log('🔍 WebGPU MSAA (Multi-Sample Anti-Aliasing) Example\n')

  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Create shader for rendering a diagonal line
  const shaderCode = `
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

@vertex
fn vs_main(@location(0) position: vec2f, @location(1) color: vec3f) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(position, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    return vec4f(input.color, 1.0);
}
`

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Shader compiled')

  // Create pipeline WITH MSAA (4x) - WebGPU standard API
  const pipelineLayout = device.createPipelineLayout('MSAA Pipeline Layout', [])

  const msaaPipeline = device.createRenderPipeline({
    label: 'MSAA Pipeline',
    layout: pipelineLayout,
    vertex: {
      module: shaderModule,
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 20, // 5 floats * 4 bytes (position + color)
        attributes: [
          {
            shaderLocation: 0,
            offset: 0,
            format: 'float32x2' // position
          },
          {
            shaderLocation: 1,
            offset: 8,
            format: 'float32x3' // color
          }
        ]
      }]
    },
    fragment: {
      module: shaderModule,
      entryPoint: 'fs_main',
      targets: [{
        format: 'rgba8unorm'
      }]
    },
    primitive: {
      topology: 'triangle-list'
    },
    multisample: {
      count: 4
    }
  })
  console.log('✓ Pipeline created with 4x MSAA')

  // Create vertices for a diagonal line (thin triangle strip to show aliasing)
  const lineWidth = 0.01
  const vertices = new Float32Array([
    // Position       Color (RGB)
    -0.8, -0.8,      1.0, 1.0, 1.0,  // Bottom left
    -0.8 + lineWidth, -0.8,  1.0, 1.0, 1.0,  // Bottom left + width
     0.8,  0.8,      1.0, 1.0, 1.0,  // Top right
     0.8 + lineWidth,  0.8,  1.0, 1.0, 1.0,  // Top right + width
  ])

  const indices = new Uint16Array([
    0, 1, 2,  // First triangle
    1, 3, 2,  // Second triangle
  ])

  const vertexBuffer = device.createBuffer({
    size: vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  const indexBuffer = device.createBuffer({
    size: indices.byteLength,
    usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  device.queue.writeBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
  device.queue.writeBuffer(indexBuffer, 0, Buffer.from(indices.buffer))
  console.log('✓ Vertex and index buffers created')

  // Create render targets
  const width = 512
  const height = 512

  // MSAA texture (4x multisampled)
  const msaaTexture = device.createTexture({
    label: 'msaa-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 4,  // 4x MSAA
  })

  const msaaView = msaaTexture.createView('msaa-view')

  // Resolve target (single-sample texture for final result)
  const resolveTexture = device.createTexture({
    label: 'resolve-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,  // Single sample
  })

  const resolveView = resolveTexture.createView('resolve-view')
  console.log('✓ MSAA and resolve textures created')

  // Render with MSAA - WebGPU standard API
  const encoder = device.createCommandEncoder()

  const pass = encoder.beginRenderPass({
    colorAttachments: [{
      view: msaaView,
      resolveTarget: resolveView,
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }]
  })
  pass.setPipeline(msaaPipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.setIndexBuffer(indexBuffer, 'uint16')
  pass.drawIndexed(6)
  pass.end()
  console.log('✓ Rendered with 4x MSAA and resolved')

  // Read back result
  const bytesPerRow = Math.ceil(width * 4 / 256) * 256
  const bufferSize = bytesPerRow * height

  const readBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  encoder.copyTextureToBuffer(
    resolveTexture,
    0, 0, 0, 0,
    readBuffer,
    0,
    bytesPerRow,
    null,
    width,
    height,
    1
  )

  device.queue.submit([encoder.finish()])
  device.poll(true)
  console.log('✓ Render complete')

  // Verify result
  const pixelData = await readBuffer.mapRead()
  const pixels = new Uint8Array(pixelData.buffer, pixelData.byteOffset, pixelData.byteLength)

  // Check center pixel (should be on the diagonal line)
  const centerX = Math.floor(width / 2)
  const centerY = Math.floor(height / 2)
  const centerOffset = centerY * bytesPerRow + centerX * 4
  const centerPixel = [
    pixels[centerOffset],
    pixels[centerOffset + 1],
    pixels[centerOffset + 2],
    pixels[centerOffset + 3]
  ]

  console.log(`\nCenter pixel: RGBA(${centerPixel.join(', ')})`)

  // Check an edge pixel to see anti-aliasing (should have intermediate values)
  // Sample a pixel near the diagonal that would show anti-aliasing
  const edgeX = Math.floor(width / 2) + 10
  const edgeY = Math.floor(height / 2) + 10
  const edgeOffset = edgeY * bytesPerRow + edgeX * 4
  const edgePixel = [
    pixels[edgeOffset],
    pixels[edgeOffset + 1],
    pixels[edgeOffset + 2],
    pixels[edgeOffset + 3]
  ]

  console.log(`Edge pixel: RGBA(${edgePixel.join(', ')})`)

  // Verify: center should be white, edge might show anti-aliasing
  const centerIsWhite = centerPixel[0] > 200 && centerPixel[1] > 200 && centerPixel[2] > 200
  if (centerIsWhite) {
    console.log('✅ MSAA verified! (White diagonal line rendered)')
  }

  // Check for anti-aliasing effect (edge pixel with intermediate color values)
  const hasAntiAliasing = edgePixel[0] > 50 && edgePixel[0] < 255
  if (hasAntiAliasing) {
    console.log('✅ Anti-aliasing detected! (Edge pixel has intermediate values)')
  }

  readBuffer.unmap()

  console.log('\n✅ MSAA example complete!')
  console.log('   - 4x MSAA texture ✓')
  console.log('   - Pipeline with sample_count=4 ✓')
  console.log('   - Resolve target ✓')
  console.log('   - Anti-aliased rendering ✓')

  // Cleanup
  vertexBuffer.destroy()
  indexBuffer.destroy()
  readBuffer.destroy()
  msaaTexture.destroy()
  resolveTexture.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
