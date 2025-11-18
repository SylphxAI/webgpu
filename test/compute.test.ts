import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, bufferUsage } from '../index.js'

describe('GPU Compute Pipeline', () => {
  let device: any
  const usage = bufferUsage()

  beforeAll(async () => {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create shader module', () => {
    const shaderCode = `
      @group(0) @binding(0) var<storage, read> input: array<f32>;
      @group(0) @binding(1) var<storage, read_write> output: array<f32>;

      @compute @workgroup_size(1)
      fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        output[global_id.x] = input[global_id.x] * 2.0;
      }
    `

    const shader = device.createShaderModule(shaderCode)
    expect(shader).toBeDefined()
  })

  test('should create bind group layout', () => {
    const layoutDesc = {
      label: 'Test Layout',
      entries: [
        { binding: 0, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 1, visibility: 4, bufferType: 'storage' },
      ],
    }

    const layout = device.createBindGroupLayout(layoutDesc)
    expect(layout).toBeDefined()
  })

  test('should create pipeline layout', () => {
    const bindGroupLayoutDesc = {
      entries: [
        { binding: 0, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 1, visibility: 4, bufferType: 'storage' },
      ],
    }
    const bindGroupLayout = device.createBindGroupLayout(bindGroupLayoutDesc)

    const pipelineLayout = device.createPipelineLayout('Test Pipeline Layout', [bindGroupLayout])
    expect(pipelineLayout).toBeDefined()
  })

  test('should create compute pipeline', () => {
    const shaderCode = `
      @group(0) @binding(0) var<storage, read> input: array<f32>;
      @group(0) @binding(1) var<storage, read_write> output: array<f32>;

      @compute @workgroup_size(1)
      fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        output[global_id.x] = input[global_id.x] * 2.0;
      }
    `
    const shader = device.createShaderModule(shaderCode)

    const bindGroupLayoutDesc = {
      entries: [
        { binding: 0, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 1, visibility: 4, bufferType: 'storage' },
      ],
    }
    const bindGroupLayout = device.createBindGroupLayout(bindGroupLayoutDesc)
    const pipelineLayout = device.createPipelineLayout('Pipeline Layout', [bindGroupLayout])

    const pipeline = device.createComputePipeline('Test Pipeline', pipelineLayout, shader, 'main')
    expect(pipeline).toBeDefined()
  })

  test('should execute compute shader (vector addition)', async () => {
    // Shader: output = a + b
    const shaderCode = `
      @group(0) @binding(0) var<storage, read> a: array<f32>;
      @group(0) @binding(1) var<storage, read> b: array<f32>;
      @group(0) @binding(2) var<storage, read_write> result: array<f32>;

      @compute @workgroup_size(1)
      fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        result[global_id.x] = a[global_id.x] + b[global_id.x];
      }
    `

    const shader = device.createShaderModule(shaderCode)

    // Create buffers
    const size = 5
    const bufferSize = size * 4

    const bufferA = device.createBuffer(bufferSize, usage.storage | usage.copyDst, false)
    const bufferB = device.createBuffer(bufferSize, usage.storage | usage.copyDst, false)
    const bufferResult = device.createBuffer(bufferSize, usage.storage | usage.copySrc, false)
    const bufferRead = device.createBuffer(bufferSize, usage.copyDst | usage.mapRead, false)

    // Write input data
    const dataA = new Float32Array([1, 2, 3, 4, 5])
    const dataB = new Float32Array([10, 20, 30, 40, 50])
    device.queueWriteBuffer(bufferA, 0, Buffer.from(dataA.buffer))
    device.queueWriteBuffer(bufferB, 0, Buffer.from(dataB.buffer))

    // Create pipeline
    const layoutDesc = {
      entries: [
        { binding: 0, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 1, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 2, visibility: 4, bufferType: 'storage' },
      ],
    }
    const layout = device.createBindGroupLayout(layoutDesc)
    const pipelineLayout = device.createPipelineLayout('Compute Layout', [layout])
    const pipeline = device.createComputePipeline('Compute Pipeline', pipelineLayout, shader, 'main')

    // Create bind group
    const bindGroup = device.createBindGroupBuffers('Compute Bind Group', layout, [bufferA, bufferB, bufferResult])

    // Execute compute
    const encoder = device.createCommandEncoder()
    encoder.computePass(pipeline, [bindGroup], size, 1, 1)
    device.copyBufferToBuffer(encoder, bufferResult, 0, bufferRead, 0, bufferSize)
    device.queueSubmit(encoder.finish())
    device.poll(true)

    // Read results
    const resultData = await bufferRead.mapRead()
    const results = new Float32Array(resultData.buffer, resultData.byteOffset, size)

    // Verify results
    expect(results[0]).toBe(11)
    expect(results[1]).toBe(22)
    expect(results[2]).toBe(33)
    expect(results[3]).toBe(44)
    expect(results[4]).toBe(55)

    bufferRead.unmap()

    // Cleanup
    bufferA.destroy()
    bufferB.destroy()
    bufferResult.destroy()
    bufferRead.destroy()
  })

  test('should execute compute with multiple workgroups', async () => {
    const shaderCode = `
      @group(0) @binding(0) var<storage, read_write> data: array<f32>;

      @compute @workgroup_size(4)
      fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        data[global_id.x] = f32(global_id.x);
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const size = 16
    const bufferSize = size * 4

    const buffer = device.createBuffer(bufferSize, usage.storage | usage.copySrc, false)
    const readBuffer = device.createBuffer(bufferSize, usage.copyDst | usage.mapRead, false)

    const layoutDesc = {
      entries: [{ binding: 0, visibility: 4, bufferType: 'storage' }],
    }
    const layout = device.createBindGroupLayout(layoutDesc)
    const pipelineLayout = device.createPipelineLayout('Layout', [layout])
    const pipeline = device.createComputePipeline('Pipeline', pipelineLayout, shader, 'main')
    const bindGroup = device.createBindGroupBuffers('Bind Group', layout, [buffer])

    const encoder = device.createCommandEncoder()
    encoder.computePass(pipeline, [bindGroup], 4, 1, 1) // 4 workgroups * 4 workgroup_size = 16 invocations
    device.copyBufferToBuffer(encoder, buffer, 0, readBuffer, 0, bufferSize)
    device.queueSubmit(encoder.finish())
    device.poll(true)

    const resultData = await readBuffer.mapRead()
    const results = new Float32Array(resultData.buffer, resultData.byteOffset, size)

    // Verify sequential values
    for (let i = 0; i < size; i++) {
      expect(results[i]).toBe(i)
    }

    readBuffer.unmap()
    buffer.destroy()
    readBuffer.destroy()
  })
})
