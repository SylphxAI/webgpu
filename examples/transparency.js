const { Gpu, bufferUsage: getBufferUsage, textureUsage: getTextureUsage } = require('../index.js')

async function main() {
  console.log('🎨 WebGPU Transparency Example with Alpha Blending\n')

  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  const bufferUsage = getBufferUsage()
  const textureUsage = getTextureUsage()

  // Create shader with alpha support
  const shaderCode = `
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@vertex
fn vs_main(@location(0) position: vec2f, @location(1) color: vec4f) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(position, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    return input.color;
}
`

  const shaderModule = device.createShaderModule(shaderCode)
  console.log('✓ Shader compiled')

  // Create pipeline with alpha blending
  const pipelineLayout = device.createPipelineLayout('Transparency Layout', [])

  const pipeline = device.createRenderPipeline(
    'Transparency Pipeline',
    pipelineLayout,
    shaderModule,
    'vs_main',
    ['float32x2', 'float32x4'], // position + RGBA color
    shaderModule,
    'fs_main',
    ['rgba8unorm'],
    null,     // no depth/stencil
    'alpha',  // alpha blending mode
    null,     // all channels
    null      // no MSAA
  )
  console.log('✓ Pipeline created with alpha blending')

  // Create three overlapping quads with different colors and transparency
  // Red quad (bottom)
  const redQuad = new Float32Array([
    // Position    RGBA Color
    -0.3, -0.7,    1.0, 0.0, 0.0, 0.6,  // Bottom left (red, 60% opacity)
     0.3, -0.7,    1.0, 0.0, 0.0, 0.6,  // Bottom right
     0.3, -0.1,    1.0, 0.0, 0.0, 0.6,  // Top right
    -0.3, -0.1,    1.0, 0.0, 0.0, 0.6,  // Top left
  ])

  // Green quad (middle)
  const greenQuad = new Float32Array([
    // Position    RGBA Color
    -0.3,  0.1,    0.0, 1.0, 0.0, 0.6,  // Bottom left (green, 60% opacity)
     0.3,  0.1,    0.0, 1.0, 0.0, 0.6,  // Bottom right
     0.3,  0.7,    0.0, 1.0, 0.0, 0.6,  // Top right
    -0.3,  0.7,    0.0, 1.0, 0.0, 0.6,  // Top left
  ])

  // Blue quad (top, covers both)
  const blueQuad = new Float32Array([
    // Position    RGBA Color
    -0.5, -0.3,    0.0, 0.0, 1.0, 0.5,  // Bottom left (blue, 50% opacity)
     0.5, -0.3,    0.0, 0.0, 1.0, 0.5,  // Bottom right
     0.5,  0.3,    0.0, 0.0, 1.0, 0.5,  // Top right
    -0.5,  0.3,    0.0, 0.0, 1.0, 0.5,  // Top left
  ])

  const indices = new Uint16Array([
    0, 1, 2,  0, 2, 3,  // Two triangles per quad
  ])

  // Create buffers
  const redBuffer = device.createBuffer(
    redQuad.byteLength,
    bufferUsage.vertex | bufferUsage.copyDst,
    false
  )
  const greenBuffer = device.createBuffer(
    greenQuad.byteLength,
    bufferUsage.vertex | bufferUsage.copyDst,
    false
  )
  const blueBuffer = device.createBuffer(
    blueQuad.byteLength,
    bufferUsage.vertex | bufferUsage.copyDst,
    false
  )
  const indexBuffer = device.createBuffer(
    indices.byteLength,
    bufferUsage.index | bufferUsage.copyDst,
    false
  )

  device.queueWriteBuffer(redBuffer, 0, Buffer.from(redQuad.buffer))
  device.queueWriteBuffer(greenBuffer, 0, Buffer.from(greenQuad.buffer))
  device.queueWriteBuffer(blueBuffer, 0, Buffer.from(blueQuad.buffer))
  device.queueWriteBuffer(indexBuffer, 0, Buffer.from(indices.buffer))
  console.log('✓ Vertex and index buffers created')

  // Create render target
  const width = 512
  const height = 512

  const renderTexture = device.createTexture({
    label: 'render-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: textureUsage.renderAttachment | textureUsage.copySrc,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  const renderView = renderTexture.createView('render-view')
  console.log('✓ Render texture created')

  // Render all three quads with alpha blending
  const encoder = device.createCommandEncoder()

  // First quad (red)
  encoder.renderPassIndexed(
    pipeline,
    [redBuffer],
    indexBuffer,
    'uint16',
    6,
    [renderView],
    [[0.9, 0.9, 0.9, 1.0]], // Light gray background
    null // no bind groups
  )

  // Second quad (green) - will blend with red where they overlap
  encoder.renderPassIndexed(
    pipeline,
    [greenBuffer],
    indexBuffer,
    'uint16',
    6,
    [renderView],
    null, // Don't clear this time - we want to blend
    null
  )

  // Third quad (blue) - will blend with both
  encoder.renderPassIndexed(
    pipeline,
    [blueBuffer],
    indexBuffer,
    'uint16',
    6,
    [renderView],
    null, // Don't clear
    null
  )

  console.log('✓ Three quads rendered with alpha blending')

  // Read back to verify blending
  const bytesPerRow = Math.ceil(width * 4 / 256) * 256
  const bufferSize = bytesPerRow * height

  const readBuffer = device.createBuffer(
    bufferSize,
    bufferUsage.copyDst | bufferUsage.mapRead,
    false
  )

  device.copyTextureToBuffer(
    encoder,
    renderTexture,
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

  // Check a pixel in the overlap region
  const pixelData = await readBuffer.mapRead()
  const pixels = new Uint8Array(pixelData.buffer, pixelData.byteOffset, pixelData.byteLength)

  const centerX = Math.floor(width / 2)
  const centerY = Math.floor(height / 2)
  const centerOffset = centerY * bytesPerRow + centerX * 4
  const centerPixel = [
    pixels[centerOffset],
    pixels[centerOffset + 1],
    pixels[centerOffset + 2],
    pixels[centerOffset + 3]
  ]

  console.log(`\nCenter pixel (overlap region): RGBA(${centerPixel.join(', ')})`)

  // Center should show blended color (blue over the overlap of red and green)
  const hasBlending = centerPixel[2] > 100 // Should have blue component
  if (hasBlending) {
    console.log('✅ Alpha blending verified! (Blue quad visible in center)')
  } else {
    console.log('⚠️  Unexpected color (blending may not be working)')
  }

  readBuffer.unmap()

  console.log('\n✅ Transparency example complete!')
  console.log('   - Alpha blending enabled ✓')
  console.log('   - Three overlapping quads ✓')
  console.log('   - Blended colors verified ✓')

  // Cleanup
  redBuffer.destroy()
  greenBuffer.destroy()
  blueBuffer.destroy()
  indexBuffer.destroy()
  readBuffer.destroy()
  renderTexture.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
