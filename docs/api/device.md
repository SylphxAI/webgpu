# GPUDevice

The GPUDevice is the main interface for GPU operations. It creates resources (buffers, textures, shaders, pipelines) and provides access to the command queue.

## Overview

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()

// Now you can create resources and execute GPU commands
```

## Properties

### `device.queue`

The queue for submitting commands and writing data.

**Type:** `GPUQueue`

**Example:**
```javascript
const queue = device.queue

// Submit commands
queue.submit(commandBuffer)

// Write data directly
queue.writeBuffer(buffer, 0, data)
```

### `device.features`

The features supported by this device.

**Type:** `GPUSupportedFeatures`

**Example:**
```javascript
const features = device.features
console.log('Supported features:', features)
```

### `device.limits`

The limits supported by this device.

**Type:** `GPUSupportedLimits`

**Example:**
```javascript
const limits = device.limits
console.log('Max buffer size:', limits.maxBufferSize)
console.log('Max texture 2D:', limits.maxTextureDimension2D)
```

### `device.label`

The label of this device (may be `null`).

**Type:** `String | null`

**Example:**
```javascript
console.log('Device label:', device.label)
```

## Resource Creation

### `device.createBuffer(descriptor)`

Creates a GPU buffer for storing data.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `size` (Number): Buffer size in bytes (must be multiple of 4)
  - `usage` (Number): Usage flags (bitwise OR of `GPUBufferUsage` flags)
  - `mappedAtCreation` (Boolean, optional): Whether to map buffer on creation (default: `false`)

**Returns:** `GPUBuffer`

**Example:**
```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

const buffer = device.createBuffer({
  label: 'Storage Buffer',
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
  mappedAtCreation: false
})
```

### `device.createTexture(descriptor)`

Creates a GPU texture for storing image data.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `width` (Number): Texture width in pixels
  - `height` (Number): Texture height in pixels
  - `depth` (Number, optional): Depth or array layers (default: `1`)
  - `format` (String): Texture format (e.g., `'rgba8unorm'`, `'bgra8unorm'`)
  - `usage` (Number): Usage flags (bitwise OR of `GPUTextureUsage` flags)
  - `dimension` (String, optional): `'1d'`, `'2d'`, or `'3d'` (default: `'2d'`)
  - `mipLevelCount` (Number, optional): Number of mip levels (default: `1`)
  - `sampleCount` (Number, optional): Sample count for MSAA (default: `1`)

**Returns:** `GPUTexture`

**Example:**
```javascript
const { GPUTextureUsage } = require('@sylphx/webgpu')

const texture = device.createTexture({
  label: 'Render Target',
  width: 800,
  height: 600,
  depth: 1,
  format: 'rgba8unorm',
  usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})
