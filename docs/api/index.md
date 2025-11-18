# API Reference

Complete API reference for @sylphx/webgpu.

## Overview

The API follows the [WebGPU specification](https://www.w3.org/TR/webgpu/) with adaptations for Node.js/Bun environments.

## Quick Links

- [GPU](/api/gpu) - GPU instance and adapter enumeration
- [Adapter](/api/adapter) - GPU adapter info and device creation
- [Device](/api/device) - Main GPU interface
- [Buffer](/api/buffer) - GPU memory management
- [Texture](/api/texture) - Texture and sampler operations
- [Pipeline](/api/pipeline) - Render and compute pipelines
- [Command Encoder](/api/command-encoder) - Command recording

## Type Definitions

Full TypeScript definitions are provided:

```typescript
import {
  Gpu,
  type AdapterInfo,
  type AdapterLimits,
  type BufferUsage,
  type TextureUsage,
  // ... more types
} from '@sylphx/webgpu'
```

## Constants

### Buffer Usage

```javascript
const { bufferUsage } = require('@sylphx/webgpu')
const usage = bufferUsage()

usage.copySrc       // 0x0004
usage.copyDst       // 0x0008
usage.storage       // 0x0080
usage.uniform       // 0x0040
usage.vertex        // 0x0020
usage.index         // 0x0010
usage.mapRead       // 0x0001
usage.mapWrite      // 0x0002
usage.indirect      // 0x0100
usage.queryResolve  // 0x0200
```

### Texture Usage

```javascript
const { textureUsage } = require('@sylphx/webgpu')
const usage = textureUsage()

usage.copySrc          // 0x01
usage.copyDst          // 0x02
usage.textureBinding   // 0x04
usage.storageBinding   // 0x08
usage.renderAttachment // 0x10
```

## Common Patterns

### Basic Setup

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()
```

### Buffer Operations

```javascript
const { bufferUsage } = require('@sylphx/webgpu')
const usage = bufferUsage()

// Create buffer
const buffer = device.createBuffer(256, usage.copyDst | usage.mapRead, false)

// Write data
const data = new Float32Array([1, 2, 3, 4])
device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))
device.poll(true)

// Read data
const result = await buffer.mapRead()
const floats = new Float32Array(result.buffer, result.byteOffset, 4)
buffer.unmap()

// Cleanup
buffer.destroy()
```

### Compute Shader

```javascript
// Create shader
const shader = device.createShaderModule(\`
  @compute @workgroup_size(64)
  fn main() { /* shader code */ }
\`)

// Create pipeline
const layout = device.createBindGroupLayout({ entries: [...] })
const pipelineLayout = device.createPipelineLayout('Layout', [layout])
const pipeline = device.createComputePipeline('Pipeline', pipelineLayout, shader, 'main')

// Execute
const encoder = device.createCommandEncoder()
encoder.computePass(pipeline, [bindGroup], workgroupsX, workgroupsY, workgroupsZ)
device.queueSubmit(encoder.finish())
device.poll(true)
```

### Rendering

```javascript
// Create pipeline
const pipeline = device.createRenderPipeline(
  'Pipeline',
  pipelineLayout,
  vertexShader, 'vs_main', ['float32x3'],
  fragmentShader, 'fs_main', ['rgba8unorm'],
  null, // no depth
  'alpha', // blend mode
  null, // all channels
  1 // no MSAA
)

// Render
const encoder = device.createCommandEncoder()
encoder.renderPass(
  pipeline,
  [vertexBuffer],
  vertexCount,
  [textureView],
  [[0, 0, 0, 1]], // clear color
  null, // bind groups
  null, // depth
  null, // clear depth
  null  // resolve targets
)
device.queueSubmit(encoder.finish())
device.poll(true)
```

## Error Handling

All async operations can throw errors:

```javascript
try {
  const adapter = await gpu.requestAdapter()
  if (!adapter) {
    throw new Error('No GPU found')
  }
  // ... rest of code
} catch (error) {
  console.error('GPU Error:', error.message)
}
```

## Resource Management

Always destroy resources when done:

```javascript
// Manual cleanup
buffer.destroy()
texture.destroy()
device.destroy()

// Or use try/finally
let device, buffer, texture
try {
  device = await adapter.requestDevice()
  buffer = device.createBuffer(...)
  texture = device.createTexture(...)
  // ... use resources
} finally {
  buffer?.destroy()
  texture?.destroy()
  device?.destroy()
}
```

## Next Steps

- Explore detailed [GPU](/api/gpu) API
- Learn about [Buffers](/api/buffer)
- See [Examples](/examples/)
