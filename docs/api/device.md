# Device

The Device is the main interface for GPU operations. It creates resources and executes commands.

## Creating Device

```javascript
const device = adapter.requestDevice({
  label: 'My Device',
  requiredLimits: {},
  requiredFeatures: []
})
```

## Resource Creation

### `device.createBuffer(size, usage, mappedAtCreation)`

Creates a GPU buffer.

**Parameters:**
- `size` (Number): Buffer size in bytes
- `usage` (Number): Usage flags (bitwise OR of `BufferUsage` flags)
- `mappedAtCreation` (Boolean): Whether to map for initial write

**Returns:** `Buffer`

**Example:**
```javascript
const { BufferUsage } = require('@sylphx/webgpu')

const buffer = device.createBuffer(
  1024,  // 1 KB
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
```

### `device.createTexture(descriptor)`

Creates a GPU texture.

**Parameters:**
- `descriptor` (Object)
  - `label` (String, optional): Debug label
  - `size` (Object): `{ width, height, depthOrArrayLayers }`
  - `format` (String): Texture format (e.g., `'rgba8unorm'`)
  - `usage` (Number): Usage flags
  - `dimension` (String): `'1d'`, `'2d'`, or `'3d'`
  - `mipLevelCount` (Number): Number of mip levels
  - `sampleCount` (Number): Sample count (1, 4, etc.)

**Returns:** `Texture`

**Example:**
```javascript
const { TextureUsage } = require('@sylphx/webgpu')

const texture = device.createTexture({
  label: 'Render Target',
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})
```

### `device.createSampler(descriptor)`

Creates a texture sampler.

**Parameters:**
- `descriptor` (Object)
  - `label` (String, optional): Debug label
  - `addressModeU` (String): `'repeat'`, `'clamp-to-edge'`, `'mirror-repeat'`
  - `addressModeV` (String): Same as U
  - `addressModeW` (String): Same as U
  - `magFilter` (String): `'nearest'` or `'linear'`
  - `minFilter` (String): `'nearest'` or `'linear'`
  - `mipmapFilter` (String): `'nearest'` or `'linear'`
  - `lodMinClamp` (Number): Minimum LOD
  - `lodMaxClamp` (Number): Maximum LOD
  - `compare` (String, optional): Compare function
  - `maxAnisotropy` (Number): Anisotropy level (1-16)

**Returns:** `Sampler`

**Example:**
```javascript
const sampler = device.createSampler({
  addressModeU: 'repeat',
  addressModeV: 'repeat',
  magFilter: 'linear',
  minFilter: 'linear',
  mipmapFilter: 'linear'
})
```

### `device.createShaderModule(code, label)`

Creates a shader module from WGSL code.

**Parameters:**
- `code` (String): WGSL shader code
- `label` (String, optional): Debug label

**Returns:** `ShaderModule`

**Example:**
```javascript
const shader = device.createShaderModule(`
  @compute @workgroup_size(64)
  fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // Shader code
  }
`, 'My Shader')
```

### `device.createComputePipeline(descriptor)`

Creates a compute pipeline.

**Parameters:**
- `descriptor` (Object)
  - `label` (String, optional): Debug label
  - `compute` (Object):
    - `module` (ShaderModule): Shader module
    - `entryPoint` (String): Entry function name

**Returns:** `ComputePipeline`

**Example:**
```javascript
const pipeline = device.createComputePipeline({
  label: 'Vector Add',
  compute: {
    module: shader,
    entryPoint: 'main'
  }
})
```

### `device.createRenderPipeline(descriptor)`

Creates a render pipeline.

**Parameters:**
- `descriptor` (Object): See [Pipelines Guide](/guide/pipelines)

**Returns:** `RenderPipeline`

**Example:**
```javascript
const pipeline = device.createRenderPipeline({
  vertex: {
    module: shader,
    entryPoint: 'vs_main',
    buffers: [/* ... */]
  },
  fragment: {
    module: shader,
    entryPoint: 'fs_main',
    targets: [{ format: 'rgba8unorm' }]
  },
  primitive: { topology: 'triangle-list' },
  multisample: { count: 1 }
})
```