```

### `device.createSampler(descriptor)`

Creates a texture sampler.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `addressModeU` (String, optional): `'repeat'`, `'clamp-to-edge'`, `'mirror-repeat'` (default: `'clamp-to-edge'`)
  - `addressModeV` (String, optional): Same as U
  - `addressModeW` (String, optional): Same as U
  - `magFilter` (String, optional): `'nearest'` or `'linear'` (default: `'nearest'`)
  - `minFilter` (String, optional): `'nearest'` or `'linear'` (default: `'nearest'`)
  - `mipmapFilter` (String, optional): `'nearest'` or `'linear'` (default: `'nearest'`)
  - `lodMinClamp` (Number, optional): Minimum LOD (default: `0`)
  - `lodMaxClamp` (Number, optional): Maximum LOD (default: `32`)
  - `compare` (String, optional): Compare function for depth textures
  - `maxAnisotropy` (Number, optional): Anisotropy level 1-16 (default: `1`)

**Returns:** `GPUSampler`

**Example:**
```javascript
const sampler = device.createSampler({
  label: 'Linear Sampler',
  addressModeU: 'repeat',
  addressModeV: 'repeat',
  magFilter: 'linear',
  minFilter: 'linear',
  mipmapFilter: 'linear',
  maxAnisotropy: 1
})
```

### `device.createShaderModule(descriptor)`

Creates a shader module from WGSL code.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `code` (String): WGSL shader source code

**Returns:** `GPUShaderModule`

**Example:**
```javascript
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
```

### `device.createComputePipeline(descriptor, layout, module)`

Creates a compute pipeline.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `entryPoint` (String): Shader entry point function name
- `layout` (GPUPipelineLayout | null): Pipeline layout (or `null` for auto-layout)
- `module` (GPUShaderModule): Shader module

**Returns:** `GPUComputePipeline`

**Example:**
```javascript
const pipeline = device.createComputePipeline(
  {
    label: 'Vector Add',
    entryPoint: 'main'
  },
  null,  // Use auto-layout
  shader
)
```

### `device.createRenderPipeline(descriptor, layout, vertexModule, fragmentModule)`

Creates a render pipeline.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `vertex` (Object):
    - `entryPoint` (String): Vertex shader entry point
    - `buffers` (Array, optional): Vertex buffer layouts
  - `primitive` (Object, optional):
    - `topology` (String, optional): Primitive topology (default: `'triangle-list'`)
    - `frontFace` (String, optional): `'ccw'` or `'cw'` (default: `'ccw'`)
    - `cullMode` (String, optional): `'none'`, `'front'`, or `'back'` (default: `'none'`)
  - `depthStencil` (Object, optional):
    - `format` (String): Depth/stencil format
    - `depthWriteEnabled` (Boolean, optional): Enable depth writes (default: `true`)
    - `depthCompare` (String, optional): Depth compare function (default: `'less'`)
  - `multisample` (Object, optional):
    - `count` (Number, optional): Sample count (default: `1`)
  - `fragment` (Object, optional):
    - `entryPoint` (String): Fragment shader entry point
    - `targets` (Array): Color target configurations
- `layout` (GPUPipelineLayout | null): Pipeline layout (or `null` for auto-layout)
- `vertexModule` (GPUShaderModule): Vertex shader module
- `fragmentModule` (GPUShaderModule | null): Fragment shader module (or `null` if no fragment shader)

**Returns:** `GPURenderPipeline`

**Example:**
```javascript
const pipeline = device.createRenderPipeline(
  {
    label: 'Triangle Pipeline',
    vertex: {
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 12,  // 3 floats * 4 bytes
        attributes: [{
          shaderLocation: 0,
          offset: 0,
          format: 'float32x3'
        }]
      }]
    },
    fragment: {
      entryPoint: 'fs_main',
      targets: [{
        format: 'rgba8unorm'
      }]
    },
    primitive: {
      topology: 'triangle-list',
      frontFace: 'ccw',
      cullMode: 'none'
    },
    multisample: {
      count: 1
    }
  },
  null,  // Use auto-layout
  shader,
  shader  // Same shader for both vertex and fragment
)
```

### `device.createBindGroupLayout(descriptor)`

Creates a bind group layout.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `entries` (Array): Layout entries

**Returns:** `GPUBindGroupLayout`

**Example:**
```javascript
const layout = device.createBindGroupLayout({
  label: 'Bind Group Layout',
  entries: [
    {
      binding: 0,
      visibility: GPUShaderStage.COMPUTE,
      buffer: { type: 'storage' }
    },
    {
      binding: 1,
      visibility: GPUShaderStage.COMPUTE,
      buffer: { type: 'storage' }
    }
  ]
})
```

### `device.createBindGroup(descriptor, layout, entries, buffers, textures, samplers)`

Creates a bind group.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
- `layout` (GPUBindGroupLayout): Bind group layout
- `entries` (Array): Binding entries with `binding` and `resourceType`
- `buffers` (Array<GPUBuffer>, optional): Buffers referenced by entries
- `textures` (Array<GPUTextureView>, optional): Texture views referenced by entries
- `samplers` (Array<GPUSampler>, optional): Samplers referenced by entries

**Returns:** `GPUBindGroup`

**Example:**
```javascript
const bindGroup = device.createBindGroup(
  { label: 'Bind Group' },
  layout,
  [
    { binding: 0, resourceType: 'buffer' },
    { binding: 1, resourceType: 'buffer' }
  ],
  [bufferA, bufferB],  // Buffers
  null,  // No textures
  null   // No samplers
)
```

### `device.createPipelineLayout(descriptor, bindGroupLayouts)`

Creates a pipeline layout.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
- `bindGroupLayouts` (Array<GPUBindGroupLayout>): Bind group layouts

**Returns:** `GPUPipelineLayout`

**Example:**
```javascript
const pipelineLayout = device.createPipelineLayout(
  { label: 'Pipeline Layout' },
  [bindGroupLayout]
)
```

### `device.createCommandEncoder(descriptor)`

Creates a command encoder for recording GPU commands.

**Parameters:**
- `descriptor` (Object, optional):
  - `label` (String, optional): Debug label

**Returns:** `GPUCommandEncoder`

**Example:**
```javascript
const encoder = device.createCommandEncoder({
  label: 'My Commands'
})
```

### `device.createQuerySet(descriptor)`

Creates a query set for timestamp or occlusion queries.

**Parameters:**
- `descriptor` (Object):
  - `label` (String, optional): Debug label
  - `type` (String): `'timestamp'` or `'occlusion'`
  - `count` (Number): Number of queries

**Returns:** `GPUQuerySet`

**Example:**
```javascript
const querySet = device.createQuerySet({
  label: 'Timestamp Queries',
  type: 'timestamp',
  count: 2
})
```

## Queue Operations

The `device.queue` property provides access to the GPU queue for submitting commands and writing data.

### `queue.submit(commandBuffer)`

Submits a command buffer to the GPU for execution.

**Parameters:**
- `commandBuffer` (GPUCommandBuffer): Command buffer to submit

**Example:**
```javascript
const encoder = device.createCommandEncoder()
// ... encode commands
const commandBuffer = encoder.finish()

