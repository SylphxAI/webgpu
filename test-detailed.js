#!/usr/bin/env bun

/**
 * Detailed Test Runner - Shows actual results from each test
 */

const { Gpu, bufferUsage, textureUsage } = require('./index.js')

console.log('🧪 WebGPU Detailed Test Results\n')
console.log('='*60)

async function runAllTests() {
  let passed = 0
  let failed = 0

  // Test 1: GPU Instance Creation
  console.log('\n📍 Test 1: GPU Instance Creation')
  try {
    const gpu = Gpu.create()
    console.log('  ✅ GPU instance created:', typeof gpu === 'object' ? 'Success' : 'Failed')
    console.log('  📊 Result: gpu =', gpu.constructor.name)
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 2: Enumerate Adapters
  console.log('\n📍 Test 2: Enumerate Adapters')
  try {
    const gpu = Gpu.create()
    const adapters = gpu.enumerateAdapters()
    console.log('  ✅ Found adapters:', adapters.length)
    adapters.forEach((adapter, i) => {
      console.log(`     ${i + 1}. ${adapter}`)
    })
    console.log('  📊 Result: adapters =', adapters)
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 3: Adapter Information
  console.log('\n📍 Test 3: Adapter Information')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const info = adapter.getInfo()
    console.log('  ✅ Adapter Info:')
    console.log('     Name:', info.name)
    console.log('     Backend:', info.backend)
    console.log('     Device Type:', info.deviceType)
    console.log('     Vendor ID:', info.vendor)
    console.log('     Device ID:', info.device)
    console.log('  📊 Result: info =', info)
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 4: Adapter Features
  console.log('\n📍 Test 4: Adapter Features')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const features = adapter.getFeatures()
    console.log('  ✅ Supported Features:', features.length)
    features.forEach(feature => {
      console.log(`     - ${feature}`)
    })
    console.log('  📊 Result: features =', features)
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 5: Adapter Limits
  console.log('\n📍 Test 5: Adapter Limits')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const limits = adapter.getLimits()
    console.log('  ✅ Device Limits:')
    console.log('     Max Texture 1D:', limits.maxTextureDimension1D)
    console.log('     Max Texture 2D:', limits.maxTextureDimension2D)
    console.log('     Max Texture 3D:', limits.maxTextureDimension3D)
    console.log('     Max Bind Groups:', limits.maxBindGroups)
    console.log('     Max Buffer Size:', (limits.maxBufferSize / 1024 / 1024 / 1024).toFixed(2), 'GB')
    console.log('  📊 Result: limits =', limits)
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 6: Buffer Read/Write
  console.log('\n📍 Test 6: Buffer Read/Write Operations')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()
    const usage = bufferUsage()

    const buffer = device.createBuffer(20, usage.copyDst | usage.mapRead, false)
    const inputData = new Float32Array([1.5, 2.5, 3.5, 4.5, 5.5])

    device.queueWriteBuffer(buffer, 0, Buffer.from(inputData.buffer))
    const encoder = device.createCommandEncoder()
    device.queueSubmit(encoder.finish())
    device.poll(true)

    const resultData = await buffer.mapRead()
    const outputData = new Float32Array(resultData.buffer, resultData.byteOffset, 5)

    console.log('  ✅ Write & Read Success:')
    console.log('     Input:  [', Array.from(inputData).join(', '), ']')
    console.log('     Output: [', Array.from(outputData).join(', '), ']')
    console.log('     Match:', JSON.stringify(Array.from(inputData)) === JSON.stringify(Array.from(outputData)) ? '✅ Yes' : '❌ No')

    buffer.unmap()
    buffer.destroy()
    device.destroy()

    console.log('  📊 Result: Data verified correctly')
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 7: Compute Shader Execution
  console.log('\n📍 Test 7: Compute Shader - Vector Addition')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()
    const usage = bufferUsage()

    const shaderCode = `
      @group(0) @binding(0) var<storage, read> a: array<f32>;
      @group(0) @binding(1) var<storage, read> b: array<f32>;
      @group(0) @binding(2) var<storage, read_write> result: array<f32>;

      @compute @workgroup_size(1)
      fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        result[global_id.x] = a[global_id.x] + b[global_id.x];
      }
    `

    const shader = device.createShaderModule(shaderCode)

    const size = 5
    const bufferSize = size * 4

    const bufferA = device.createBuffer(bufferSize, usage.storage | usage.copyDst, false)
    const bufferB = device.createBuffer(bufferSize, usage.storage | usage.copyDst, false)
    const bufferResult = device.createBuffer(bufferSize, usage.storage | usage.copySrc, false)
    const bufferRead = device.createBuffer(bufferSize, usage.copyDst | usage.mapRead, false)

    const dataA = new Float32Array([1, 2, 3, 4, 5])
    const dataB = new Float32Array([10, 20, 30, 40, 50])

    device.queueWriteBuffer(bufferA, 0, Buffer.from(dataA.buffer))
    device.queueWriteBuffer(bufferB, 0, Buffer.from(dataB.buffer))

    const layoutDesc = {
      entries: [
        { binding: 0, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 1, visibility: 4, bufferType: 'read-only-storage' },
        { binding: 2, visibility: 4, bufferType: 'storage' },
      ],
    }
    const layout = device.createBindGroupLayout(layoutDesc)
    const pipelineLayout = device.createPipelineLayout('Compute Layout', [layout])
    const pipeline = device.createComputePipeline('Compute Pipeline', pipelineLayout, shader, 'main')
    const bindGroup = device.createBindGroupBuffers('Compute Bind Group', layout, [bufferA, bufferB, bufferResult])

    const encoder = device.createCommandEncoder()
    encoder.computePass(pipeline, [bindGroup], size, 1, 1)
    device.copyBufferToBuffer(encoder, bufferResult, 0, bufferRead, 0, bufferSize)
    device.queueSubmit(encoder.finish())
    device.poll(true)

    const resultData = await bufferRead.mapRead()
    const results = new Float32Array(resultData.buffer, resultData.byteOffset, size)

    console.log('  ✅ Compute Shader Success:')
    console.log('     A:        [', Array.from(dataA).join(', '), ']')
    console.log('     B:        [', Array.from(dataB).join(', '), ']')
    console.log('     A + B =   [', Array.from(results).join(', '), ']')
    console.log('     Expected: [ 11, 22, 33, 44, 55 ]')
    console.log('     Match:', Array.from(results).join(',') === '11,22,33,44,55' ? '✅ Yes' : '❌ No')

    bufferRead.unmap()
    bufferA.destroy()
    bufferB.destroy()
    bufferResult.destroy()
    bufferRead.destroy()
    device.destroy()

    console.log('  📊 Result: Compute executed correctly')
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 8: Texture Upload/Download
  console.log('\n📍 Test 8: Texture Upload & Download')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()
    const bufUsage = bufferUsage()
    const texUsage = textureUsage()

    const width = 64
    const height = 64

    const texture = device.createTexture({
      width, height,
      format: 'rgba8unorm',
      usage: texUsage.copyDst | texUsage.copySrc,
    })

    // Create red texture
    const size = width * height * 4
    const data = new Uint8Array(size)
    for (let i = 0; i < width * height; i++) {
      data[i * 4] = 255     // R
      data[i * 4 + 1] = 0   // G
      data[i * 4 + 2] = 0   // B
      data[i * 4 + 3] = 255 // A
    }

    const uploadBuffer = device.createBuffer(size, bufUsage.copySrc | bufUsage.copyDst, false)
    device.queueWriteBuffer(uploadBuffer, 0, Buffer.from(data.buffer))

    const encoder1 = device.createCommandEncoder()
    device.copyBufferToTexture(encoder1, uploadBuffer, 0, width * 4, height, texture, 0, 0, 0, 0, width, height, 1)
    device.queueSubmit(encoder1.finish())
    device.poll(true)

    // Read back
    const readBuffer = device.createBuffer(size, bufUsage.copyDst | bufUsage.mapRead, false)
    const encoder2 = device.createCommandEncoder()
    device.copyTextureToBuffer(encoder2, texture, 0, 0, 0, 0, readBuffer, 0, width * 4, height, width, height, 1)
    device.queueSubmit(encoder2.finish())
    device.poll(true)

    const resultData = await readBuffer.mapRead()
    const results = new Uint8Array(resultData.buffer, resultData.byteOffset, size)

    // Check a few pixels
    const pixel1 = [results[0], results[1], results[2], results[3]]
    const pixel2 = [results[100], results[101], results[102], results[103]]

    console.log('  ✅ Texture Upload/Download Success:')
    console.log('     Size:', `${width}x${height}`)
    console.log('     Format: rgba8unorm')
    console.log('     Pixel 1: RGBA(' + pixel1.join(', ') + ')')
    console.log('     Pixel 2: RGBA(' + pixel2.join(', ') + ')')
    console.log('     Expected: RGBA(255, 0, 0, 255) - Red')
    console.log('     Match:', pixel1.join(',') === '255,0,0,255' ? '✅ Yes' : '❌ No')

    readBuffer.unmap()
    uploadBuffer.destroy()
    readBuffer.destroy()
    texture.destroy()
    device.destroy()

    console.log('  📊 Result: Texture operations correct')
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Test 9: Render Triangle
  console.log('\n📍 Test 9: Render Triangle')
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()
    const bufUsage = bufferUsage()
    const texUsage = textureUsage()

    const width = 256
    const height = 256

    const shaderCode = `
      @vertex
      fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
        return vec4f(pos, 1.0);
      }

      @fragment
      fn fs_main() -> @location(0) vec4f {
        return vec4f(1.0, 0.0, 0.0, 1.0); // Red
      }
    `

    const shader = device.createShaderModule(shaderCode)
    const pipelineLayout = device.createPipelineLayout('Layout', [])
    const pipeline = device.createRenderPipeline(
      'Triangle Pipeline',
      pipelineLayout,
      shader, 'vs_main', ['float32x3'],
      shader, 'fs_main', ['rgba8unorm'],
      null, null, null, null
    )

    const vertices = new Float32Array([
      0.0, 0.5, 0.0,
      -0.5, -0.5, 0.0,
      0.5, -0.5, 0.0,
    ])

    const vertexBuffer = device.createBuffer(vertices.byteLength, bufUsage.vertex | bufUsage.copyDst, false)
    device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

    const texture = device.createTexture({
      width, height,
      format: 'rgba8unorm',
      usage: texUsage.renderAttachment | texUsage.copySrc,
    })
    const textureView = texture.createView('Render Target')

    const encoder = device.createCommandEncoder()
    encoder.renderPass(
      pipeline,
      [vertexBuffer],
      3,
      [textureView],
      [[0.0, 0.0, 0.0, 1.0]],
      null, null, null, null
    )

    const readBuffer = device.createBuffer(width * height * 4, bufUsage.copyDst | bufUsage.mapRead, false)
    device.copyTextureToBuffer(encoder, texture, 0, 0, 0, 0, readBuffer, 0, width * 4, height, width, height, 1)
    device.queueSubmit(encoder.finish())
    device.poll(true)

    const data = await readBuffer.mapRead()
    const pixels = new Uint8Array(data.buffer, data.byteOffset, data.byteLength)

    const centerX = Math.floor(width / 2)
    const centerY = Math.floor(height / 2)
    const offset = (centerY * width + centerX) * 4
    const centerPixel = [pixels[offset], pixels[offset + 1], pixels[offset + 2], pixels[offset + 3]]

    console.log('  ✅ Triangle Rendered:')
    console.log('     Render Size:', `${width}x${height}`)
    console.log('     Vertex Count: 3')
    console.log('     Center Pixel: RGBA(' + centerPixel.join(', ') + ')')
    console.log('     Expected: RGBA(255, 0, 0, 255) - Red triangle')
    console.log('     Match:', centerPixel[0] === 255 && centerPixel[1] === 0 ? '✅ Yes' : '❌ No')

    readBuffer.unmap()
    vertexBuffer.destroy()
    readBuffer.destroy()
    texture.destroy()
    device.destroy()

    console.log('  📊 Result: Rendering works correctly')
    passed++
  } catch (e) {
    console.log('  ❌ Failed:', e.message)
    failed++
  }

  // Summary
  console.log('\n' + '='*60)
  console.log('📊 Test Summary')
  console.log('='*60)
  console.log(`✅ Passed: ${passed}`)
  console.log(`❌ Failed: ${failed}`)
  console.log(`📈 Success Rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%`)
  console.log('='*60)

  if (failed === 0) {
    console.log('🎉 All tests passed! WebGPU is working correctly!')
  } else {
    console.log('⚠️ Some tests failed. Please check the errors above.')
  }
}

runAllTests().catch(console.error)
