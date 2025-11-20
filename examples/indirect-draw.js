const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function main() {
  console.log('🎮 WebGPU Indirect Draw Example\n')

  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

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

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Shader compiled')

  // Vertex data: positions and colors for a triangle
  const vertices = new Float32Array([
    // Position (x, y, z)     Color (r, g, b)
    0.0, 0.5, 0.0,            1.0, 0.0, 0.0,  // Top (red)
    -0.5, -0.5, 0.0,          0.0, 1.0, 0.0,  // Bottom-left (green)
    0.5, -0.5, 0.0,           0.0, 0.0, 1.0,  // Bottom-right (blue)
  ])

  const vertexBuffer = device.createBuffer({
    size: vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
  console.log('✓ Vertex buffer created')

  // Create indirect draw buffer
  // Format: [vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32]
  const indirectParams = new Uint32Array([
    3,  // vertex_count (draw 3 vertices)
    1,  // instance_count (1 instance)
    0,  // first_vertex (start at vertex 0)
    0,  // first_instance (start at instance 0)
  ])

  const indirectBuffer = device.createBuffer({
    size: indirectParams.byteLength,
    usage: GPUBufferUsage.INDIRECT | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(indirectBuffer, 0, Buffer.from(indirectParams.buffer))
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

  // Create pipeline - WebGPU standard API
  const pipelineLayout = device.createPipelineLayout('Pipeline Layout', [])
  const pipeline = device.createRenderPipeline({
    label: 'Triangle Pipeline',
    layout: pipelineLayout,
    vertex: {
      module: shaderModule,
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 24, // 6 floats * 4 bytes (position + color)
        attributes: [
          {
            shaderLocation: 0,
            offset: 0,
            format: 'float32x3' // position
          },
          {
            shaderLocation: 1,
            offset: 12,
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
    }
  })
  console.log('✓ Render pipeline created')

  // Create encoder and execute indirect render pass - WebGPU standard API
  const encoder = device.createCommandEncoder()

  const pass = encoder.beginRenderPass({
    colorAttachments: [{
      view: textureView,
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }
    }]
  })
  pass.setPipeline(pipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.drawIndirect(indirectBuffer, 0)
  pass.end()

  // Create readback buffer
  const bytesPerRow = 256 * 4 // 4 bytes per pixel (RGBA)
  const readbackSize = bytesPerRow * height

  const readbackBuffer = device.createBuffer({
    size: readbackSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  // Copy texture to buffer
  encoder.copyTextureToBuffer(
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
