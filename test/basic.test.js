const assert = require('assert')

console.log('🧪 Running WebGPU Tests\n')

// Test 1: Module loads
console.log('Test 1: Module loads correctly')
try {
    const webgpu = require('../index.js')
    assert(webgpu.Gpu, 'Gpu class should be exported')
    assert(typeof webgpu.Gpu.create === 'function', 'Gpu.create should be a function')
    console.log('  ✓ Module exports Gpu class\n')
} catch (err) {
    console.error('  ❌ Failed:', err.message)
    process.exit(1)
}

// Test 2: GPU instance creation
console.log('Test 2: Create GPU instance')
try {
    const { Gpu } = require('../index.js')
    const gpu = Gpu.create()
    assert(gpu, 'GPU instance should be created')
    console.log('  ✓ GPU instance created successfully\n')
} catch (err) {
    console.error('  ❌ Failed:', err.message)
    process.exit(1)
}

// Test 3: Enumerate adapters
console.log('Test 3: Enumerate adapters')
;(async () => {
    try {
        const { Gpu } = require('../index.js')
        const gpu = Gpu.create()
        const adapters = gpu.enumerateAdapters()
        assert(Array.isArray(adapters), 'Should return an array')
        console.log(`  ✓ Found ${adapters.length} adapter(s)\n`)
    } catch (err) {
        console.error('  ❌ Failed:', err.message)
        process.exit(1)
    }

    // Test 4: Request adapter
    console.log('Test 4: Request adapter')
    try {
        const { Gpu } = require('../index.js')
        const gpu = Gpu.create()
        const adapter = await gpu.requestAdapter()
        assert(adapter, 'Adapter should be returned')
        console.log('  ✓ Adapter acquired\n')
    } catch (err) {
        console.error('  ❌ Failed:', err.message)
        process.exit(1)
    }

    // Test 5: Get adapter info
    console.log('Test 5: Get adapter information')
    try {
        const { Gpu } = require('../index.js')
        const gpu = Gpu.create()
        const adapter = await gpu.requestAdapter()
        const info = adapter.getInfo()
        assert(info.name, 'Adapter should have a name')
        assert(typeof info.vendor === 'number', 'Vendor should be a number')
        assert(info.backend, 'Backend should be specified')
        console.log(`  ✓ Adapter: ${info.name} (${info.backend})\n`)
    } catch (err) {
        console.error('  ❌ Failed:', err.message)
        process.exit(1)
    }

    // Test 6: Request device
    console.log('Test 6: Request device')
    try {
        const { Gpu } = require('../index.js')
        const gpu = Gpu.create()
        const adapter = await gpu.requestAdapter()
        const device = await adapter.requestDevice()
        assert(device, 'Device should be returned')
        console.log('  ✓ Device acquired\n')
    } catch (err) {
        console.error('  ❌ Failed:', err.message)
        process.exit(1)
    }

    // Test 7: Create buffer
    console.log('Test 7: Create buffer')
    try {
        const { Gpu, bufferUsage } = require('../index.js')
        const gpu = Gpu.create()
        const adapter = await gpu.requestAdapter()
        const device = await adapter.requestDevice()
        const buffer = device.createBuffer(256, bufferUsage.UNIFORM, false)
        assert(buffer, 'Buffer should be created')
        assert(buffer.size() === 256, 'Buffer size should match')
        buffer.destroy()
        console.log('  ✓ Buffer created and destroyed\n')
    } catch (err) {
        console.error('  ❌ Failed:', err.message)
        process.exit(1)
    }

    console.log('✨ All tests passed!')
})()
