# Command Encoder

Command encoders record GPU commands for later execution.

## Creating Command Encoder

```javascript
const encoder = device.createCommandEncoder('My Commands')
```

## Compute Pass

### `encoder.beginComputePass(descriptor)`

Begins a compute pass for running compute shaders.

**Parameters:**
- `descriptor` (Object, optional)
  - `label` (String, optional): Debug label

**Returns:** `ComputePassEncoder`

**Example:**
```javascript
const encoder = device.createCommandEncoder()
const pass = encoder.beginComputePass({ label: 'Vector Add' })

pass.setPipeline(pipeline)
pass.setBindGroup(0, bindGroup)
pass.dispatchWorkgroups(numWorkgroups, 1, 1)
pass.end()

device.queueSubmit(encoder.finish())
```

### Compute Pass Methods

**`pass.setPipeline(pipeline)`**
Sets the compute pipeline.

**`pass.setBindGroup(index, bindGroup)`**
Sets a bind group at the specified index (0-3).

**`pass.dispatchWorkgroups(x, y, z)`**
Dispatches compute workgroups.

**`pass.end()`**
Ends the compute pass.

## Render Pass

### `encoder.beginRenderPass(descriptor)`

Begins a render pass for rendering.

**Parameters:**
- `descriptor` (Object)
  - `label` (String, optional): Debug label
  - `colorAttachments` (Array): Color render targets
  - `depthStencilAttachment` (Object, optional): Depth/stencil target

**Returns:** `RenderPassEncoder`

**Example:**
```javascript
const encoder = device.createCommandEncoder()
const pass = encoder.beginRenderPass({
  label: 'Main Render Pass',
  colorAttachments: [
    {
      view: textureView,
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
  ],
  depthStencilAttachment: {
    view: depthTextureView,
    depthLoadOp: 'clear',
    depthStoreOp: 'store',
    depthClearValue: 1.0
  }
})

pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.draw(3, 1, 0, 0)
pass.end()

device.queueSubmit(encoder.finish())
```

### Color Attachment

**Properties:**
- `view` (TextureView): Render target
- `resolveTarget` (TextureView, optional): For MSAA resolve
- `loadOp` (String): `'load'` or `'clear'`
- `storeOp` (String): `'store'` or `'discard'`
- `clearValue` (Object, optional): `{ r, g, b, a }` (0.0-1.0)

### Depth/Stencil Attachment

**Properties:**
- `view` (TextureView): Depth/stencil target
- `depthLoadOp` (String): `'load'` or `'clear'`
- `depthStoreOp` (String): `'store'` or `'discard'`
- `depthClearValue` (Number): Clear value (0.0-1.0)
- `depthReadOnly` (Boolean, optional): Read-only depth
- `stencilLoadOp` (String, optional): `'load'` or `'clear'`
- `stencilStoreOp` (String, optional): `'store'` or `'discard'`
- `stencilClearValue` (Number, optional): Clear value (0-255)
- `stencilReadOnly` (Boolean, optional): Read-only stencil

### Render Pass Methods

**`pass.setPipeline(pipeline)`**
Sets the render pipeline.

**`pass.setBindGroup(index, bindGroup)`**
Sets a bind group at the specified index.

**`pass.setVertexBuffer(slot, buffer)`**
Sets a vertex buffer at the specified slot.

**`pass.setIndexBuffer(buffer, format)`**
Sets the index buffer. Format: `'uint16'` or `'uint32'`.

**`pass.draw(vertexCount, instanceCount, firstVertex, firstInstance)`**
Draws vertices.

**`pass.drawIndexed(indexCount, instanceCount, firstIndex, baseVertex, firstInstance)`**
Draws indexed vertices.

**`pass.drawIndirect(buffer, offset)`**
Draws with GPU-generated parameters.

**`pass.drawIndexedIndirect(buffer, offset)`**
Draws indexed with GPU-generated parameters.

**`pass.executeBundles(bundles)`**
Executes pre-recorded render bundles.

**`pass.end()`**
Ends the render pass.

## Buffer Operations

### `encoder.copyBufferToBuffer(src, srcOffset, dst, dstOffset, size)`

Copies data between buffers.

**Example:**
```javascript
const encoder = device.createCommandEncoder()
encoder.copyBufferToBuffer(
  srcBuffer, 0,    // Source buffer, offset
  dstBuffer, 0,    // Destination buffer, offset
  1024             // Size in bytes
)
device.queueSubmit(encoder.finish())
```

### `encoder.copyBufferToTexture(source, destination, size)`

Copies buffer data to texture.

