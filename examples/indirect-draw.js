const { Gpu, bufferUsage: getBufferUsage } = require('../index.js')

async function main() {
  console.log('🎮 WebGPU Indirect Draw Example\n')

  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  const bufferUsage = getBufferUsage()

  // Shader for rendering a simple triangle
  const shaderCode = `
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@vertex
fn vs_main(@location(0) position: vec3f, @location(1) color: vec3f) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(position, 1.0);
    output.color = vec4f(color, 1.0);
    return output;
}

@fragment
fn fs_main(@location(0) color: vec4f) -> @location(0) vec4f {
    return color;
}
`

  const shaderModule = device.createShaderModule(shaderCode)
  console.log('✓ Shader compiled')

  // Vertex data: positions and colors for a triangle
  const vertices = new Float32Array([
    // Position (x, y, z)     Color (r, g, b)
    0.0, 0.5, 0.0,            1.0, 0.0, 0.0,  // Top (red)
    -0.5, -0.5, 0.0,          0.0, 1.0, 0.0,  // Bottom-left (green)
    0.5, -0.5, 0.0,           0.0, 0.0, 1.0,  // Bottom-right (blue)
  ])

  const vertexBuffer = device.createBuffer(
    vertices.byteLength,
    bufferUsage.vertex | bufferUsage.copyDst,
    false
  )
  device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
  console.log('✓ Vertex buffer created')

  // Create indirect draw buffer
  // Format: [vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32]
  const indirectParams = new Uint32Array([
    3,  // vertex_count (draw 3 vertices)
    1,  // instance_count (1 instance)
    0,  // first_vertex (start at vertex 0)
    0,  // first_instance (start at instance 0)
  ])

  const indirectBuffer = device.createBuffer(
    indirectParams.byteLength,
    bufferUsage.indirect | bufferUsage.copyDst,
    false
  )
  device.queueWriteBuffer(indirectBuffer, 0, Buffer.from(indirectParams.buffer))
  console.log('✓ Indirect draw buffer created')

  // Create render texture (256x256 RGBA)
  const width = 256
  const height = 256

  const texture = device.createTexture({
    width,
    height,
    format: 'rgba8unorm',
    usage: 0x11, // RENDER_ATTACHMENT (0x10) | COPY_SRC (0x01)
  })
  const textureView = texture.createView('Render Target')
  console.log('✓ Render texture created')

  // Create pipeline
  const pipelineLayout = device.createPipelineLayout('Pipeline Layout', [])
  const pipeline = device.createRenderPipeline(
    'Triangle Pipeline',
    pipelineLayout,
    shaderModule,
    'vs_main',
    ['float32x3', 'float32x3'], // position + color attributes
    shaderModule,
    'fs_main',
    ['rgba8unorm'],
    null,  // no depth
    null,  // default blend
    null,  // all channels
    null   // no MSAA
  )
  console.log('✓ Render pipeline created')

  // Create encoder and execute indirect render pass
  const encoder = device.createCommandEncoder()

  encoder.renderPassIndirect(
    pipeline,
    [vertexBuffer],
    indirectBuffer,
    0, // indirect offset
    [textureView],
    [[0.1, 0.1, 0.1, 1.0]], // Dark gray background
    null,  // no bind groups
    null,  // no depth
    null,  // no clear depth
    null   // no resolve targets
  )

  // Create readback buffer
  const bytesPerRow = 256 * 4 // 4 bytes per pixel (RGBA)
  const readbackSize = bytesPerRow * height

  const readbackBuffer = device.createBuffer(
    readbackSize,
    bufferUsage.copyDst | bufferUsage.mapRead,
    false
  )

  // Copy texture to buffer
  device.copyTextureToBuffer(
    encoder,
    texture,
    0, // mip level
    0, 0, 0, // origin
    readbackBuffer,
    0, // offset
    bytesPerRow,
    height,
    width,
    height,
    1 // depth
  )

  device.queueSubmit(encoder.finish())
  device.poll(true)
  console.log('✓ Indirect draw executed')

  // Read back and verify
  const data = await readbackBuffer.mapRead()
  const pixels = new Uint8Array(data.buffer, data.byteOffset, data.byteLength)

  // Check center pixel (should be colored by the triangle)
  const centerX = Math.floor(width / 2)
  const centerY = Math.floor(height / 2)
  const centerOffset = (centerY * width + centerX) * 4

  const r = pixels[centerOffset]
  const g = pixels[centerOffset + 1]
  const b = pixels[centerOffset + 2]
  const a = pixels[centerOffset + 3]

  console.log(`\n📊 Center pixel: RGBA(${r}, ${g}, ${b}, ${a})`)

  readbackBuffer.unmap()

  // Verify the triangle was rendered (center should have color, not background)
  if ((r > 25 || g > 25 || b > 25) && !(r === 25 && g === 25 && b === 25)) {
    console.log('✅ Indirect draw verified! Triangle rendered successfully')
  } else {
    console.log('⚠️  Unexpected pixel color (background color detected)')
  }

  console.log('\n✅ Indirect draw example complete!')
  console.log('   - Indirect buffer created ✓')
  console.log('   - Draw parameters from GPU buffer ✓')
  console.log('   - GPU-driven rendering ✓')

  // Cleanup
  vertexBuffer.destroy()
  indirectBuffer.destroy()
  readbackBuffer.destroy()
  texture.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
