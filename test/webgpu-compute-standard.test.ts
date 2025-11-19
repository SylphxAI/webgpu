/**
 * WebGPU Standard Compute Pipeline Tests
 *
 * Comprehensive tests for compute shaders, compute pipelines, and compute passes.
 * All APIs conform to W3C WebGPU specification.
 */

import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, GPUBufferUsage } from '../webgpu.js'

describe('WebGPU Standard: Compute Pass', () => {
  let device: Awaited<ReturnType<Awaited<ReturnType<ReturnType<typeof Gpu>['requestAdapter']>>['requestDevice']>>

  beforeAll(async () => {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create and run compute pass (standard)', () => {
    // Create shader module
    const shaderModule = device.createShaderModule({
      code: `
        @compute @workgroup_size(1)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
          // Simple compute shader
        }
      `
    })

    // Create pipeline
    const pipeline = device.createComputePipeline({
      layout: null,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    // Create command encoder and compute pass
    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()

    // Standard: setPipeline, dispatchWorkgroups, end
    computePass.setPipeline(pipeline)
    computePass.dispatchWorkgroups(1, 1, 1)
    computePass.end()

    // Submit
    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    expect(computePass).toBeDefined()
  })

  test('should run compute with storage buffer (standard)', async () => {
    // Create storage buffer
    const buffer = device.createBuffer({
      size: 16,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST
    })

    // Write initial data
    const inputData = new Float32Array([1, 2, 3, 4])
    device.queue.writeBuffer(buffer, 0, Buffer.from(inputData.buffer))

    // Create shader that doubles each value
    const shaderModule = device.createShaderModule({
      code: `
        struct Data {
          values: array<f32>
        }

        @group(0) @binding(0)
        var<storage, read_write> data: Data;

        @compute @workgroup_size(4)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
          let index = global_id.x;
          if (index < 4u) {
            data.values[index] = data.values[index] * 2.0;
          }
        }
      `
    })

    // Create bind group layout
    const bindGroupLayout = device.createBindGroupLayout({
      entries: [
        {
          binding: 0,
          visibility: 4, // COMPUTE
          buffer: {
            type: 'storage'
          }
        }
      ]
    })

    // Create pipeline layout
    const pipelineLayout = device.createPipelineLayout({
      bindGroupLayouts: [bindGroupLayout]
    })

    // Create pipeline
    const pipeline = device.createComputePipeline({
      layout: pipelineLayout,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    // Create bind group
    const bindGroup = device.createBindGroup({
      layout: bindGroupLayout,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: buffer
          }
        }
      ]
    })

    // Run compute pass
    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()
    computePass.setPipeline(pipeline)
    computePass.setBindGroup(0, bindGroup)
    computePass.dispatchWorkgroups(1)
    computePass.end()

    // Submit and wait
    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    buffer.destroy()
  })

  test('should support compute pass descriptors (standard)', () => {
    const encoder = device.createCommandEncoder()

    // Standard: beginComputePass can take descriptor with label
    const computePass = encoder.beginComputePass({
      label: 'test-compute-pass'
    })

    expect(computePass).toBeDefined()
    computePass.end()

    encoder.finish()
  })

  test('should support debug groups in compute pass (standard)', () => {
    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()

    // Standard: debug markers and groups
    computePass.pushDebugGroup('test-group')
    computePass.insertDebugMarker('test-marker')
    computePass.popDebugGroup()
    computePass.end()

    encoder.finish()
  })

  test('should dispatch workgroups with all dimensions (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @compute @workgroup_size(8, 8, 1)
        fn main() {}
      `
    })

    const pipeline = device.createComputePipeline({
      layout: null,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()
    computePass.setPipeline(pipeline)

    // Standard: dispatchWorkgroups(x, y, z)
    computePass.dispatchWorkgroups(4, 4, 1)
    computePass.end()

    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)
  })

  test('should support indirect dispatch (standard)', () => {
    // Create indirect buffer with dispatch params
    const indirectBuffer = device.createBuffer({
      size: 12, // 3 x u32 (x, y, z)
      usage: GPUBufferUsage.INDIRECT | GPUBufferUsage.COPY_DST
    })

    // Write dispatch parameters
    const params = new Uint32Array([1, 1, 1])
    device.queue.writeBuffer(indirectBuffer, 0, Buffer.from(params.buffer))

    const shaderModule = device.createShaderModule({
      code: `
        @compute @workgroup_size(1)
        fn main() {}
      `
    })

    const pipeline = device.createComputePipeline({
      layout: null,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()
    computePass.setPipeline(pipeline)

    // Standard: dispatchWorkgroupsIndirect
    computePass.dispatchWorkgroupsIndirect(indirectBuffer, 0)
    computePass.end()

    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    indirectBuffer.destroy()
  })

  test('should support multiple bind groups in compute pass (standard)', () => {
    // Create buffers for multiple bind groups
    const buffer1 = device.createBuffer({
      size: 16,
      usage: GPUBufferUsage.STORAGE
    })

    const buffer2 = device.createBuffer({
      size: 16,
      usage: GPUBufferUsage.STORAGE
    })

    // Create bind group layouts
    const layout1 = device.createBindGroupLayout({
      entries: [
        {
          binding: 0,
          visibility: 4, // COMPUTE
          buffer: { type: 'storage' }
        }
      ]
    })

    const layout2 = device.createBindGroupLayout({
      entries: [
        {
          binding: 0,
          visibility: 4, // COMPUTE
          buffer: { type: 'storage' }
        }
      ]
    })

    // Create pipeline layout with multiple bind groups
    const pipelineLayout = device.createPipelineLayout({
      bindGroupLayouts: [layout1, layout2]
    })

    // Create bind groups
    const bindGroup1 = device.createBindGroup({
      layout: layout1,
      entries: [
        {
          binding: 0,
          resource: { buffer: buffer1 }
        }
      ]
    })

    const bindGroup2 = device.createBindGroup({
      layout: layout2,
      entries: [
        {
          binding: 0,
          resource: { buffer: buffer2 }
        }
      ]
    })

    const shaderModule = device.createShaderModule({
      code: `
        @group(0) @binding(0)
        var<storage, read_write> data1: array<f32>;

        @group(1) @binding(0)
        var<storage, read_write> data2: array<f32>;

        @compute @workgroup_size(1)
        fn main() {}
      `
    })

    const pipeline = device.createComputePipeline({
      layout: pipelineLayout,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()
    computePass.setPipeline(pipeline)

    // Standard: setBindGroup for each group index
    computePass.setBindGroup(0, bindGroup1)
    computePass.setBindGroup(1, bindGroup2)

    computePass.dispatchWorkgroups(1)
    computePass.end()

    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    buffer1.destroy()
    buffer2.destroy()
  })

  test('should support dynamic offsets in bind groups (standard)', () => {
    // Create buffer large enough for multiple uniform blocks
    const buffer = device.createBuffer({
      size: 256,
      usage: GPUBufferUsage.UNIFORM
    })

    const layout = device.createBindGroupLayout({
      entries: [
        {
          binding: 0,
          visibility: 4, // COMPUTE
          buffer: {
            type: 'uniform',
            hasDynamicOffset: true
          }
        }
      ]
    })

    const bindGroup = device.createBindGroup({
      layout: layout,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: buffer,
            offset: 0,
            size: 16
          }
        }
      ]
    })

    const pipelineLayout = device.createPipelineLayout({
      bindGroupLayouts: [layout]
    })

    const shaderModule = device.createShaderModule({
      code: `
        struct Uniforms {
          value: vec4<f32>
        }

        @group(0) @binding(0)
        var<uniform> uniforms: Uniforms;

        @compute @workgroup_size(1)
        fn main() {}
      `
    })

    const pipeline = device.createComputePipeline({
      layout: pipelineLayout,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    const encoder = device.createCommandEncoder()
    const computePass = encoder.beginComputePass()
    computePass.setPipeline(pipeline)

    // Standard: setBindGroup with dynamic offsets array
    computePass.setBindGroup(0, bindGroup, [0])
    computePass.dispatchWorkgroups(1)
    computePass.end()

    const commandBuffer = encoder.finish()
    device.queue.submit(commandBuffer)

    buffer.destroy()
  })
})

describe('WebGPU Standard: Compute Pipeline Auto Layout', () => {
  let device: Awaited<ReturnType<Awaited<ReturnType<ReturnType<typeof Gpu>['requestAdapter']>>['requestDevice']>>

  beforeAll(async () => {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should create pipeline with auto layout (standard)', () => {
    const shaderModule = device.createShaderModule({
      code: `
        @compute @workgroup_size(1)
        fn main() {}
      `
    })

    // Standard: layout: null for automatic layout (no bind groups)
    const pipeline = device.createComputePipeline({
      layout: null,
      compute: {
        module: shaderModule,
        entryPoint: 'main'
      }
    })

    expect(pipeline).toBeDefined()
    expect(pipeline).toHaveProperty('getBindGroupLayout')
  })
})
