const { Gpu } = require('../index.js')

async function main() {
    console.log('🚀 WebGPU Basic Example\n')

    // Create GPU instance
    const gpu = Gpu.create()
    console.log('✓ GPU instance created')

    // Enumerate adapters
    const adapters = gpu.enumerateAdapters()
    console.log(`✓ Found ${adapters.length} adapter(s):`)
    adapters.forEach((adapter, i) => {
        console.log(`  ${i + 1}. ${adapter}`)
    })

    // Request adapter
    console.log('\nRequesting adapter...')
    const adapter = await gpu.requestAdapter('high-performance')
    console.log('✓ Adapter acquired')

    // Get adapter info
    const info = adapter.getInfo()
    console.log('\nAdapter Information:')
    console.log(`  Name: ${info.name}`)
    console.log(`  Vendor: 0x${info.vendor.toString(16)}`)
    console.log(`  Device: 0x${info.device.toString(16)}`)
    console.log(`  Type: ${info.deviceType}`)
    console.log(`  Backend: ${info.backend}`)

    // Get adapter limits
    const limits = adapter.getLimits()
    console.log('\nAdapter Limits:')
    console.log(`  Max Texture 2D: ${limits.maxTextureDimension2d}`)
    console.log(`  Max Bind Groups: ${limits.maxBindGroups}`)
    console.log(`  Max Buffer Size: ${limits.maxBufferSize}`)

    // Get features
    const features = adapter.getFeatures()
    console.log(`\nSupported Features: ${features.length}`)
    features.forEach(feature => {
        console.log(`  - ${feature}`)
    })

    // Request device
    console.log('\nRequesting device...')
    const device = await adapter.requestDevice()
    console.log('✓ Device acquired')

    console.log('\n✨ Success! WebGPU is ready to use.')
}

main().catch(err => {
    console.error('❌ Error:', err.message)
    process.exit(1)
})
