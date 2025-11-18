import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, bufferUsage, textureUsage } from '../index.js'

describe('GPU Texture', () => {
  let device: any
  const bufUsage = bufferUsage()
  const texUsage = textureUsage()

  beforeAll(async () => {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should get texture usage constants', () => {
    expect(texUsage).toBeDefined()
    expect(typeof texUsage.copySrc).toBe('number')
    expect(typeof texUsage.copyDst).toBe('number')
    expect(typeof texUsage.textureBinding).toBe('number')
    expect(typeof texUsage.storageBinding).toBe('number')
    expect(typeof texUsage.renderAttachment).toBe('number')
  })

  test('should create texture', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: texUsage.renderAttachment | texUsage.copySrc,
    })

    expect(texture).toBeDefined()
    expect(texture).toHaveProperty('width')
    expect(texture).toHaveProperty('height')
    expect(texture).toHaveProperty('createView')
    expect(texture).toHaveProperty('destroy')

    expect(texture.width()).toBe(256)
    expect(texture.height()).toBe(256)

    texture.destroy()
  })

  test('should create texture with different formats', () => {
    const formats = ['rgba8unorm', 'bgra8unorm', 'rgba16float', 'rgba32float']

    formats.forEach(format => {
      const texture = device.createTexture({
        width: 128,
        height: 128,
        format,
        usage: texUsage.renderAttachment,
      })

      expect(texture).toBeDefined()
      expect(texture.width()).toBe(128)
      expect(texture.height()).toBe(128)

      texture.destroy()
    })
  })

  test('should create texture view', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'rgba8unorm',
      usage: texUsage.renderAttachment,
    })

    const view = texture.createView('Test View')
    expect(view).toBeDefined()

    texture.destroy()
  })

  test('should create depth texture', () => {
    const texture = device.createTexture({
      width: 256,
      height: 256,
      format: 'depth24plus',
      usage: texUsage.renderAttachment,
    })

    expect(texture).toBeDefined()
    expect(texture.width()).toBe(256)
    expect(texture.height()).toBe(256)

    texture.destroy()
  })

  test('should upload texture data', async () => {
    const width = 64 // Must be compatible with alignment
    const height = 64
    const texture = device.createTexture({
      width,
      height,
      format: 'rgba8unorm',
      usage: texUsage.copyDst | texUsage.copySrc,
    })

    // Create checkerboard pattern
    const data = new Uint8Array(width * height * 4)
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const offset = (y * width + x) * 4
        const isWhite = (x + y) % 2 === 0
        data[offset] = isWhite ? 255 : 0     // R
        data[offset + 1] = isWhite ? 255 : 0 // G
        data[offset + 2] = isWhite ? 255 : 0 // B
        data[offset + 3] = 255               // A
      }
    }

    // Upload
    const uploadBuffer = device.createBuffer(
      data.byteLength,
      bufUsage.copySrc | bufUsage.copyDst,
      false
    )
    device.queueWriteBuffer(uploadBuffer, 0, Buffer.from(data.buffer))

    const encoder = device.createCommandEncoder()
    device.copyBufferToTexture(
      encoder,
      uploadBuffer,
      0, // offset
      width * 4, // bytes per row
      height, // rows per image
      texture,
      0, // mip level
      0, 0, 0, // origin
      width,
      height,
      1 // depth
    )
    device.queueSubmit(encoder.finish())
    device.poll(true)

    uploadBuffer.destroy()
    texture.destroy()
  })

  test('should read texture data', async () => {
    const width = 64 // Must be compatible with alignment
    const height = 64
    const texture = device.createTexture({
      width,
      height,
      format: 'rgba8unorm',
      usage: texUsage.copyDst | texUsage.copySrc,
    })

    // Create solid red texture
    const data = new Uint8Array(width * height * 4)
    for (let i = 0; i < width * height; i++) {
      data[i * 4] = 255     // R
      data[i * 4 + 1] = 0   // G
      data[i * 4 + 2] = 0   // B
      data[i * 4 + 3] = 255 // A
    }

    const uploadBuffer = device.createBuffer(data.byteLength, bufUsage.copySrc | bufUsage.copyDst, false)
    device.queueWriteBuffer(uploadBuffer, 0, Buffer.from(data.buffer))

    const encoder1 = device.createCommandEncoder()
    device.copyBufferToTexture(encoder1, uploadBuffer, 0, width * 4, height, texture, 0, 0, 0, 0, width, height, 1)
    device.queueSubmit(encoder1.finish())
    device.poll(true)

    // Read back
    const readBuffer = device.createBuffer(data.byteLength, bufUsage.copyDst | bufUsage.mapRead, false)
    const encoder2 = device.createCommandEncoder()
    device.copyTextureToBuffer(encoder2, texture, 0, 0, 0, 0, readBuffer, 0, width * 4, height, width, height, 1)
    device.queueSubmit(encoder2.finish())
    device.poll(true)

    const resultData = await readBuffer.mapRead()
    const results = new Uint8Array(resultData.buffer, resultData.byteOffset, data.byteLength)

    // Verify all pixels are red
    for (let i = 0; i < width * height; i++) {
      expect(results[i * 4]).toBe(255)     // R
      expect(results[i * 4 + 1]).toBe(0)   // G
      expect(results[i * 4 + 2]).toBe(0)   // B
      expect(results[i * 4 + 3]).toBe(255) // A
    }

    readBuffer.unmap()
    uploadBuffer.destroy()
    readBuffer.destroy()
    texture.destroy()
  })

  test('should create sampler', () => {
    const sampler = device.createSampler({
      magFilter: 'linear',
      minFilter: 'linear',
      addressModeU: 'repeat',
      addressModeV: 'repeat',
    })

    expect(sampler).toBeDefined()
  })

  test('should create sampler with different modes', () => {
    const configs = [
      { magFilter: 'nearest', minFilter: 'nearest' },
      { magFilter: 'linear', minFilter: 'linear' },
      { addressModeU: 'clamp-to-edge', addressModeV: 'clamp-to-edge' },
      { addressModeU: 'repeat', addressModeV: 'repeat' },
      { addressModeU: 'mirror-repeat', addressModeV: 'mirror-repeat' },
    ]

    configs.forEach(config => {
      const sampler = device.createSampler(config)
      expect(sampler).toBeDefined()
    })
  })
})
