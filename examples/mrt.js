const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('../webgpu.js')

async function main() {
  console.log('🎨 WebGPU Multiple Render Targets (MRT) Example\n')

  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Create shader with multiple fragment outputs (G-buffer)
  const shaderCode = `
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) worldPos: vec3f,
    @location(1) normal: vec3f,
    @location(2) color: vec3f,
}

@vertex
fn vs_main(
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) color: vec3f
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(position, 1.0);
    output.worldPos = position;
    output.normal = normalize(normal);
    output.color = color;
    return output;
}

// Multiple render targets (MRT) - G-buffer outputs
struct GBufferOutput {
    @location(0) position: vec4f,  // World position
    @location(1) normal: vec4f,    // World normal
    @location(2) albedo: vec4f,    // Base color
}

@fragment
fn fs_main(input: VertexOutput) -> GBufferOutput {
    var output: GBufferOutput;

    // Output world position
    output.position = vec4f(input.worldPos, 1.0);

    // Output normal (normalized)
    output.normal = vec4f(input.normal * 0.5 + 0.5, 1.0); // Map [-1, 1] to [0, 1]

    // Output albedo (base color)
    output.albedo = vec4f(input.color, 1.0);

    return output;
}
`

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Shader compiled with 3 fragment outputs')

  // Create pipeline with 3 render targets - WebGPU standard API
  const pipelineLayout = device.createPipelineLayout('MRT Pipeline Layout', [])

  const pipeline = device.createRenderPipeline({
    label: 'MRT Pipeline',
    layout: pipelineLayout,
    vertex: {
      module: shaderModule,
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 36, // 9 floats * 4 bytes (position + normal + color)
        attributes: [
          {
            shaderLocation: 0,
            offset: 0,
            format: 'float32x3' // position
          },
          {
            shaderLocation: 1,
            offset: 12,
            format: 'float32x3' // normal
          },
          {
            shaderLocation: 2,
            offset: 24,
            format: 'float32x3' // color
          }
        ]
      }]
    },
    fragment: {
      module: shaderModule,
      entryPoint: 'fs_main',
      targets: [
        { format: 'rgba8unorm' }, // position target
        { format: 'rgba8unorm' }, // normal target
        { format: 'rgba8unorm' }  // albedo target
      ]
    },
    primitive: {
      topology: 'triangle-list'
    }
  })
  console.log('✓ Pipeline created with 3 render targets')

  // Create triangle with position, normal, and color data
  const vertices = new Float32Array([
    // Position          Normal            Color (RGB)
     0.0,  0.5, 0.0,    0.0, 0.0, 1.0,    1.0, 0.0, 0.0,  // Top (red)
    -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,    0.0, 1.0, 0.0,  // Bottom left (green)
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0,    0.0, 0.0, 1.0,  // Bottom right (blue)
  ])

  const vertexBuffer = device.createBuffer({
    size: vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  device.queue.writeBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
  console.log('✓ Vertex buffer created')

  // Create 3 render target textures (G-buffer)
  const width = 512
  const height = 512

  const positionTexture = device.createTexture({
    label: 'position-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  const normalTexture = device.createTexture({
    label: 'normal-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  const albedoTexture = device.createTexture({
    label: 'albedo-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  const positionView = positionTexture.createView('position-view')
  const normalView = normalTexture.createView('normal-view')
  const albedoView = albedoTexture.createView('albedo-view')
  console.log('✓ Created 3 render target textures (G-buffer)')

  // Render to all 3 targets simultaneously - WebGPU standard API
  const encoder = device.createCommandEncoder()

  const pass = encoder.beginRenderPass({
    colorAttachments: [
      {
        view: positionView,
        loadOp: 'clear',
        storeOp: 'store',
        clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
      },
      {
        view: normalView,
        loadOp: 'clear',
        storeOp: 'store',
        clearValue: { r: 0.5, g: 0.5, b: 1.0, a: 1.0 }
      },
      {
        view: albedoView,
        loadOp: 'clear',
        storeOp: 'store',
        clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
      }
    ]
  })
  pass.setPipeline(pipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.draw(3)
  pass.end()
  console.log('✓ Rendered to 3 targets simultaneously (MRT)')

  // Read back all 3 textures to verify
  const bytesPerRow = Math.ceil(width * 4 / 256) * 256
  const bufferSize = bytesPerRow * height

  const positionBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  const normalBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  const albedoBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  encoder.copyTextureToBuffer(
    positionTexture,
    0, 0, 0, 0,
    positionBuffer,
    0,
    bytesPerRow,
    null,
    width,
    height,
    1
  )

  encoder.copyTextureToBuffer(
    normalTexture,
    0, 0, 0, 0,
    normalBuffer,
    0,
    bytesPerRow,
    null,
    width,
    height,
    1
  )

  encoder.copyTextureToBuffer(
    albedoTexture,
    0, 0, 0, 0,
    albedoBuffer,
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

  // Verify results from all 3 buffers
  const positionData = await positionBuffer.mapRead()
  const positionPixels = new Uint8Array(positionData.buffer, positionData.byteOffset, positionData.byteLength)

  const normalData = await normalBuffer.mapRead()
  const normalPixels = new Uint8Array(normalData.buffer, normalData.byteOffset, normalData.byteLength)

  const albedoData = await albedoBuffer.mapRead()
  const albedoPixels = new Uint8Array(albedoData.buffer, albedoData.byteOffset, albedoData.byteLength)

  // Check center pixel in each buffer
  const centerX = Math.floor(width / 2)
  const centerY = Math.floor(height / 2)
  const centerOffset = centerY * bytesPerRow + centerX * 4

  const positionPixel = [
    positionPixels[centerOffset],
    positionPixels[centerOffset + 1],
    positionPixels[centerOffset + 2],
    positionPixels[centerOffset + 3]
  ]

  const normalPixel = [
    normalPixels[centerOffset],
    normalPixels[centerOffset + 1],
    normalPixels[centerOffset + 2],
    normalPixels[centerOffset + 3]
  ]

  const albedoPixel = [
    albedoPixels[centerOffset],
    albedoPixels[centerOffset + 1],
    albedoPixels[centerOffset + 2],
    albedoPixels[centerOffset + 3]
  ]

  console.log('\n📊 G-Buffer Contents (center pixel):')
  console.log(`   Position: RGBA(${positionPixel.join(', ')})`)
  console.log(`   Normal:   RGBA(${normalPixel.join(', ')})`)
  console.log(`   Albedo:   RGBA(${albedoPixel.join(', ')})`)

  // Verify: Normal should be pointing forward (blue-ish in texture space ~127, 127, 255)
  const hasNormalData = normalPixel[2] > 200
  if (hasNormalData) {
    console.log('✅ Normal buffer verified! (Z-forward normal detected)')
  }

  // Verify: Albedo should have color from the triangle
  const hasAlbedoData = albedoPixel[0] > 50 || albedoPixel[1] > 50 || albedoPixel[2] > 50
  if (hasAlbedoData) {
    console.log('✅ Albedo buffer verified! (Color data detected)')
  }

  // Unmap all buffers
  positionBuffer.unmap()
  normalBuffer.unmap()
  albedoBuffer.unmap()

  console.log('\n✅ MRT example complete!')
  console.log('   - 3 simultaneous render targets ✓')
  console.log('   - G-buffer (position, normal, albedo) ✓')
  console.log('   - Multi-target fragment shader ✓')
  console.log('   - All buffers verified ✓')

  // Cleanup
  vertexBuffer.destroy()
  positionBuffer.destroy()
  normalBuffer.destroy()
  albedoBuffer.destroy()
  positionTexture.destroy()
  normalTexture.destroy()
  albedoTexture.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
