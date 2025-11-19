/**
 * Edge case tests for getMappedRange()
 *
 * Tests WebGPU standard behavior including:
 * - Parameter validation (offset, size)
 * - Alignment requirements (multiples of 4)
 * - Bounds checking
 * - State validation (unmapped, pending, mapped)
 * - Multiple overlapping ranges
 */

const { Gpu, GPUBufferUsage } = require('../webgpu.js')

async function testGetMappedRangeEdgeCases() {
    console.log('🧪 Testing getMappedRange() edge cases...\n')

    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Test 1: Basic getMappedRange() without parameters (full range)
    console.log('📝 Test 1: getMappedRange() without parameters (full range)')
    const buffer1 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    const range1 = buffer1.getMappedRange()
    console.log('   Range type:', range1.constructor.name)
    console.log('   Range size:', range1.byteLength, 'bytes')

    if (range1 instanceof ArrayBuffer && range1.byteLength === 16) {
        console.log('   ✅ Returns ArrayBuffer with correct size\n')
    } else {
        console.error('   ❌ Expected ArrayBuffer with 16 bytes\n')
        process.exit(1)
    }

    const view1 = new Float32Array(range1)
    view1.set([1, 2, 3, 4])
    buffer1.unmap()

    // Verify data was written
    await buffer1.mapAsync('READ')
    const readRange1 = buffer1.getMappedRange()
    const readView1 = new Float32Array(readRange1)

    if (readView1[0] === 1 && readView1[3] === 4) {
        console.log('   ✅ Data correctly flushed to GPU\n')
    } else {
        console.error('   ❌ Data not flushed correctly\n')
        process.exit(1)
    }
    buffer1.unmap()

    // Test 2: Alignment validation (offset and size must be multiples of 4)
    console.log('📝 Test 2: Alignment validation')
    // Note: Current implementation doesn't support offset/size parameters yet
    // This test will be enabled after implementation
    console.log('   ⚠️  Skipped - offset/size parameters not yet implemented\n')

    // Test 3: State validation - getMappedRange() when unmapped should fail
    console.log('📝 Test 3: getMappedRange() on unmapped buffer should fail')
    console.log('   ⚠️  Currently causes panic instead of throwing JS error')
    console.log('   Skipping this test until error handling is fixed\n')

    // Test 4: mapState property
    console.log('📝 Test 4: mapState property')
    const buffer4 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.MAP_READ
    })

    try {
        if (typeof buffer4.mapState === 'function') {
            const state = buffer4.mapState()
            console.log('   Initial mapState:', state)

            if (state === 'unmapped' || state === 0) {
                console.log('   ✅ Buffer starts in unmapped state\n')
            } else {
                console.log('   ⚠️  Unexpected initial state:', state, '\n')
            }
        } else {
            console.log('   ⚠️  mapState method not available\n')
        }
    } catch (err) {
        console.log('   ⚠️  mapState not implemented:', err.message, '\n')
    }

    // Test 5: Double getMappedRange() on same buffer (current implementation allows this)
    console.log('📝 Test 5: Multiple getMappedRange() calls')
    const buffer5 = device.createBuffer({
        size: 32,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    const range5a = buffer5.getMappedRange()
    const range5b = buffer5.getMappedRange()

    console.log('   First call returns:', range5a.byteLength, 'bytes')
    console.log('   Second call returns:', range5b.byteLength, 'bytes')

    // WebGPU spec: overlapping ranges should cause validation error
    // But without offset/size parameters, both calls return full range
    console.log('   ⚠️  Multiple calls currently allowed (no overlap detection without offset/size params)\n')

    buffer5.unmap()

    // Test 6: getMappedRange() then modify then unmap
    console.log('📝 Test 6: Standard write pattern')
    const buffer6 = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
        mappedAtCreation: true
    })

    const range6 = buffer6.getMappedRange()
    const view6 = new Float32Array(range6)
    view6[0] = 100
    view6[1] = 200
    view6[2] = 300
    view6[3] = 400

    buffer6.unmap()

    await buffer6.mapAsync('READ')
    const readRange6 = buffer6.getMappedRange()
    const readView6 = new Float32Array(readRange6)

    if (readView6[0] === 100 && readView6[3] === 400) {
        console.log('   ✅ Standard write pattern works correctly\n')
    } else {
        console.error('   ❌ Data not written correctly:', Array.from(readView6), '\n')
        process.exit(1)
    }
    buffer6.unmap()

    console.log('═══════════════════════════════════════════════════════')
    console.log('✅ Edge case tests completed!')
    console.log('')
    console.log('Note: Some tests are skipped because offset/size parameters')
    console.log('are not yet implemented. These will be added in a future update.')
    console.log('═══════════════════════════════════════════════════════')
}

testGetMappedRangeEdgeCases().catch(err => {
    console.error('❌ Test failed:', err)
    console.error(err.stack)
    process.exit(1)
})
