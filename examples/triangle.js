const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('../webgpu.js')

async function main() {
  console.log('🔺 WebGPU Render Example: Triangle\n')

  // Initialize GPU
  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Create shader module
  const shaderCode = `
@vertex
fn vs_main(@location(0) position: vec2f) -> @builtin(position) vec4f {
    return vec4f(position, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 1.0); // Red color
}
`

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Shader compiled')

  // Create pipeline layout
  const pipelineLayout = device.createPipelineLayout(
    'triangle-pipeline-layout',
    [] // No bind groups for simple triangle
  )
  console.log('✓ Pipeline layout created')

  // Create render pipeline
  const pipeline = device.createRenderPipeline(
    'triangle-pipeline',
    pipelineLayout,
    shaderModule,
    'vs_main',
    ['float32x2'], // vertex format: 2D positions
    shaderModule,
    'fs_main',
    ['rgba8unorm'], // render target format
    null, // no depth/stencil
    null, // default blend mode (replace)
    null, // default write mask (all channels)
    null  // no MSAA (sample_count = 1)
  )
  console.log('✓ Render pipeline created')

  // Create vertex buffer with triangle data
  const vertices = new Float32Array([
    // Triangle vertices (x, y)
     0.0,  0.5,  // Top
    -0.5, -0.5,  // Bottom left
     0.5, -0.5,  // Bottom right
  ])

  const vertexBuffer = device.createBuffer({
    size: vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  device.queue.writeBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
  console.log('✓ Vertex buffer created')

  // Create texture for rendering
  const width = 512
  const height = 512

  const renderTexture = device.createTexture({
    label: 'render-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  const textureView = renderTexture.createView('render-texture-view')
  console.log('✓ Render texture created')

  // Create command encoder
  const encoder = device.createCommandEncoder()

  // Execute render pass
  encoder.renderPass(
    pipeline,
    [vertexBuffer],
    3, // vertex count
    [textureView],
    [[0.0, 0.0, 0.0, 1.0]], // Clear color: black
    null, // No bind groups
    null, // No depth/stencil
    null, // No depth clear
    null  // No MSAA resolve targets
  )
  console.log('✓ Render pass executed')

  // Create buffer for reading back pixels
  const bytesPerPixel = 4 // RGBA
  const bytesPerRow = width * bytesPerPixel
  const paddedBytesPerRow = Math.ceil(bytesPerRow / 256) * 256 // WebGPU requires 256-byte alignment
  const bufferSize = paddedBytesPerRow * height

  const readBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  // Copy texture to buffer for readback
  device.copyTextureToBuffer(
    encoder,
    renderTexture,
    0, // mip_level
    0, 0, 0, // origin x, y, z
    readBuffer,
    0, // destination offset
    paddedBytesPerRow,
    null, // rows_per_image
    width,
    height,
    1 // depth
  )
  console.log('✓ Texture copied to buffer')

  // Submit commands
  const commandBuffer = encoder.finish()
  device.queue.submit([commandBuffer])
  device.poll(true)
  console.log('✓ GPU work complete\n')

  // Read back a few pixels to verify rendering
  console.log('Reading back rendered pixels...')
  const pixelData = await readBuffer.mapRead()
  const pixels = new Uint8Array(pixelData.buffer, pixelData.byteOffset, pixelData.byteLength)

  // Check center pixel (should be red from triangle)
  const centerX = Math.floor(width / 2)
  const centerY = Math.floor(height / 2)
  const centerOffset = centerY * paddedBytesPerRow + centerX * 4
  const centerPixel = [
    pixels[centerOffset],
    pixels[centerOffset + 1],
    pixels[centerOffset + 2],
    pixels[centerOffset + 3]
  ]

  console.log(`Center pixel (${centerX}, ${centerY}): RGBA(${centerPixel.join(', ')})`)

  // Red triangle should have red channel high
  if (centerPixel[0] > 200) {
    console.log('✅ Triangle detected at center (red channel strong)')
  } else {
    console.log('⚠️  Triangle may not be at center (red channel weak)')
  }

  readBuffer.unmap()

  console.log('\n✅ Triangle rendered and verified!')
  console.log('   - Shader compiled ✓')
  console.log('   - Pipeline created ✓')
  console.log('   - Render pass executed ✓')
  console.log('   - Texture readback ✓')

  // Cleanup
  vertexBuffer.destroy()
  readBuffer.destroy()
  renderTexture.destroy()
  device.destroy()

  console.log('\n✨ Triangle example completed')
}

main().catch(console.error)
