const { Gpu, bufferUsage } = require('../index.js')

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
    const buffer1 = device.createBuffer(size, bufferUsage.STORAGE | bufferUsage.COPY_DST, false)
    const buffer2 = device.createBuffer(size, bufferUsage.STORAGE | bufferUsage.COPY_DST, false)
    const resultBuffer = device.createBuffer(
        size,
        bufferUsage.STORAGE | bufferUsage.COPY_SRC,
        false
    )
    const readBuffer = device.createBuffer(
        size,
        bufferUsage.COPY_DST | bufferUsage.MAP_READ,
        false
    )

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
    console.log('\n✓ Shader compiled')

    // TODO: Complete compute pipeline setup once implemented

    console.log('\n⚠️  Note: Compute pipeline implementation in progress')
    console.log('This example demonstrates the API structure.')

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
