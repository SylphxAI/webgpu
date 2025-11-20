const { Gpu } = require('../webgpu.js')

async function main() {
    console.log('🚀 WebGPU Basic Example\n')

    // Create GPU instance
    const gpu = Gpu()
    console.log('✓ GPU instance created')

    // Request adapter
    console.log('\nRequesting adapter...')
    const adapter = await gpu.requestAdapter({ powerPreference: 'high-performance' })
    console.log('✓ Adapter acquired')

    // Get adapter info
    const info = adapter.info
    console.log('\nAdapter Information:')
    console.log(`  Name: ${info.name}`)
    console.log(`  Vendor: 0x${info.vendor.toString(16)}`)
    console.log(`  Device: 0x${info.device.toString(16)}`)
    console.log(`  Type: ${info.deviceType}`)
    console.log(`  Backend: ${info.backend}`)

    // Get adapter limits (property, not method)
    const limits = adapter.limits
    console.log('\nAdapter Limits:')
    console.log(`  Max Texture 2D: ${limits.maxTextureDimension2d}`)
    console.log(`  Max Bind Groups: ${limits.maxBindGroups}`)
    console.log(`  Max Buffer Size: ${limits.maxBufferSize}`)

    // Get features (property, not method)
    const features = adapter.features
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
