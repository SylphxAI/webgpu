# API Reference

Complete API reference for `@sylphx/webgpu` v1.0 - 100% WebGPU Standard Compliant.

## Overview

This library provides WebGPU for Node.js and Bun via wgpu-rs bindings. It follows the [WebGPU specification](https://www.w3.org/TR/webgpu/) with adaptations for server-side environments.

## Quick Links

- [GPU](/api/gpu) - GPU instance and adapter enumeration
- [Adapter](/api/adapter) - GPU adapter info, features, and limits
- [Device](/api/device) - Main GPU interface for creating resources
- [Buffer](/api/buffer) - GPU memory management and mapping
- [Texture](/api/texture) - Texture and sampler operations
- [Pipeline](/api/pipeline) - Compute and render pipelines
- [Command Encoder](/api/command-encoder) - Recording GPU commands

## Installation

```bash
npm install @sylphx/webgpu
```

## Quick Start

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

async function main() {
  // Initialize WebGPU
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  // Create a buffer
  const buffer = device.createBuffer({
    label: 'My Buffer',
    size: 256,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  // Write data
  const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
  device.queue.writeBuffer(buffer, 0, Buffer.from(data.buffer))
  device.poll(true)

  // Cleanup
  buffer.destroy()
  device.destroy()

  console.log('✅ Done!')
}

main()
```

## Core Concepts

### GPU Instance

The entry point for all WebGPU operations:

```javascript
const gpu = Gpu.create()
const adapter = await gpu.requestAdapter({
  powerPreference: 'high-performance'
})
```

### Adapter

Represents a physical GPU and provides capability information:

```javascript
const info = adapter.getInfo()
console.log('GPU:', info.name)
console.log('Backend:', info.backend)

const limits = adapter.getLimits()
console.log('Max buffer size:', limits.maxBufferSize)

const features = adapter.getFeatures()
console.log('Features:', features)
```

### Device

The main interface for GPU operations:

```javascript
const device = await adapter.requestDevice()

// Create resources
const buffer = device.createBuffer({ size: 1024, usage: GPUBufferUsage.STORAGE })
const texture = device.createTexture({ width: 512, height: 512, format: 'rgba8unorm', usage: GPUTextureUsage.TEXTURE_BINDING })
const shader = device.createShaderModule({ code: wgslCode })
const pipeline = device.createComputePipeline({ entryPoint: 'main' }, null, shader)

// Submit work
device.queue.submit(commandBuffer)
device.poll(true)
```

## Usage Flags

### GPUBufferUsage

Combine with bitwise OR (`|`):

```javascript
const { GPUBufferUsage } = require('@sylphx/webgpu')

GPUBufferUsage.MAP_READ       // 0x0001 - Can be mapped for reading
GPUBufferUsage.MAP_WRITE      // 0x0002 - Can be mapped for writing
GPUBufferUsage.COPY_SRC       // 0x0004 - Can be copied from
GPUBufferUsage.COPY_DST       // 0x0008 - Can be copied to
GPUBufferUsage.INDEX          // 0x0010 - Can be used as index buffer
GPUBufferUsage.VERTEX         // 0x0020 - Can be used as vertex buffer
GPUBufferUsage.UNIFORM        // 0x0040 - Can be used as uniform buffer
GPUBufferUsage.STORAGE        // 0x0080 - Can be used as storage buffer
GPUBufferUsage.INDIRECT       // 0x0100 - Can be used for indirect commands
GPUBufferUsage.QUERY_RESOLVE  // 0x0200 - Can be used for query results
```

**Example:**
```javascript
const buffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})
```

### GPUTextureUsage

```javascript
const { GPUTextureUsage } = require('@sylphx/webgpu')

GPUTextureUsage.COPY_SRC          // 0x01 - Can be copied from
GPUTextureUsage.COPY_DST          // 0x02 - Can be copied to
GPUTextureUsage.TEXTURE_BINDING   // 0x04 - Can be sampled in shaders
GPUTextureUsage.STORAGE_BINDING   // 0x08 - Can be used as storage texture
GPUTextureUsage.RENDER_ATTACHMENT // 0x10 - Can be rendered to
```

**Example:**
```javascript
const texture = device.createTexture({
  width: 800,
  height: 600,
  format: 'rgba8unorm',
  usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC
})
```

## Common Patterns

### Basic Setup

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()
```

### Creating and Writing Buffers

```javascript
const { GPUBufferUsage } = require('@sylphx/webgpu')

// Method 1: mappedAtCreation (initial data)
const buffer1 = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE,
  mappedAtCreation: true
})

const range = buffer1.getMappedRange()
const view = new Float32Array(range)
view.set([1.0, 2.0, 3.0, 4.0])
buffer1.unmap()  // Ready for GPU

// Method 2: queue.writeBuffer (COPY_DST required)
const buffer2 = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queue.writeBuffer(buffer2, 0, Buffer.from(data.buffer))

// Method 3: mapAsync (for updating)
const buffer3 = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.MAP_WRITE
})

await buffer3.mapAsync('WRITE')
const range3 = buffer3.getMappedRange()
const view3 = new Float32Array(range3)
view3.set([1.0, 2.0, 3.0, 4.0])
buffer3.unmap()
```

### Reading Buffers

```javascript
const buffer = device.createBuffer({
  size: 256,
  usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ
})

// Write data first
device.queue.writeBuffer(buffer, 0, data)
device.poll(true)

// Read back
await buffer.mapAsync('READ')
const range = buffer.getMappedRange()
const result = new Float32Array(range)
console.log('Data:', Array.from(result))
buffer.unmap()
```

### Compute Shader

```javascript
// Create shader
const shader = device.createShaderModule({
  label: 'Compute Shader',
  code: `
    @group(0) @binding(0) var<storage, read_write> data: array<f32>;

    @compute @workgroup_size(64)
    fn main(@builtin(global_invocation_id) id: vec3<u32>) {
      data[id.x] = data[id.x] * 2.0;
    }
  `
})

// Create pipeline
const pipeline = device.createComputePipeline(
  { label: 'Multiply', entryPoint: 'main' },
  null,  // Auto-layout
  shader
)

// Create bind group
const bindGroup = device.createBindGroup(
  { label: 'Bindings' },
  pipeline.getBindGroupLayout(0),
  [{ binding: 0, resourceType: 'buffer' }],
  [buffer],
  null,
  null
)

// Execute
const encoder = device.createCommandEncoder()
const pass = encoder.beginComputePass()
pass.setPipeline(pipeline)
pass.setBindGroup(0, bindGroup)
pass.dispatchWorkgroups(Math.ceil(256 / 64))
pass.end()

device.queue.submit(encoder.finish())
device.poll(true)
```

### Rendering

```javascript
// Create shader
const shader = device.createShaderModule({
  code: `
    @vertex
    fn vs_main(@location(0) pos: vec2<f32>) -> @builtin(position) vec4<f32> {
      return vec4<f32>(pos, 0.0, 1.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4<f32> {
      return vec4<f32>(1.0, 0.0, 0.0, 1.0);  // Red
    }
  `
})

// Create pipeline
const pipeline = device.createRenderPipeline(
  {
    vertex: {
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 8,
        attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' }]
      }]
    },
    fragment: {
      entryPoint: 'fs_main',
      targets: [{ format: 'rgba8unorm' }]
    },
    primitive: { topology: 'triangle-list' },
    multisample: { count: 1 }
  },
  null,
  shader,
  shader
)

// Create render target
const texture = device.createTexture({
  width: 800,
  height: 600,
  format: 'rgba8unorm',
  usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC
})

// Render
const encoder = device.createCommandEncoder()
const pass = encoder.beginRenderPass(
  {
    colorAttachments: [{
      view: texture.createView(),
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0, g: 0, b: 0, a: 1 }
    }]
  },
  [texture.createView()],
  null,
  null
)

pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.draw(3)
pass.end()

device.queue.submit(encoder.finish())
device.poll(true)
```

## Error Handling

All async operations can throw errors:

```javascript
try {
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()

  if (!adapter) {
    throw new Error('No GPU adapter found')
  }

  const device = await adapter.requestDevice()

  // GPU operations

} catch (error) {
  console.error('WebGPU Error:', error.message)
  // Fallback to CPU or show error
}
```

### GPU Error Scopes

Use error scopes for capturing validation errors:

```javascript
device.pushErrorScope('validation')

// Operations that might fail
const buffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE
})

const error = await device.popErrorScope()
if (error) {
  console.error('Validation error:', error)
}
```

## Resource Management

**Always destroy resources when done:**

```javascript
// Manual cleanup
buffer.destroy()
texture.destroy()
device.destroy()

// Or use try/finally
let device, buffer, texture
try {
  device = await adapter.requestDevice()
  buffer = device.createBuffer({ size: 1024, usage: GPUBufferUsage.STORAGE })
  texture = device.createTexture({ width: 512, height: 512, format: 'rgba8unorm', usage: GPUTextureUsage.TEXTURE_BINDING })

  // Use resources

} finally {
  buffer?.destroy()
  texture?.destroy()
  device?.destroy()
}
```

## TypeScript

Full TypeScript definitions are provided:

```typescript
import {
  Gpu,
  type GPUAdapter,
  type GPUDevice,
  type GPUBuffer,
  type GPUTexture,
  type GPUShaderModule,
  GPUBufferUsage,
  GPUTextureUsage,
} from '@sylphx/webgpu'

const gpu: Gpu = Gpu.create()
const adapter: GPUAdapter = await gpu.requestAdapter()
const device: GPUDevice = await adapter.requestDevice()

const buffer: GPUBuffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

const texture: GPUTexture = device.createTexture({
  width: 512,
  height: 512,
  format: 'rgba8unorm',
  usage: GPUTextureUsage.TEXTURE_BINDING
})

const shader: GPUShaderModule = device.createShaderModule({
  code: wgslCode
})
```

## Platform Support

- **macOS**: Metal backend
- **Windows**: DirectX 12 or Vulkan backend
- **Linux**: Vulkan backend

## Node.js / Bun Compatibility

This library works with both Node.js (≥18.0.0) and Bun runtime:

```javascript
// Node.js
const { Gpu } = require('@sylphx/webgpu')

// Bun
import { Gpu } from '@sylphx/webgpu'
```

## Complete Example

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

async function vectorAdd() {
  // Setup
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  // Shader
  const shader = device.createShaderModule({
    code: `
      @group(0) @binding(0) var<storage, read> a: array<f32>;
      @group(0) @binding(1) var<storage, read> b: array<f32>;
      @group(0) @binding(2) var<storage, read_write> result: array<f32>;

      @compute @workgroup_size(64)
      fn main(@builtin(global_invocation_id) id: vec3<u32>) {
        let i = id.x;
        if (i >= arrayLength(&result)) { return; }
        result[i] = a[i] + b[i];
      }
    `
  })

  // Pipeline
  const pipeline = device.createComputePipeline(
    { entryPoint: 'main' },
    null,
    shader
  )

  // Buffers
  const size = 256
  const bufferA = device.createBuffer({
    size,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
  })
  const bufferB = device.createBuffer({
    size,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
  })
  const bufferResult = device.createBuffer({
    size,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST
  })

  // Write input
  const a = new Float32Array(64).fill(1.0)
  const b = new Float32Array(64).fill(2.0)
  device.queue.writeBuffer(bufferA, 0, Buffer.from(a.buffer))
  device.queue.writeBuffer(bufferB, 0, Buffer.from(b.buffer))

  // Bind group
  const bindGroup = device.createBindGroup(
    {},
    pipeline.getBindGroupLayout(0),
    [
      { binding: 0, resourceType: 'buffer' },
      { binding: 1, resourceType: 'buffer' },
      { binding: 2, resourceType: 'buffer' }
    ],
    [bufferA, bufferB, bufferResult],
    null,
    null
  )

  // Execute
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginComputePass()
  pass.setPipeline(pipeline)
  pass.setBindGroup(0, bindGroup)
  pass.dispatchWorkgroups(1)
  pass.end()

  device.queue.submit(encoder.finish())
  device.poll(true)

  // Read result
  const stagingBuffer = device.createBuffer({
    size,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ
  })

  const copyEncoder = device.createCommandEncoder()
  copyEncoder.copyBufferToBuffer(bufferResult, 0, stagingBuffer, 0, size)
  device.queue.submit(copyEncoder.finish())
  device.poll(true)

  await stagingBuffer.mapAsync('READ')
  const result = new Float32Array(stagingBuffer.getMappedRange())
  console.log('Result:', Array.from(result.slice(0, 10)))
  stagingBuffer.unmap()

  // Cleanup
  bufferA.destroy()
  bufferB.destroy()
  bufferResult.destroy()
  stagingBuffer.destroy()
  device.destroy()

  console.log('✅ Vector addition complete!')
}

vectorAdd().catch(console.error)
```

## Best Practices

### 1. Use Labels for Debugging

```javascript
const buffer = device.createBuffer({
  label: 'Particle Positions',  // Descriptive label
  size: 1024,
  usage: GPUBufferUsage.STORAGE
})
```

### 2. Minimize Buffer Usage Flags

Only specify necessary flags to optimize memory usage:

```javascript
// ✅ Good
const buffer = device.createBuffer({
  size: 256,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

// ❌ Bad: Unnecessary flags
const buffer = device.createBuffer({
  size: 256,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC | GPUBufferUsage.MAP_READ
})
```

### 3. Poll After Submit

Always poll after submitting work to ensure completion:

```javascript
device.queue.submit(commandBuffer)
device.poll(true)  // Wait for completion
```

### 4. Clean Up Resources

Always destroy resources to avoid memory leaks:

```javascript
buffer.destroy()
texture.destroy()
device.destroy()
```

## Next Steps

- **[GPU API](/api/gpu)** - Initialize WebGPU and request adapters
- **[Device API](/api/device)** - Create resources and submit work
- **[Buffer API](/api/buffer)** - GPU memory and mapping operations
- **[Compute Guide](/guide/compute)** - Compute shader tutorial
- **[Rendering Guide](/guide/rendering)** - Rendering tutorial
- **[Examples](/examples/)** - Complete working examples

## Standards Compliance

This library implements WebGPU v1.0 with **100% standard compliance**:

- ✅ Standard buffer mapping with overlapping range detection
- ✅ Standard texture operations
- ✅ Standard pipeline creation
- ✅ Standard error handling
- ✅ No non-standard APIs

See the [WebGPU Specification](https://www.w3.org/TR/webgpu/) for full API details.