device.queue.submit(commandBuffer)
```

### `queue.writeBuffer(buffer, offset, data)`

Writes data directly to a buffer without mapping.

**Parameters:**
- `buffer` (GPUBuffer): Target buffer (must have `COPY_DST` usage)
- `offset` (Number): Byte offset in buffer
- `data` (Buffer): Data to write (Node.js Buffer or TypedArray)

**Example:**
```javascript
const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queue.writeBuffer(buffer, 0, Buffer.from(data.buffer))
```

## Device Management

### `device.poll(forceWait)`

Polls the device for completed GPU operations.

**Parameters:**
- `forceWait` (Boolean, optional): If `true`, waits for all pending operations to complete (default: `false`)

**Example:**
```javascript
// Submit commands
device.queue.submit(commandBuffer)

// Wait for GPU to finish
device.poll(true)
```

### `device.destroy()`

Destroys the device and releases resources.

**Example:**
```javascript
device.destroy()
```

## Error Handling

### `device.pushErrorScope(filter)`

Pushes an error scope for capturing GPU errors.

**Parameters:**
- `filter` (String): `'validation'` or `'out-of-memory'`

**Example:**
```javascript
device.pushErrorScope('validation')

// GPU operations that might error

const error = await device.popErrorScope()
if (error) {
  console.error('GPU error:', error)
}
```

### `device.popErrorScope()`

Pops an error scope and returns any captured error.

**Returns:** `Promise<String | null>`

**Example:**
```javascript
device.pushErrorScope('validation')

// GPU operations

const error = await device.popErrorScope()
if (error) {
  console.error('Validation error:', error)
}
```

## Complete Examples

### Compute Pipeline

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

async function computeExample() {
  // Setup
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  // Create shader
  const shader = device.createShaderModule({
    label: 'Vector Add',
    code: `
      @group(0) @binding(0) var<storage, read> a: array<f32>;
      @group(0) @binding(1) var<storage, read> b: array<f32>;
      @group(0) @binding(2) var<storage, read_write> result: array<f32>;

      @compute @workgroup_size(64)
      fn main(@builtin(global_invocation_id) id: vec3<u32>) {
        let index = id.x;
        if (index >= arrayLength(&result)) {
          return;
        }
        result[index] = a[index] + b[index];
      }
    `
  })

  // Create pipeline
  const pipeline = device.createComputePipeline(
    { label: 'Add Pipeline', entryPoint: 'main' },
    null,
    shader
  )

  // Create buffers
  const bufferA = device.createBuffer({
    size: 256,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  const bufferB = device.createBuffer({
    size: 256,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })

  const bufferResult = device.createBuffer({
    size: 256,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
    mappedAtCreation: false
  })

  // Write input data
  const a = new Float32Array(64).fill(1.0)
  const b = new Float32Array(64).fill(2.0)
  device.queue.writeBuffer(bufferA, 0, Buffer.from(a.buffer))
  device.queue.writeBuffer(bufferB, 0, Buffer.from(b.buffer))

  // Create bind group
  const bindGroup = device.createBindGroup(
    { label: 'Compute Bind Group' },
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

  // Encode and submit
  const encoder = device.createCommandEncoder({ label: 'Compute Encoder' })
  const pass = encoder.beginComputePass({ label: 'Compute Pass' })
  pass.setPipeline(pipeline)
  pass.setBindGroup(0, bindGroup)
  pass.dispatchWorkgroups(1)
  pass.end()

  device.queue.submit(encoder.finish())
  device.poll(true)

  console.log('✅ Computation complete')

  // Cleanup
  bufferA.destroy()
  bufferB.destroy()
  bufferResult.destroy()
  device.destroy()
}

computeExample()
```

### Render Pipeline

