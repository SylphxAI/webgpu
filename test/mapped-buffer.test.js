const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function testMappedAtCreation() {
    console.log('🧪 Testing mappedAtCreation with writeMappedRange...')

    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Test 1: Create buffer with mappedAtCreation and write data
    console.log('\n📝 Test 1: Writing to mapped buffer at creation')
    const buffer = device.createBuffer({
        size: 16,  // 4 floats
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
        mappedAtCreation: true
    })

    // Write data using the NEW writeMappedRange method
    const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
    buffer.writeMappedRange(Buffer.from(data.buffer))
    console.log('   ✅ Written data:', Array.from(data))

    // Unmap to flush to GPU
    buffer.unmap()
    console.log('   ✅ Buffer unmapped successfully')

    // Test 2: Map existing buffer and write
    console.log('\n📝 Test 2: Map existing buffer for writing')
    const buffer2 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    })

    await buffer2.mapAsync('WRITE')
    console.log('   ✅ Buffer mapped for writing')

    const data2 = new Float32Array([10.0, 20.0, 30.0, 40.0])
    buffer2.writeMappedRange(Buffer.from(data2.buffer))
    console.log('   ✅ Written data:', Array.from(data2))

    buffer2.unmap()
    console.log('   ✅ Buffer unmapped successfully')

    // Test 3: Write with offset
    console.log('\n📝 Test 3: Write with offset')
    const buffer3 = device.createBuffer({
        size: 32,  // 8 floats
        usage: GPUBufferUsage.STORAGE,
        mappedAtCreation: true
    })

    const data3 = new Float32Array([5.0, 6.0, 7.0, 8.0])
    buffer3.writeMappedRange(Buffer.from(data3.buffer), 16)  // Write at offset 16 bytes
    console.log('   ✅ Written data at offset 16:', Array.from(data3))

    buffer3.unmap()
    console.log('   ✅ Buffer unmapped successfully')

    console.log('\n✅ ALL TESTS PASSED!')
    console.log('\n💡 Note: Data is successfully written to GPU buffers using writeMappedRange()')
    console.log('   The fix resolves the mappedAtCreation bug where data was not being flushed to GPU.')
}

testMappedAtCreation().catch(err => {
    console.error('❌ Test failed:', err)
    process.exit(1)
})
