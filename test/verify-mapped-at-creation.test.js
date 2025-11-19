/**
 * Test mappedAtCreation buffer write pattern
 *
 * User reports: All versions (including 0.11.0) fail to write data to GPU
 * when using mappedAtCreation pattern.
 */

const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function testMappedAtCreation() {
    console.log('🧪 Testing mappedAtCreation buffer write pattern...\n')

    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Test: Create buffer with mappedAtCreation, write data, unmap, read back
    console.log('📝 Test: mappedAtCreation → getMappedRange() → modify → unmap() → read back')

    const buffer = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    console.log('   ✅ Buffer created with mappedAtCreation: true')

    // Get mapped range and write data
    const range = buffer.getMappedRange()
    const arr = new Float32Array(range, 0, 4)
    arr.set([1, 2, 3, 4])

    console.log('   ✅ Before unmap:', Array.from(arr))

    // Unmap
    buffer.unmap()
    console.log('   ✅ Buffer unmapped')

    // Read back from GPU
    await buffer.mapAsync('READ')
    const readRange = buffer.getMappedRange()
    const readArr = new Float32Array(readRange.buffer, readRange.byteOffset, 4)

    console.log('   📖 GPU result:  ', Array.from(readArr))

    if (readArr[0] === 0 && readArr[1] === 0 && readArr[2] === 0 && readArr[3] === 0) {
        console.log('\n❌ BUG CONFIRMED: mappedAtCreation data NOT written to GPU!')
        console.log('   Expected: [1, 2, 3, 4]')
        console.log('   Got:      [0, 0, 0, 0]')
        process.exit(1)
    } else if (readArr[0] === 1 && readArr[1] === 2 && readArr[2] === 3 && readArr[3] === 4) {
        console.log('\n✅ SUCCESS: mappedAtCreation data correctly written to GPU!')
    } else {
        console.log('\n⚠️  UNEXPECTED: Got partial data:', Array.from(readArr))
        process.exit(1)
    }

    buffer.unmap()
    buffer.destroy()
}

testMappedAtCreation().catch(err => {
    console.error('❌ Test failed with error:', err)
    process.exit(1)
})
