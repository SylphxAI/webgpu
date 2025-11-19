/**
 * Test to reproduce buffer.unmap() not flushing data to GPU
 *
 * Issue: getMappedRange() returns a COPY, so modifications don't reach GPU
 */

const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function reproduceBug() {
    console.log('🐛 Reproducing buffer.unmap() bug...\n')

    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Test 1: Standard WebGPU pattern (SHOULD work but doesn't)
    console.log('📝 Test 1: Standard WebGPU getMappedRange() pattern')
    const buffer1 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    // Get mapped range and write data
    const range1 = buffer1.getMappedRange()
    const arr1 = new Float32Array(range1.buffer, range1.byteOffset, 4)
    arr1.set([1, 2, 3, 4])
    console.log('   ✅ JS array after set:', Array.from(arr1))

    // Unmap (should flush to GPU)
    buffer1.unmap()
    console.log('   ✅ Buffer unmapped')

    // Read back from GPU
    await buffer1.mapAsync('READ')
    const readRange1 = buffer1.getMappedRange()
    const readArr1 = new Float32Array(readRange1.buffer, readRange1.byteOffset, 4)
    console.log('   📖 GPU data:', Array.from(readArr1))

    if (readArr1[0] === 0 && readArr1[1] === 0 && readArr1[2] === 0 && readArr1[3] === 0) {
        console.log('   ❌ BUG CONFIRMED: Data not flushed to GPU!\n')
    } else {
        console.log('   ✅ Data correctly flushed to GPU\n')
    }

    buffer1.unmap()
    buffer1.destroy()

    // Test 2: Using writeMappedRange() workaround (works)
    console.log('📝 Test 2: Using writeMappedRange() workaround')
    const buffer2 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    const data2 = new Float32Array([5, 6, 7, 8])
    buffer2.writeMappedRange(Buffer.from(data2.buffer))
    console.log('   ✅ Written via writeMappedRange:', Array.from(data2))

    buffer2.unmap()
    console.log('   ✅ Buffer unmapped')

    await buffer2.mapAsync('READ')
    const readRange2 = buffer2.getMappedRange()
    const readArr2 = new Float32Array(readRange2.buffer, readRange2.byteOffset, 4)
    console.log('   📖 GPU data:', Array.from(readArr2))

    if (readArr2[0] === 5 && readArr2[1] === 6 && readArr2[2] === 7 && readArr2[3] === 8) {
        console.log('   ✅ Workaround works correctly\n')
    } else {
        console.log('   ❌ Workaround also broken!\n')
    }

    buffer2.unmap()
    buffer2.destroy()

    // Test 3: Different usage flags (MAP_WRITE only allows COPY_SRC)
    console.log('📝 Test 3: Buffer with MAP_WRITE usage')
    const buffer3 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC,
        mappedAtCreation: true
    })

    const range3 = buffer3.getMappedRange()
    const arr3 = new Float32Array(range3.buffer, range3.byteOffset, 4)
    arr3.set([9, 10, 11, 12])
    console.log('   ✅ JS array after set:', Array.from(arr3))

    buffer3.unmap()
    console.log('   ✅ Buffer unmapped')
    console.log('   ⚠️  Cannot read back (no MAP_READ), but data should be in GPU\n')

    buffer3.destroy()

    // Test 4: Map for WRITE, modify, unmap
    console.log('📝 Test 4: mapAsync(WRITE) then modify')
    const buffer4 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    })

    await buffer4.mapAsync('WRITE')
    const range4 = buffer4.getMappedRange()
    const arr4 = new Float32Array(range4.buffer, range4.byteOffset, 4)
    arr4.set([13, 14, 15, 16])
    console.log('   ✅ JS array after set:', Array.from(arr4))

    buffer4.unmap()
    console.log('   ✅ Buffer unmapped')
    console.log('   ⚠️  Cannot read back (MAP_WRITE + COPY_SRC only), but data should be in GPU\n')

    buffer4.destroy()

    console.log('═══════════════════════════════════════════════════════')
    console.log('Summary: The issue is that getMappedRange() returns a COPY')
    console.log('of the GPU data, not a reference to the mapped memory.')
    console.log('Modifying the returned buffer does NOT modify GPU memory.')
    console.log('═══════════════════════════════════════════════════════')
}

reproduceBug().catch(err => {
    console.error('❌ Test failed:', err)
    process.exit(1)
})