```javascript
const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('@sylphx/webgpu')

async function renderExample() {
  // Setup
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  // Create shader
  const shader = device.createShaderModule({
    label: 'Triangle Shader',
    code: `
      @vertex
      fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
        return vec4<f32>(position, 0.0, 1.0);
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
      label: 'Triangle Pipeline',
      vertex: {
        entryPoint: 'vs_main',
        buffers: [{
          arrayStride: 8,  // 2 floats * 4 bytes
          attributes: [{
            shaderLocation: 0,
            offset: 0,
            format: 'float32x2'
          }]
        }]
      },
      fragment: {
        entryPoint: 'fs_main',
        targets: [{ format: 'rgba8unorm' }]
      },
      primitive: {
        topology: 'triangle-list'
      },
      multisample: {
        count: 1
      }
    },
    null,
    shader,
    shader
  )

  // Create vertex buffer
  const vertices = new Float32Array([
     0.0,  0.5,  // Top
    -0.5, -0.5,  // Bottom left
     0.5, -0.5   // Bottom right
  ])

  const vertexBuffer = device.createBuffer({
    size: vertices.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    mappedAtCreation: false
  })
  device.queue.writeBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

  // Create render target
  const texture = device.createTexture({
    width: 800,
    height: 600,
    format: 'rgba8unorm',
    usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC
  })

  const view = texture.createView()

  // Render
  const encoder = device.createCommandEncoder({ label: 'Render Encoder' })
  const pass = encoder.beginRenderPass(
    {
      colorAttachments: [{
        view: view,
        loadOp: 'clear',
        storeOp: 'store',
        clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
      }]
    },
    [view],
    null,
    null
  )

  pass.setPipeline(pipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.draw(3, 1, 0, 0)
  pass.end()

  device.queue.submit(encoder.finish())
  device.poll(true)

  console.log('✅ Rendering complete')

  // Cleanup
  vertexBuffer.destroy()
  texture.destroy()
  device.destroy()
}

renderExample()
```

## Best Practices

### 1. Resource Cleanup

Always destroy resources when done:

```javascript
// Manual cleanup
buffer.destroy()
texture.destroy()
device.destroy()

// Or use try/finally
let device, buffer
try {
  device = await adapter.requestDevice()
  buffer = device.createBuffer({ size: 1024, usage: GPUBufferUsage.STORAGE })
  // ... use resources
} finally {
  buffer?.destroy()
  device?.destroy()
}
```

### 2. Error Handling

Use error scopes for robust error handling:

```javascript
device.pushErrorScope('validation')

try {
  // GPU operations that might fail
  const buffer = device.createBuffer({
    size: 1024,
    usage: GPUBufferUsage.STORAGE
  })
} catch (error) {
  console.error('Synchronous error:', error)
}

const gpuError = await device.popErrorScope()
if (gpuError) {
  console.error('GPU validation error:', gpuError)
}
```

### 3. Buffer Usage Flags

Always specify the minimum necessary usage flags:

```javascript
// ✅ Good: Only necessary flags
const buffer = device.createBuffer({
  size: 256,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

// ❌ Bad: Unnecessary flags
const buffer = device.createBuffer({
  size: 256,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST |
         GPUBufferUsage.COPY_SRC | GPUBufferUsage.MAP_READ  // Unnecessary
})
```

### 4. Use Labels for Debugging

Always provide labels in development:

```javascript
const buffer = device.createBuffer({
  label: 'Particle Positions',  // Helps with debugging
  size: 1024,
  usage: GPUBufferUsage.STORAGE
})

const shader = device.createShaderModule({
  label: 'Particle Update Shader',
  code: shaderCode
})
```

## Troubleshooting

### Error: "Failed to request device"

**Cause:** Adapter doesn't support requested features or limits.

**Solution:**
```javascript
// Check adapter capabilities first
const limits = adapter.getLimits()
console.log('Max buffer size:', limits.maxBufferSize)

const device = await adapter.requestDevice()
```

### Error: "Buffer size must be multiple of 4"

**Cause:** Buffer sizes must be aligned to 4 bytes.

**Solution:**
```javascript
// ❌ Wrong
const buffer = device.createBuffer({
  size: 17,  // Not multiple of 4
  usage: GPUBufferUsage.STORAGE
})

// ✅ Correct
const buffer = device.createBuffer({
  size: 20,  // Multiple of 4
  usage: GPUBufferUsage.STORAGE
})
```

### Error: "Invalid usage flags"

**Cause:** Incompatible usage flag combinations.

**Solution:**
```javascript
// Some combinations are invalid
// For example: MAP_READ + MAP_WRITE is not allowed

// ✅ Correct: Use either MAP_READ or MAP_WRITE, not both
const readBuffer = device.createBuffer({
  size: 256,
  usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ
})
```

## TypeScript

```typescript
import { GPUDevice, GPUBuffer, GPUTexture, GPUShaderModule } from '@sylphx/webgpu'

const device: GPUDevice = await adapter.requestDevice()

const buffer: GPUBuffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE
})

const texture: GPUTexture = device.createTexture({
  width: 512,
  height: 512,
  format: 'rgba8unorm',
  usage: GPUTextureUsage.TEXTURE_BINDING
})

const shader: GPUShaderModule = device.createShaderModule({
  code: shaderCode
})
```

## See Also

- [Buffer](/api/buffer) - GPU buffer operations
- [Texture](/api/texture) - Texture operations
- [Pipeline](/api/pipeline) - Pipeline creation
- [Command Encoder](/api/command-encoder) - Recording commands
- [Adapter](/api/adapter) - GPU adapter info
