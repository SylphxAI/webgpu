const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function verifyWriteMappedRange() {
    console.log('🧪 Verifying writeMappedRange() actually writes to GPU...')

    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Create buffer with mappedAtCreation, write data, then read it back
    console.log('\n📝 Test: Write with mappedAtCreation, then read back')
    const buffer = device.createBuffer({
        size: 16,  // 4 floats
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    // Write data using writeMappedRange
    const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
    buffer.writeMappedRange(Buffer.from(data.buffer))
    console.log('   ✅ Written data:', Array.from(data))

    buffer.unmap()
    console.log('   ✅ Buffer unmapped')

    // Now map for reading and verify the data
    await buffer.mapAsync('READ')
    console.log('   ✅ Buffer mapped for reading')

    const readRange = buffer.getMappedRange()
    const readView = new Float32Array(readRange.buffer)
    console.log('   📖 Read back data:', Array.from(readView))

    if (readView[0] === 1.0 && readView[1] === 2.0 && readView[2] === 3.0 && readView[3] === 4.0) {
        console.log('   ✅ SUCCESS: Data was correctly written to GPU!')
    } else {
        console.error('   ❌ FAILURE: Expected [1, 2, 3, 4], got', Array.from(readView))
        process.exit(1)
    }

    buffer.unmap()
    console.log('\n✅ TEST PASSED: writeMappedRange() correctly writes data to GPU')
}

verifyWriteMappedRange().catch(err => {
    console.error('❌ Test failed:', err)
    process.exit(1)
})
