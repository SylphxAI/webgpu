const { Gpu, bufferUsage: getBufferUsage } = require('../index.js')
const bufferUsage = getBufferUsage()

async function main() {
    console.log('🧮 WebGPU Compute Example: Vector Addition\n')

    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()
    console.log('✓ Device ready\n')

    // Input data: two arrays of numbers to add
    const input1 = new Float32Array([1, 2, 3, 4, 5])
    const input2 = new Float32Array([10, 20, 30, 40, 50])

    console.log('Input arrays:')
    console.log('  A:', Array.from(input1))
    console.log('  B:', Array.from(input2))

    // Create buffers
    const size = input1.byteLength
    const buffer1 = device.createBuffer(size, bufferUsage.storage | bufferUsage.copyDst, false)
    const buffer2 = device.createBuffer(size, bufferUsage.storage | bufferUsage.copyDst, false)
    const resultBuffer = device.createBuffer(
        size,
        bufferUsage.storage | bufferUsage.copySrc,
        false
    )
    const readBuffer = device.createBuffer(
        size,
        bufferUsage.copyDst | bufferUsage.mapRead,
        false
    )

    // Write input data
    device.queueWriteBuffer(buffer1, 0, Buffer.from(input1.buffer))
    device.queueWriteBuffer(buffer2, 0, Buffer.from(input2.buffer))
    console.log('\n✓ Input data written to buffers')

    // Compute shader: simple vector addition
    const shaderCode = `
        @group(0) @binding(0) var<storage, read> input1: array<f32>;
        @group(0) @binding(1) var<storage, read> input2: array<f32>;
        @group(0) @binding(2) var<storage, read_write> output: array<f32>;

        @compute @workgroup_size(1)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            let i = global_id.x;
            if (i < ${input1.length}u) {
                output[i] = input1[i] + input2[i];
            }
        }
    `

    const shaderModule = device.createShaderModule(shaderCode)
    console.log('✓ Shader compiled')

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
    console.log('✓ Bind group layout created')

    // Create pipeline layout
    const pipelineLayout = device.createPipelineLayout('Compute Pipeline Layout', [bindGroupLayout])
    console.log('✓ Pipeline layout created')

    // Create compute pipeline
    const pipeline = device.createComputePipeline(
        'Vector Addition Pipeline',
        pipelineLayout,
        shaderModule,
        'main'
    )
    console.log('✓ Compute pipeline created')

    // Create bind group
    const bindGroup = device.createBindGroupBuffers(
        'Compute Bind Group',
        bindGroupLayout,
        [buffer1, buffer2, resultBuffer]
    )
    console.log('✓ Bind group created')

    // Create command encoder and execute compute pass
    const encoder = device.createCommandEncoder()
    encoder.computePass(pipeline, [bindGroup], input1.length)
    const commandBuffer = encoder.finish()
    console.log('✓ Commands encoded')

    // Submit to queue
    device.queueSubmit(commandBuffer)
    device.poll(true)
    console.log('✓ GPU work complete\n')

    // TODO: Add copy from resultBuffer to readBuffer once copyBufferToBuffer is implemented
    // For now, we just show the pipeline is working

    console.log('✅ Compute pipeline successfully executed!')
    console.log('Note: Buffer reading to verify results will be added in next iteration')

    // Cleanup
    buffer1.destroy()
    buffer2.destroy()
    resultBuffer.destroy()
    readBuffer.destroy()
    device.destroy()

    console.log('\n✨ Compute example completed')
}

main().catch(err => {
    console.error('❌ Error:', err.message)
    console.error(err.stack)
    process.exit(1)
})
