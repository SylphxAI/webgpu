/**
 * Test native Rust getMappedRange() + unmap() fix
 *
 * This tests the NATIVE implementation (index.js) directly,
 * bypassing the JavaScript wrapper (webgpu.js).
 */

const native = require('../index.js')

async function testNativeUnmapFix() {
    console.log('🧪 Testing NATIVE Rust unmap() fix...\n')

    const gpu = native.Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    const usage = native.bufferUsage()

    // Create buffer with mappedAtCreation (standard descriptor format)
    const buffer = device.createBuffer({
        size: 16,
        usage: usage.copyDst | usage.mapRead,
        mappedAtCreation: true
    })

    console.log('   ✅ Buffer created with mappedAtCreation: true')

    // Get mapped range and write data
    const range = buffer.getMappedRange()
    const arr = new Float32Array(range.buffer, range.byteOffset, 4)
    arr.set([1, 2, 3, 4])

    console.log('   ✅ Before unmap:', Array.from(arr))

    // TEST: Pass modified buffer to unmap (Rust should accept Option<Buffer>)
    buffer.unmap(range)
    console.log('   ✅ Buffer unmapped with modified data')

    // Read back from GPU
    await buffer.mapAsync('READ')
    const readRange = buffer.getMappedRange()
    const readArr = new Float32Array(readRange.buffer, readRange.byteOffset, 4)

    console.log('   📖 GPU result:  ', Array.from(readArr))

    if (readArr[0] === 0 && readArr[1] === 0) {
        console.log('\n❌ NATIVE FIX FAILED: Data not written to GPU!')
        console.log('   Expected: [1, 2, 3, 4]')
        console.log('   Got:      [0, 0, 0, 0]')
        process.exit(1)
    } else if (readArr[0] === 1 && readArr[1] === 2 && readArr[2] === 3 && readArr[3] === 4) {
        console.log('\n✅ NATIVE FIX WORKS: Data correctly written to GPU!')
        console.log('   No JavaScript wrapper needed! 🎉')
    } else {
        console.log('\n⚠️  UNEXPECTED: Got partial data:', Array.from(readArr))
        process.exit(1)
    }

    buffer.unmap()
    buffer.destroy()
}

testNativeUnmapFix().catch(err => {
    console.error('❌ Test failed with error:', err)
    process.exit(1)
})