### `device.createBindGroup(descriptor)`

Creates a bind group for shader resources.

**Parameters:**
- `descriptor` (Object)
  - `label` (String, optional): Debug label
  - `layout` (BindGroupLayout): Layout from pipeline
  - `entries` (Array): Resource bindings

**Returns:** `BindGroup`

**Example:**
```javascript
const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: { buffer: bufferA } },
    { binding: 1, resource: { buffer: bufferB } },
    { binding: 2, resource: texture.createView() },
    { binding: 3, resource: sampler }
  ]
})
```

## Command Encoding

### `device.createCommandEncoder(label)`

Creates a command encoder.

**Parameters:**
- `label` (String, optional): Debug label

**Returns:** `CommandEncoder`

**Example:**
```javascript
const encoder = device.createCommandEncoder('My Commands')
```

### `device.createRenderBundleEncoder(descriptor)`

Creates a render bundle encoder for pre-recording render commands.

**Parameters:**
- `descriptor` (Object)
  - `colorFormats` (Array): Render target formats
  - `depthStencilFormat` (String, optional): Depth format
  - `sampleCount` (Number): Sample count

**Returns:** `RenderBundleEncoder`

**Example:**
```javascript
const bundleEncoder = device.createRenderBundleEncoder({
  colorFormats: ['rgba8unorm'],
  sampleCount: 1
})
```

## Queue Operations

### `device.queueWriteBuffer(buffer, offset, data)`

Writes data to buffer.

**Parameters:**
- `buffer` (Buffer): Target buffer
- `offset` (Number): Byte offset in buffer
- `data` (Buffer): Data to write

**Example:**
```javascript
const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))
```

### `device.queueWriteTexture(destination, data, dataLayout, size)`

Writes data to texture.

**Parameters:**
- `destination` (Object): `{ texture, mipLevel, origin }`
- `data` (Buffer): Pixel data
- `dataLayout` (Object): `{ offset, bytesPerRow, rowsPerImage }`
- `size` (Object): `{ width, height, depthOrArrayLayers }`

**Example:**
```javascript
device.queueWriteTexture(
  { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  Buffer.from(pixels.buffer),
  { offset: 0, bytesPerRow: width * 4, rowsPerImage: height },
  { width, height, depthOrArrayLayers: 1 }
)
```

### `device.queueSubmit(...commandBuffers)`

Submits command buffers to GPU.

**Parameters:**
- `commandBuffers` (CommandBuffer): One or more command buffers

**Example:**
```javascript
const encoder = device.createCommandEncoder()
// ... encode commands
const commandBuffer = encoder.finish()

device.queueSubmit(commandBuffer)
```

### `device.poll(wait)`

Polls for GPU work completion.

**Parameters:**
- `wait` (Boolean): Whether to wait for completion

**Example:**
```javascript
device.queueSubmit(commandBuffer)
device.poll(true)  // Wait for GPU to finish
```

## Device Management

### `device.destroy()`

Destroys the device and releases resources.

**Example:**
```javascript
device.destroy()
```

## Complete Example

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

// Setup
const gpu = Gpu.create()
const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
const device = adapter.requestDevice({ label: 'Main Device' })

// Create buffer
const buffer = device.createBuffer(
  16,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)

// Write data
const data = new Float32Array([1, 2, 3, 4])
device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))

// Submit and wait
const encoder = device.createCommandEncoder()
device.queueSubmit(encoder.finish())
device.poll(true)

// Cleanup
buffer.destroy()
device.destroy()
```

## TypeScript

```typescript
import { Device, Buffer, Texture, ComputePipeline } from '@sylphx/webgpu'

const device: Device = adapter.requestDevice()

const buffer: Buffer = device.createBuffer(1024, usage, false)
const texture: Texture = device.createTexture({ /* ... */ })
const pipeline: ComputePipeline = device.createComputePipeline({ /* ... */ })
```

## See Also

- [Buffer](/api/buffer)
- [Texture](/api/texture)
- [Pipeline](/api/pipeline)
- [Command Encoder](/api/command-encoder)
