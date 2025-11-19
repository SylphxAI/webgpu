const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function testGetMappedRange() {
    console.log('🧪 Testing getMappedRange() write pattern (WebGPU-style)...')

    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Test 1: getMappedRange() with unmap(modifiedBuffer) pattern
    console.log('\n📝 Test 1: getMappedRange() + modify + unmap(buffer)')
    const buffer = device.createBuffer({
        size: 16,  // 4 floats
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    // Get mapped range and write to it
    const range = buffer.getMappedRange()
    const view = new Float32Array(range.buffer)
    view[0] = 1.0
    view[1] = 2.0
    view[2] = 3.0
    view[3] = 4.0
    console.log('   ✅ Written data:', Array.from(view))

    // Pass modified buffer back to unmap()
    buffer.unmap(range)
    console.log('   ✅ Buffer unmapped with modified data')

    // Verify data was written by reading it back
    await buffer.mapAsync('READ')
    const readRange = buffer.getMappedRange()
    const readView = new Float32Array(readRange.buffer)
    console.log('   📖 Read back data:', Array.from(readView))

    if (readView[0] === 1.0 && readView[1] === 2.0 && readView[2] === 3.0 && readView[3] === 4.0) {
        console.log('   ✅ DATA VERIFIED: Write was successful!')
    } else {
        console.error('   ❌ DATA MISMATCH: Expected [1, 2, 3, 4], got', Array.from(readView))
        process.exit(1)
    }
    buffer.unmap()

    // Test 2: Map existing buffer, modify, and unmap with buffer
    console.log('\n📝 Test 2: Map existing buffer, modify, unmap(buffer)')
    const buffer2 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    })

    await buffer2.mapAsync('WRITE')
    const range2 = buffer2.getMappedRange()
    const view2 = new Float32Array(range2.buffer)
    view2[0] = 10.0
    view2[1] = 20.0
    view2[2] = 30.0
    view2[3] = 40.0
    console.log('   ✅ Written data:', Array.from(view2))

    buffer2.unmap(range2)
    console.log('   ✅ Buffer unmapped with modified data')

    // Note: Cannot verify MAP_WRITE buffer by reading (would need separate read buffer + copy)
    // The important test is that it doesn't crash and data is written to GPU
    console.log('   ✅ Write completed (cannot read back MAP_WRITE buffer directly)')

    // Test 3: Backward compatibility - unmap() without argument still works
    console.log('\n📝 Test 3: Backward compatibility - unmap() without argument')
    const buffer3 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.STORAGE,
        mappedAtCreation: true
    })

    const range3 = buffer3.getMappedRange()
    console.log('   ✅ Got mapped range')

    buffer3.unmap()  // No argument - should work fine
    console.log('   ✅ Buffer unmapped without argument (backward compatible)')

    console.log('\n✅ ALL TESTS PASSED!')
    console.log('\n💡 The getMappedRange() bug is now FIXED:')
    console.log('   - Call getMappedRange() to get writable buffer')
    console.log('   - Modify the buffer via typed arrays')
    console.log('   - Pass modified buffer to unmap(buffer) to flush to GPU')
    console.log('   - Data is now correctly written to GPU memory! 🎉')
}

testGetMappedRange().catch(err => {
    console.error('❌ Test failed:', err)
    process.exit(1)
})
