const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function main() {
  console.log('🔢 WebGPU Indirect Compute Dispatch Example\n')

  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  // Compute shader for vector addition
  const shaderCode = `
@group(0) @binding(0) var<storage, read> input_a: array<f32>;
@group(0) @binding(1) var<storage, read> input_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3u) {
    let index = global_id.x;
    if (index < arrayLength(&output)) {
        output[index] = input_a[index] + input_b[index];
    }
}
`

  const shaderModule = device.createShaderModule({ code: shaderCode })
  console.log('✓ Compute shader compiled')

  // Create input/output buffers
  const arraySize = 256
  const byteSize = arraySize * 4

  const inputA = new Float32Array(arraySize)
  const inputB = new Float32Array(arraySize)
  for (let i = 0; i < arraySize; i++) {
    inputA[i] = i
    inputB[i] = i * 10
  }

  const bufferA = device.createBuffer({
    size: byteSize,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(bufferA, 0, Buffer.from(inputA.buffer))

  const bufferB = device.createBuffer({
    size: byteSize,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(bufferB, 0, Buffer.from(inputB.buffer))

  const outputBuffer = device.createBuffer({
    size: byteSize,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
    mappedAtCreation: false
  })

  console.log(`✓ Buffers created (${arraySize} elements each)`)

  // Create indirect dispatch buffer
  // Format: [workgroups_x: u32, workgroups_y: u32, workgroups_z: u32]
  const workgroupSize = 64
  const numWorkgroups = Math.ceil(arraySize / workgroupSize)

  const indirectParams = new Uint32Array([
    numWorkgroups,  // workgroups_x (4 workgroups for 256 elements with size 64)
    1,              // workgroups_y
    1,              // workgroups_z
  ])

  const indirectBuffer = device.createBuffer({
    size: indirectParams.byteLength,
    usage: GPUBufferUsage.INDIRECT | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(indirectBuffer, 0, Buffer.from(indirectParams.buffer))
  console.log('✓ Indirect dispatch buffer created')
  console.log(`   Workgroups: ${numWorkgroups}x1x1 (from GPU buffer)`)

  // Create bind group layout
  const bindGroupLayout = device.createBindGroupLayout({
    label: 'Compute Bind Group Layout',
    entries: [
      {
        binding: 0,
        visibility: 0x4, // COMPUTE
        bufferType: 'read-only-storage',
      },
      {
        binding: 1,
        visibility: 0x4, // COMPUTE
        bufferType: 'read-only-storage',
      },
      {
        binding: 2,
        visibility: 0x4, // COMPUTE
        bufferType: 'storage',
      },
    ],
  })

  const bindGroup = device.createBindGroupBuffers(
    'Compute Bind Group',
    bindGroupLayout,
    [bufferA, bufferB, outputBuffer]
  )

  // Create pipeline
  const pipelineLayout = device.createPipelineLayout('Compute Pipeline Layout', [bindGroupLayout])
  const pipeline = device.createComputePipeline(
    'Vector Add Pipeline',
    pipelineLayout,
    shaderModule,
    'main'
  )
  console.log('✓ Compute pipeline created')

  // Execute indirect compute pass - WebGPU standard API
  const encoder = device.createCommandEncoder()

  const pass = encoder.beginComputePass()
  pass.setPipeline(pipeline)
  pass.setBindGroup(0, bindGroup)
  pass.dispatchWorkgroupsIndirect(indirectBuffer, 0)
  pass.end()

  // Copy result to readable buffer
  const readbackBuffer = device.createBuffer({
    size: byteSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: false
  })

  encoder.copyBufferToBuffer( outputBuffer, 0, readbackBuffer, 0, byteSize)

  device.queue.submit([encoder.finish()])
  device.poll(true)
  console.log('✓ Indirect compute dispatch executed')

  // Read back results
  const data = await readbackBuffer.mapRead()
  const result = new Float32Array(data.buffer, data.byteOffset, arraySize)

  // Verify results (show first 5)
  console.log('\n📊 Results (first 5 elements):')
  let allCorrect = true
  for (let i = 0; i < Math.min(5, arraySize); i++) {
    const expected = inputA[i] + inputB[i]
    const actual = result[i]
    const match = Math.abs(actual - expected) < 0.001
    console.log(`   ${inputA[i]} + ${inputB[i]} = ${actual} ${match ? '✓' : '✗ (expected ' + expected + ')'}`)
    if (!match) allCorrect = false
  }

  readbackBuffer.unmap()

  if (allCorrect) {
    console.log('\n✅ Indirect compute verified! GPU calculations correct')
  } else {
    console.log('\n⚠️  Some calculations incorrect')
  }

  console.log('\n✅ Indirect compute example complete!')
  console.log('   - Indirect dispatch buffer created ✓')
  console.log('   - Dispatch parameters from GPU buffer ✓')
  console.log('   - GPU-driven compute ✓')

  // Cleanup
  bufferA.destroy()
  bufferB.destroy()
  outputBuffer.destroy()
  indirectBuffer.destroy()
  readbackBuffer.destroy()
  device.destroy()

  console.log('\n✨ Example completed')
}

main().catch(console.error)