**Example:**
```javascript
encoder.copyBufferToTexture(
  {
    buffer: buffer,
    offset: 0,
    bytesPerRow: width * 4,
    rowsPerImage: height
  },
  {
    texture: texture,
    mipLevel: 0,
    origin: { x: 0, y: 0, z: 0 }
  },
  {
    width: width,
    height: height,
    depthOrArrayLayers: 1
  }
)
```

### `encoder.copyTextureToBuffer(source, destination, size)`

Copies texture data to buffer.

**Example:**
```javascript
encoder.copyTextureToBuffer(
  {
    texture: texture,
    mipLevel: 0,
    origin: { x: 0, y: 0, z: 0 }
  },
  {
    buffer: buffer,
    offset: 0,
    bytesPerRow: width * 4,
    rowsPerImage: height
  },
  {
    width: width,
    height: height,
    depthOrArrayLayers: 1
  }
)
```

### `encoder.copyTextureToTexture(source, destination, size)`

Copies data between textures.

**Example:**
```javascript
encoder.copyTextureToTexture(
  { texture: srcTexture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  { texture: dstTexture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  { width, height, depthOrArrayLayers: 1 }
)
```

## Finishing

### `encoder.finish()`

Finishes encoding and returns a command buffer.

**Returns:** `CommandBuffer`

**Example:**
```javascript
const encoder = device.createCommandEncoder()
// ... encode commands
const commandBuffer = encoder.finish()

device.queueSubmit(commandBuffer)
```

## Complete Example

### Compute Pass

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = gpu.requestAdapter()
const device = adapter.requestDevice()

// Create resources
const shader = device.createShaderModule(`
  @group(0) @binding(0) var<storage, read_write> data: array<f32>;

  @compute @workgroup_size(64)
  fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    data[id.x] = data[id.x] * 2.0;
  }
`)

const pipeline = device.createComputePipeline({
  compute: { module: shader, entryPoint: 'main' }
})

const buffer = device.createBuffer(256, BufferUsage.STORAGE | BufferUsage.COPY_DST, false)

const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [{ binding: 0, resource: { buffer } }]
})

// Encode commands
const encoder = device.createCommandEncoder('Multiply by 2')

const pass = encoder.beginComputePass({ label: 'Compute Pass' })
pass.setPipeline(pipeline)
pass.setBindGroup(0, bindGroup)
pass.dispatchWorkgroups(4, 1, 1)  // 4 * 64 = 256 threads
pass.end()

const commandBuffer = encoder.finish()

// Submit
device.queueSubmit(commandBuffer)
device.poll(true)
```

### Render Pass

```javascript
const { TextureUsage } = require('@sylphx/webgpu')

// Create render target
const texture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})

const view = texture.createView()

// Encode render pass
const encoder = device.createCommandEncoder('Render Triangle')

const pass = encoder.beginRenderPass({
  colorAttachments: [
    {
      view: view,
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }
    }
  ]
})

pass.setPipeline(renderPipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.draw(3, 1, 0, 0)
pass.end()

device.queueSubmit(encoder.finish())
device.poll(true)
```

## Best Practices

### 1. Minimize Encoders

```javascript
// ❌ Don't: Many encoders
for (const obj of objects) {
  const encoder = device.createCommandEncoder()
  // ... encode
  device.queueSubmit(encoder.finish())
}

// ✅ Do: Single encoder
const encoder = device.createCommandEncoder()
for (const obj of objects) {
  // ... encode all commands
}
device.queueSubmit(encoder.finish())
```

### 2. Batch Submissions

```javascript
// ❌ Don't: Submit every iteration
for (let i = 0; i < 100; i++) {
  const encoder = device.createCommandEncoder()
  // ... encode
  device.queueSubmit(encoder.finish())
}

// ✅ Do: Batch submissions
const commands = []
for (let i = 0; i < 100; i++) {
  const encoder = device.createCommandEncoder()
  // ... encode
  commands.push(encoder.finish())
}
device.queueSubmit(...commands)
```

### 3. Use Labels for Debugging

```javascript
const encoder = device.createCommandEncoder('Frame 123')

const pass = encoder.beginRenderPass({
  label: 'Shadow Map',
  // ...
})
```

## TypeScript

```typescript
import { CommandEncoder, ComputePassEncoder, RenderPassEncoder } from '@sylphx/webgpu'

const encoder: CommandEncoder = device.createCommandEncoder()

const computePass: ComputePassEncoder = encoder.beginComputePass()
const renderPass: RenderPassEncoder = encoder.beginRenderPass({ /* ... */ })
```

## See Also

- [Device](/api/device)
- [Pipeline](/api/pipeline)
- [Compute Guide](/guide/compute)
- [Rendering Guide](/guide/rendering)
