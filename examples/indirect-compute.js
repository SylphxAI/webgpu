const { Gpu, bufferUsage: getBufferUsage } = require('../index.js')

async function main() {
  console.log('🔢 WebGPU Indirect Compute Dispatch Example\n')

  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  console.log('✓ Device ready\n')

  const bufferUsage = getBufferUsage()

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

  const shaderModule = device.createShaderModule(shaderCode)
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

  const bufferA = device.createBuffer(
    byteSize,
    bufferUsage.storage | bufferUsage.copyDst,
    false
  )
  device.queueWriteBuffer(bufferA, 0, Buffer.from(inputA.buffer))

  const bufferB = device.createBuffer(
    byteSize,
    bufferUsage.storage | bufferUsage.copyDst,
    false
  )
  device.queueWriteBuffer(bufferB, 0, Buffer.from(inputB.buffer))

  const outputBuffer = device.createBuffer(
    byteSize,
    bufferUsage.storage | bufferUsage.copySrc,
    false
  )

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

  const indirectBuffer = device.createBuffer(
    indirectParams.byteLength,
    bufferUsage.indirect | bufferUsage.copyDst,
    false
  )
  device.queueWriteBuffer(indirectBuffer, 0, Buffer.from(indirectParams.buffer))
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

  // Execute indirect compute pass
  const encoder = device.createCommandEncoder()

  encoder.computePassIndirect(
    pipeline,
    [bindGroup],
    indirectBuffer,
    0 // offset
  )

  // Copy result to readable buffer
  const readbackBuffer = device.createBuffer(
    byteSize,
    bufferUsage.copyDst | bufferUsage.mapRead,
    false
  )

  device.copyBufferToBuffer(encoder, outputBuffer, 0, readbackBuffer, 0, byteSize)

  device.queueSubmit(encoder.finish())
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
