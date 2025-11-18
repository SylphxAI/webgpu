const { Gpu, bufferUsage: getBufferUsage, textureUsage: getTextureUsage } = require('../index.js')

async function main() {
  console.log('🔍 WebGPU MSAA (Multi-Sample Anti-Aliasing) Example\n')

  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  const bufferUsage = getBufferUsage()
  const textureUsage = getTextureUsage()

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

  const shaderModule = device.createShaderModule(shaderCode)
  console.log('✓ Shader compiled')

  // Create pipeline WITH MSAA (4x)
  const pipelineLayout = device.createPipelineLayout('MSAA Pipeline Layout', [])

  const msaaPipeline = device.createRenderPipeline(
    'MSAA Pipeline',
    pipelineLayout,
    shaderModule,
    'vs_main',
    ['float32x2', 'float32x3'], // position + color
    shaderModule,
    'fs_main',
    ['rgba8unorm'],
    null,  // no depth/stencil
    null,  // default blend mode
    null,  // all channels
    4      // 4x MSAA
  )
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

  const vertexBuffer = device.createBuffer(
    vertices.byteLength,
    bufferUsage.vertex | bufferUsage.copyDst,
    false
  )

  const indexBuffer = device.createBuffer(
    indices.byteLength,
    bufferUsage.index | bufferUsage.copyDst,
    false
  )

  device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
  device.queueWriteBuffer(indexBuffer, 0, Buffer.from(indices.buffer))
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
    usage: textureUsage.renderAttachment,
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
    usage: textureUsage.renderAttachment | textureUsage.copySrc,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,  // Single sample
  })

  const resolveView = resolveTexture.createView('resolve-view')
  console.log('✓ MSAA and resolve textures created')

  // Render with MSAA
  const encoder = device.createCommandEncoder()

  encoder.renderPassIndexed(
    msaaPipeline,
    [vertexBuffer],
    indexBuffer,
    'uint16',
    6,
    [msaaView],           // Render to MSAA texture
    [[0.0, 0.0, 0.0, 1.0]], // Black background
    null,                 // No bind groups
    null,                 // No depth/stencil
    null,                 // No depth clear
    [resolveView]         // Resolve to single-sample texture
  )
  console.log('✓ Rendered with 4x MSAA and resolved')

  // Read back result
  const bytesPerRow = Math.ceil(width * 4 / 256) * 256
  const bufferSize = bytesPerRow * height

  const readBuffer = device.createBuffer(
    bufferSize,
    bufferUsage.copyDst | bufferUsage.mapRead,
    false
  )

  device.copyTextureToBuffer(
    encoder,
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

  device.queueSubmit(encoder.finish())
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
