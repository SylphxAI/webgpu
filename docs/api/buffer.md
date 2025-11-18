# Buffer

Buffers are contiguous blocks of GPU memory for storing data.

## Creating Buffers

```javascript
const { BufferUsage } = require('@sylphx/webgpu')

const buffer = device.createBuffer(
  size,                  // Size in bytes
  usage,                 // Usage flags
  mappedAtCreation       // Map for initial write
)
```

## Buffer Usage Flags

Combine flags with bitwise OR (`|`):

```javascript
const usage = BufferUsage.STORAGE | BufferUsage.COPY_DST
```

### Available Flags

| Flag | Value | Description |
|------|-------|-------------|
| `MAP_READ` | 0x0001 | Can be mapped for reading |
| `MAP_WRITE` | 0x0002 | Can be mapped for writing |
| `COPY_SRC` | 0x0004 | Can be copied from |
| `COPY_DST` | 0x0008 | Can be copied to |
| `INDEX` | 0x0010 | Can be used as index buffer |
| `VERTEX` | 0x0020 | Can be used as vertex buffer |
| `UNIFORM` | 0x0040 | Can be used as uniform buffer |
| `STORAGE` | 0x0080 | Can be used as storage buffer |
| `INDIRECT` | 0x0100 | Can be used for indirect draw/dispatch |
| `QUERY_RESOLVE` | 0x0200 | Can store query results |

## Writing to Buffers

### Queue Write (Recommended)

```javascript
const data = new Float32Array([1.0, 2.0, 3.0, 4.0])

device.queueWriteBuffer(
  buffer,
  0,                           // Offset in buffer
  Buffer.from(data.buffer)     // Data to write
)
```

### Mapped Write (Initial Data)

```javascript
const buffer = device.createBuffer(
  16,  // 4 floats * 4 bytes
  BufferUsage.STORAGE,
  true  // mappedAtCreation
)

const mapped = buffer.getMappedRange(0, 16)
const view = new Float32Array(mapped.buffer, mapped.byteOffset, 4)
view.set([1.0, 2.0, 3.0, 4.0])
buffer.unmap()
```

## Reading from Buffers

Buffers must have `MAP_READ` usage to be mapped for reading.

### Async Mapping

```javascript
const buffer = device.createBuffer(
  16,
  BufferUsage.MAP_READ | BufferUsage.COPY_DST,
  false
)

// Write data
const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))

// Submit and wait
const encoder = device.createCommandEncoder()
device.queueSubmit(encoder.finish())
device.poll(true)

// Map for reading
const mapped = await buffer.mapRead()
const result = new Float32Array(mapped.buffer, mapped.byteOffset, 4)

console.log('Data:', Array.from(result))
// Output: Data: [1, 2, 3, 4]
```

## Methods

### `buffer.getMappedRange(offset, size)`

Gets a mapped range for writing (only if created with `mappedAtCreation: true`).

**Parameters:**
- `offset` (Number): Byte offset
- `size` (Number): Byte size

**Returns:** Object with `buffer`, `byteOffset`, `byteLength`

**Example:**
```javascript
const buffer = device.createBuffer(16, usage, true)
const mapped = buffer.getMappedRange(0, 16)
const view = new Float32Array(mapped.buffer, mapped.byteOffset, 4)
view.set([1, 2, 3, 4])
buffer.unmap()
```

### `buffer.mapRead()`

Maps buffer for reading.

**Returns:** Promise resolving to object with `buffer`, `byteOffset`, `byteLength`

**Requirements:**
- Buffer must have `MAP_READ` usage
- Must wait for GPU operations to complete

**Example:**
```javascript
const mapped = await buffer.mapRead()
const data = new Float32Array(mapped.buffer, mapped.byteOffset, length)
console.log('Data:', Array.from(data))
```

### `buffer.unmap()`

Unmaps the buffer.

**Example:**
```javascript
const buffer = device.createBuffer(16, usage, true)
const mapped = buffer.getMappedRange(0, 16)
// ... write to mapped
buffer.unmap()  // Now ready for GPU use
```

### `buffer.destroy()`

Destroys the buffer and releases GPU memory.

**Example:**
```javascript
buffer.destroy()
```

## Buffer Types

### Storage Buffer

For read/write access in compute shaders.

```javascript
const buffer = device.createBuffer(
  1024,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
```

