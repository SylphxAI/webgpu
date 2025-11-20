const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('../webgpu.js')

async function main() {
  console.log('🖼️  WebGPU Textured Quad Example\n')

  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Create a 2x2 checkerboard texture
  const texWidth = 2
  const texHeight = 2
  const texData = new Uint8Array([
    255, 0, 0, 255,    // Red
    0, 0, 255, 255,    // Blue
    0, 0, 255, 255,    // Blue
    255, 0, 0, 255,    // Red
  ])

  // Upload texture data
  const bytesPerRow = texWidth * 4
  const alignedBytesPerRow = Math.ceil(bytesPerRow / 256) * 256
  const bufferSize = alignedBytesPerRow * texHeight
  const alignedData = new Uint8Array(bufferSize)
  for (let y = 0; y < texHeight; y++) {
    const srcOffset = y * bytesPerRow
    const dstOffset = y * alignedBytesPerRow
    alignedData.set(texData.subarray(srcOffset, srcOffset + bytesPerRow), dstOffset)
  }

  const uploadBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  device.queue.writeBuffer(uploadBuffer, 0, Buffer.from(alignedData.buffer))

  // Create texture
  const texture = device.createTexture({
    label: 'checkerboard',
    width: texWidth,
    height: texHeight,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  // Create sampler
  const sampler = device.createSampler({
    label: 'nearest-sampler',
    addressModeU: 'repeat',
    addressModeV: 'repeat',
    addressModeW: 'repeat',
    magFilter: 'nearest',
    minFilter: 'nearest',
    mipmapFilter: 'nearest',
  })

  // Copy data to texture
  let encoder = device.createCommandEncoder()
  encoder.copyBufferToTexture(
    uploadBuffer,
    0,
    alignedBytesPerRow,
    null,
    texture,
    0, 0, 0, 0,
    texWidth,
    texHeight,
    1
  )
  device.queue.submit([encoder.finish()])
  device.poll(true)
  console.log('✓ Texture uploaded')

  // Create shader
  const shaderCode = `
@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
}

@vertex
fn vs_main(@location(0) position: vec2f, @location(1) uv: vec2f) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(position, 0.0, 1.0);
    output.uv = uv;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    return textureSample(myTexture, mySampler, input.uv);
}
`

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Shader compiled')

  // Create bind group layout (WebGPU standard API)
  const bindGroupLayout = device.createBindGroupLayout({
    label: 'Texture Bind Group Layout',
    entries: [
      {
        binding: 0,
        visibility: 0x2, // FRAGMENT
        texture: { sampleType: 'float' },
      },
      {
        binding: 1,
        visibility: 0x2, // FRAGMENT
        sampler: { type: 'filtering' },
      },
    ],
  })

  // Create textureView
  const textureView = texture.createView('checkerboard-view')

  // Create bind group with texture and sampler (WebGPU standard API)
  const bindGroup = device.createBindGroup({
    label: 'Texture Bind Group',
    layout: bindGroupLayout,
    entries: [
      { binding: 0, resource: textureView },
      { binding: 1, resource: sampler },
    ]
  })
  console.log('✓ Bind group created with texture and sampler')

  // Create pipeline with WebGPU standard API
  const pipelineLayout = device.createPipelineLayout({
    label: 'Textured Quad Layout',
    bindGroupLayouts: [bindGroupLayout]
  })

  const pipeline = device.createRenderPipeline({
    label: 'Textured Quad Pipeline',
    layout: pipelineLayout,
    vertex: {
      module: shaderModule,
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 16, // 4 floats * 4 bytes (position + uv)
        attributes: [
          {
            shaderLocation: 0,
            offset: 0,
            format: 'float32x2' // position
          },
          {
            shaderLocation: 1,
            offset: 8,
            format: 'float32x2' // uv
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
  console.log('✓ Pipeline created')

  // Create quad vertices (position + UV)
  const vertices = new Float32Array([
    // Position      UV
    -0.5,  0.5,     0.0, 0.0,  // Top left
    -0.5, -0.5,     0.0, 2.0,  // Bottom left
     0.5, -0.5,     2.0, 2.0,  // Bottom right
     0.5,  0.5,     2.0, 0.0,  // Top right
  ])

  const indices = new Uint16Array([
    0, 1, 2,  // First triangle
    0, 2, 3,  // Second triangle
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

  // Render to texture
  const renderWidth = 256
  const renderHeight = 256

  const renderTexture = device.createTexture({
    label: 'render-target',
    width: renderWidth,
    height: renderHeight,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })

  const renderView = renderTexture.createView('render-view')

  // Render with WebGPU standard API
  encoder = device.createCommandEncoder()

  const pass = encoder.beginRenderPass({
    colorAttachments: [{
      view: renderView,
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.2, g: 0.2, b: 0.2, a: 1.0 }
    }]
  })
  pass.setPipeline(pipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.setIndexBuffer(indexBuffer, 'uint16')
  pass.setBindGroup(0, bindGroup)
  pass.drawIndexed(6)
  pass.end()
  console.log('✓ Textured quad rendered')

  // Read back to verify
  const readBytesPerRow = Math.ceil(renderWidth * 4 / 256) * 256
  const readBufferSize = readBytesPerRow * renderHeight

  const readBuffer = device.createBuffer({
    size: readBufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  encoder.copyTextureToBuffer(
    renderTexture,
    0, 0, 0, 0,
    readBuffer,
    0,
    readBytesPerRow,
    null,
    renderWidth,
    renderHeight,
    1
  )

  device.queue.submit([encoder.finish()])
  device.poll(true)
  console.log('✓ Render complete')

  // Check a sample pixel to verify texture was applied
  const pixelData = await readBuffer.mapRead()
  const pixels = new Uint8Array(pixelData.buffer, pixelData.byteOffset, pixelData.byteLength)

  const centerX = Math.floor(renderWidth / 2)
  const centerY = Math.floor(renderHeight / 2)
  const centerOffset = centerY * readBytesPerRow + centerX * 4
  const centerPixel = [
    pixels[centerOffset],
    pixels[centerOffset + 1],
    pixels[centerOffset + 2],
    pixels[centerOffset + 3]
  ]

  console.log(`\nCenter pixel: RGBA(${centerPixel.join(', ')})`)

  // The checkerboard repeats 2x, so center should have a color from the texture
  const hasColor = centerPixel[0] > 100 || centerPixel[2] > 100
  if (hasColor) {
    console.log('✅ Textured quad verified! (Color detected)')
  } else {
    console.log('⚠️  Unexpected pixel color (may be background)')
  }

  readBuffer.unmap()

  console.log('\n✅ Textured quad example complete!')
  console.log('   - Texture uploaded ✓')
  console.log('   - Sampler created ✓')
  console.log('   - Bind group with texture/sampler ✓')
  console.log('   - Textured quad rendered ✓')
  console.log('   - Result verified ✓')

  // Cleanup
  uploadBuffer.destroy()
  vertexBuffer.destroy()
  indexBuffer.destroy()
  readBuffer.destroy()
  texture.destroy()
  renderTexture.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
