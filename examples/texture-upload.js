const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('../webgpu.js')

async function main() {
  console.log('🖼️  WebGPU Texture Upload Example\n')

  // Initialize GPU
  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Create a simple 4x4 texture with a pattern
  // Format: RGBA8 (4 bytes per pixel)
  const width = 4
  const height = 4
  const bytesPerPixel = 4

  // Create a checkerboard pattern: alternating red and blue
  const imageData = new Uint8Array(width * height * bytesPerPixel)
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const offset = (y * width + x) * bytesPerPixel
      const isRed = (x + y) % 2 === 0

      if (isRed) {
        imageData[offset + 0] = 255 // R
        imageData[offset + 1] = 0   // G
        imageData[offset + 2] = 0   // B
        imageData[offset + 3] = 255 // A
      } else {
        imageData[offset + 0] = 0   // R
        imageData[offset + 1] = 0   // G
        imageData[offset + 2] = 255 // B
        imageData[offset + 3] = 255 // A
      }
    }
  }

  console.log('Created 4x4 checkerboard pattern:')
  console.log('  Red and Blue squares')

  // Create upload buffer with aligned bytes per row
  const bytesPerRow = width * bytesPerPixel
  const alignedBytesPerRow = Math.ceil(bytesPerRow / 256) * 256 // 256-byte alignment
  const bufferSize = alignedBytesPerRow * height

  // Create aligned buffer and copy image data
  const alignedData = new Uint8Array(bufferSize)
  for (let y = 0; y < height; y++) {
    const srcOffset = y * bytesPerRow
    const dstOffset = y * alignedBytesPerRow
    alignedData.set(imageData.subarray(srcOffset, srcOffset + bytesPerRow), dstOffset)
  }

  const uploadBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  // Write image data to buffer
  device.queue.writeBuffer(uploadBuffer, 0, Buffer.from(alignedData.buffer))
  console.log('✓ Image data written to upload buffer')

  // Create texture
  const texture = device.createTexture({
    label: 'checkerboard-texture',
    width,
    height,
    depth: 1,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1,
  })
  console.log('✓ Texture created')

  // Create command encoder
  const encoder = device.createCommandEncoder()

  // Copy buffer to texture
  device.copyBufferToTexture(
    encoder,
    uploadBuffer,
    0, // source offset
    alignedBytesPerRow,
    null, // rows_per_image
    texture,
    0, // mip_level
    0, 0, 0, // origin x, y, z
    width,
    height,
    1 // depth
  )
  console.log('✓ Buffer copied to texture')

  // Verify by copying back to a read buffer
  const readBuffer = device.createBuffer({
    size: bufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  device.copyTextureToBuffer(
    encoder,
    texture,
    0, // mip_level
    0, 0, 0, // origin x, y, z
    readBuffer,
    0, // destination offset
    alignedBytesPerRow,
    null, // rows_per_image
    width,
    height,
    1 // depth
  )
  console.log('✓ Texture copied back to buffer for verification')

  // Submit commands
  const commandBuffer = encoder.finish()
  device.queue.submit([commandBuffer])
  device.poll(true)
  console.log('✓ GPU work complete\n')

  // Read back and verify
  console.log('Verifying uploaded texture data...')
  const readData = await readBuffer.mapRead()
  const readPixels = new Uint8Array(readData.buffer, readData.byteOffset, readData.byteLength)

  let allCorrect = true
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const srcOffset = (y * width + x) * bytesPerPixel
      const dstOffset = y * alignedBytesPerRow + x * bytesPerPixel

      for (let c = 0; c < bytesPerPixel; c++) {
        if (readPixels[dstOffset + c] !== imageData[srcOffset + c]) {
          console.log(`❌ Mismatch at (${x},${y}) channel ${c}: expected ${imageData[srcOffset + c]}, got ${readPixels[dstOffset + c]}`)
          allCorrect = false
          break
        }
      }
      if (!allCorrect) break
    }
    if (!allCorrect) break
  }

  if (allCorrect) {
    console.log('✅ All pixels match! Texture upload verified!')

    // Show a few sample pixels
    console.log('\nSample pixels:')
    const getPixel = (x, y) => {
      const offset = y * alignedBytesPerRow + x * bytesPerPixel
      return `RGBA(${readPixels[offset]}, ${readPixels[offset+1]}, ${readPixels[offset+2]}, ${readPixels[offset+3]})`
    }
    console.log(`  (0,0): ${getPixel(0, 0)} - Red`)
    console.log(`  (1,0): ${getPixel(1, 0)} - Blue`)
    console.log(`  (0,1): ${getPixel(0, 1)} - Blue`)
    console.log(`  (1,1): ${getPixel(1, 1)} - Red`)
  }

  readBuffer.unmap()

  console.log('\n✅ Texture upload example completed!')
  console.log('   - Created image data ✓')
  console.log('   - Uploaded to GPU texture ✓')
  console.log('   - Verified round-trip ✓')

  // Cleanup
  uploadBuffer.destroy()
  readBuffer.destroy()
  texture.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