**Shader:**
```wgsl
@group(0) @binding(0) var<storage, read_write> data: array<f32>;
```

### Uniform Buffer

For constant data in shaders (faster but smaller).

```javascript
const buffer = device.createBuffer(
  64,  // Must be multiple of 16 bytes
  BufferUsage.UNIFORM | BufferUsage.COPY_DST,
  false
)
```

**Shader:**
```wgsl
struct Uniforms {
  time: f32,
  resolution: vec2<f32>
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
```

### Vertex Buffer

For vertex data in render pipelines.

```javascript
const vertices = new Float32Array([
  -1.0, -1.0,  // Vertex 1
   1.0, -1.0,  // Vertex 2
   0.0,  1.0   // Vertex 3
])

const buffer = device.createBuffer(
  vertices.byteLength,
  BufferUsage.VERTEX | BufferUsage.COPY_DST,
  false
)

device.queueWriteBuffer(buffer, 0, Buffer.from(vertices.buffer))
```

### Index Buffer

For indexed rendering.

```javascript
const indices = new Uint16Array([0, 1, 2, 0, 2, 3])

const buffer = device.createBuffer(
  indices.byteLength,
  BufferUsage.INDEX | BufferUsage.COPY_DST,
  false
)

device.queueWriteBuffer(buffer, 0, Buffer.from(indices.buffer))
```

## Copying Buffers

```javascript
const srcBuffer = device.createBuffer(
  1024,
  BufferUsage.STORAGE | BufferUsage.COPY_SRC,
  false
)

const dstBuffer = device.createBuffer(
  1024,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)

const encoder = device.createCommandEncoder()
encoder.copyBufferToBuffer(
  srcBuffer, 0,   // Source buffer, offset
  dstBuffer, 0,   // Destination buffer, offset
  1024            // Size
)
device.queueSubmit(encoder.finish())
```

## Best Practices

### 1. Use Correct Usage Flags

```javascript
// ❌ Don't: Too many flags
const buffer = device.createBuffer(
  size,
  BufferUsage.MAP_READ | BufferUsage.MAP_WRITE |
  BufferUsage.COPY_SRC | BufferUsage.COPY_DST |
  BufferUsage.STORAGE | BufferUsage.UNIFORM,
  false
)

// ✅ Do: Only what you need
const buffer = device.createBuffer(
  size,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
```

### 2. Align Sizes

Uniform buffers must be 16-byte aligned:

```javascript
// ❌ Don't
const buffer = device.createBuffer(
  13,
  BufferUsage.UNIFORM,
  false
)

// ✅ Do
const buffer = device.createBuffer(
  16,  // Aligned to 16 bytes
  BufferUsage.UNIFORM,
  false
)
```

### 3. Reuse Buffers

```javascript
// ✅ Create once
const buffer = device.createBuffer(maxSize, usage, false)

// Reuse for different data
for (const dataset of datasets) {
  device.queueWriteBuffer(buffer, 0, dataset)
  runComputation()
}

// Cleanup once
buffer.destroy()
```

### 4. Wait Before Reading

```javascript
// ✅ Always wait for GPU before mapping
device.queueSubmit(encoder.finish())
device.poll(true)  // Wait for completion

const mapped = await buffer.mapRead()
```

## Complete Example

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

async function bufferExample() {
  // Setup
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter()
  const device = adapter.requestDevice()

  // Create buffer
  const buffer = device.createBuffer(
    16,  // 4 floats
    BufferUsage.STORAGE | BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  // Write data
  const input = new Float32Array([1.5, 2.5, 3.5, 4.5])
  device.queueWriteBuffer(buffer, 0, Buffer.from(input.buffer))

  // Wait for GPU
  const encoder = device.createCommandEncoder()
  device.queueSubmit(encoder.finish())
  device.poll(true)

  // Read data
  const mapped = await buffer.mapRead()
  const output = new Float32Array(mapped.buffer, mapped.byteOffset, 4)

  console.log('Input: ', Array.from(input))
  console.log('Output:', Array.from(output))

  // Cleanup
  buffer.destroy()
  device.destroy()
}

bufferExample()
```

## TypeScript

```typescript
import { Buffer, BufferUsage } from '@sylphx/webgpu'

const buffer: Buffer = device.createBuffer(
  1024,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
```

## See Also

- [Device](/api/device)
- [Buffers Guide](/guide/buffers)
- [Compute Shaders](/guide/compute)
