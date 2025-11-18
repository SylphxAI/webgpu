import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, bufferUsage } from '../index.js'

describe('GPU Buffer', () => {
  let device: Awaited<ReturnType<Awaited<ReturnType<ReturnType<typeof Gpu.create>['requestAdapter']>>['requestDevice']>>
  const usage = bufferUsage()

  beforeAll(async () => {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should get buffer usage constants', () => {
    expect(usage).toBeDefined()
    expect(typeof usage.copySrc).toBe('number')
    expect(typeof usage.copyDst).toBe('number')
    expect(typeof usage.storage).toBe('number')
    expect(typeof usage.uniform).toBe('number')
    expect(typeof usage.vertex).toBe('number')
    expect(typeof usage.index).toBe('number')
    expect(typeof usage.mapRead).toBe('number')
    expect(typeof usage.mapWrite).toBe('number')
    expect(typeof usage.indirect).toBe('number')
    expect(typeof usage.queryResolve).toBe('number')
  })

  test('should create buffer', () => {
    const size = 256
    const buffer = device.createBuffer(
      size,
      usage.copyDst | usage.mapRead,
      false
    )

    expect(buffer).toBeDefined()
    expect(buffer).toHaveProperty('size')
    expect(buffer).toHaveProperty('usage')
    expect(buffer).toHaveProperty('mapRead')
    expect(buffer).toHaveProperty('unmap')
    expect(buffer).toHaveProperty('destroy')

    expect(buffer.size()).toBe(size)

    buffer.destroy()
  })

  test('should create mapped buffer', () => {
    const buffer = device.createBuffer(
      256,
      usage.copyDst,
      true // mapped at creation
    )

    expect(buffer).toBeDefined()
    expect(buffer.size()).toBe(256)

    buffer.destroy()
  })

  test('should write and read buffer data', async () => {
    const size = 5 * 4 // 5 floats
    const buffer = device.createBuffer(
      size,
      usage.copyDst | usage.mapRead,
      false
    )

    // Write data
    const data = new Float32Array([1.0, 2.0, 3.0, 4.0, 5.0])
    device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))

    // Need to submit commands and poll
    const encoder = device.createCommandEncoder()
    device.queueSubmit(encoder.finish())
    device.poll(true)

    // Read back
    const readData = await buffer.mapRead()
    const floats = new Float32Array(readData.buffer, readData.byteOffset, 5)

    expect(floats[0]).toBe(1.0)
    expect(floats[1]).toBe(2.0)
    expect(floats[2]).toBe(3.0)
    expect(floats[3]).toBe(4.0)
    expect(floats[4]).toBe(5.0)

    buffer.unmap()
    buffer.destroy()
  })

  test('should copy buffer to buffer', () => {
    const size = 64
    const srcBuffer = device.createBuffer(size, usage.copySrc | usage.copyDst, false)
    const dstBuffer = device.createBuffer(size, usage.copyDst | usage.mapRead, false)

    // Write to source
    const data = new Uint32Array([1, 2, 3, 4])
    device.queueWriteBuffer(srcBuffer, 0, Buffer.from(data.buffer))

    // Copy
    const encoder = device.createCommandEncoder()
    device.copyBufferToBuffer(encoder, srcBuffer, 0, dstBuffer, 0, size)
    const cmdBuffer = encoder.finish()
    device.queueSubmit(cmdBuffer)
    device.poll(true)

    srcBuffer.destroy()
    dstBuffer.destroy()
  })

  test('should handle buffer with different usage flags', () => {
    const buffers = [
      { usage: usage.vertex, name: 'vertex' },
      { usage: usage.index, name: 'index' },
      { usage: usage.uniform, name: 'uniform' },
      { usage: usage.storage, name: 'storage' },
      { usage: usage.indirect, name: 'indirect' },
    ]

    buffers.forEach(({ usage: u, name }) => {
      const buffer = device.createBuffer(256, u | usage.copyDst, false)
      expect(buffer).toBeDefined()
      expect(buffer.size()).toBe(256)
      buffer.destroy()
    })
  })
})
