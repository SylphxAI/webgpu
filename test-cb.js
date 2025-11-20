const {Gpu} = require('./webgpu.js')
async function test() {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()
    const enc = device.createCommandEncoder()
    const cb = enc.finish()
    console.log('CommandBuffer type:', cb.constructor.name)
    console.log('Submit expects array of these')
    
    try {
        device.queue.submit([cb])
        console.log('✓ Submit succeeded')
    } catch(e) {
        console.log('✗ Submit failed:', e.message)
    }
}
test()
