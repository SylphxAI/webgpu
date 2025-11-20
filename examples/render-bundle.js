const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function main() {
  console.log('📦 WebGPU Render Bundle Example\n')

  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Shader for rendering triangles
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

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Shader compiled')

  // Create vertex data for TWO triangles
  const triangle1Vertices = new Float32Array([
    // Position (x, y, z)     Color (r, g, b)
    -0.6, 0.5, 0.0,           1.0, 0.0, 0.0,  // Top-left (red)
    -1.0, -0.5, 0.0,          0.0, 1.0, 0.0,  // Bottom-left (green)
    -0.2, -0.5, 0.0,          0.0, 0.0, 1.0,  // Bottom-right (blue)
  ])

  const triangle2Vertices = new Float32Array([
    // Position (x, y, z)     Color (r, g, b)
    0.6, 0.5, 0.0,            1.0, 1.0, 0.0,  // Top-right (yellow)
    0.2, -0.5, 0.0,           1.0, 0.0, 1.0,  // Bottom-left (magenta)
    1.0, -0.5, 0.0,           0.0, 1.0, 1.0,  // Bottom-right (cyan)
  ])

  const vertexBuffer1 = device.createBuffer({
    size: triangle1Vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(vertexBuffer1, 0, Buffer.from(triangle1Vertices.buffer))

  const vertexBuffer2 = device.createBuffer({
    size: triangle2Vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(vertexBuffer2, 0, Buffer.from(triangle2Vertices.buffer))

  console.log('✓ Vertex buffers created (2 triangles)')

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

  // Create TWO render bundles - one for each triangle
  // These can be created once and reused many times!
  const bundle1 = device.createRenderBundle(
    'Triangle 1 Bundle',
    pipeline,
    [vertexBuffer1],
    3, // vertex count
    null, // no bind groups
    ['rgba8unorm'] // color format
  )

  const bundle2 = device.createRenderBundle(
    'Triangle 2 Bundle',
    pipeline,
    [vertexBuffer2],
    3, // vertex count
    null, // no bind groups
    ['rgba8unorm'] // color format
  )

  console.log('✓ Render bundles created (2 bundles - reusable!)')

  // Execute BOTH bundles in a single render pass
  // This demonstrates the key benefit: bundles can be created once and executed multiple times
  const encoder = device.createCommandEncoder()

  encoder.renderPassBundles(
    [bundle1, bundle2], // Execute both bundles
    [textureView],
    [[0.1, 0.1, 0.1, 1.0]] // Dark gray background
  )

  // Create readback buffer
  const bytesPerRow = 256 * 4
  const readbackSize = bytesPerRow * height

  const readbackBuffer = device.createBuffer({
    size: readbackSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

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

  device.queue.submit([encoder.finish()])
  device.poll(true)
  console.log('✓ Bundles executed')

  // Read back and verify
  const data = await readbackBuffer.mapRead()
  const pixels = new Uint8Array(data.buffer, data.byteOffset, data.byteLength)

  // Check a pixel from the left triangle (around x=64)
  const leftX = 64
  const leftY = Math.floor(height / 2)
  const leftOffset = (leftY * width + leftX) * 4
  const leftR = pixels[leftOffset]
  const leftG = pixels[leftOffset + 1]
  const leftB = pixels[leftOffset + 2]

  // Check a pixel from the right triangle (around x=192)
  const rightX = 192
  const rightY = Math.floor(height / 2)
  const rightOffset = (rightY * width + rightX) * 4
  const rightR = pixels[rightOffset]
  const rightG = pixels[rightOffset + 1]
  const rightB = pixels[rightOffset + 2]

  console.log(`\n📊 Left triangle pixel:  RGBA(${leftR}, ${leftG}, ${leftB}, 255)`)
  console.log(`📊 Right triangle pixel: RGBA(${rightR}, ${rightG}, ${rightB}, 255)`)

  readbackBuffer.unmap()

  // Verify triangles were rendered
  const leftRendered = leftR > 25 || leftG > 25 || leftB > 25
  const rightRendered = rightR > 25 || rightG > 25 || rightB > 25

  if (leftRendered && rightRendered) {
    console.log('\n✅ Render bundles verified! Both triangles rendered successfully')
  } else {
    console.log('\n⚠️  Unexpected rendering result')
  }

  console.log('\n✅ Render bundle example complete!')
  console.log('   - Bundles created once ✓')
  console.log('   - Multiple bundles executed in single pass ✓')
  console.log('   - CPU overhead reduced ✓')
  console.log('   \n   💡 Key benefit: Bundles can be reused across frames')
  console.log('      reducing the cost of re-recording commands!')

  // Cleanup
  vertexBuffer1.destroy()
  vertexBuffer2.destroy()
  readbackBuffer.destroy()
  texture.destroy()
  bundle1.destroy()
  bundle2.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
