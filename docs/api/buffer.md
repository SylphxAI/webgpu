# GPUBuffer

Buffers are contiguous blocks of GPU memory for storing data. They are used for vertex data, index data, uniform data, storage data, and more.

## Overview

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

const gpu = Gpu()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()

const buffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
  mappedAtCreation: false
})
```

## Creating Buffers

### `device.createBuffer(descriptor)`

Creates a new GPU buffer.

**Parameters:**
- `descriptor` (Object):
  - `size` (Number): Size in bytes
  - `usage` (Number): Buffer usage flags (bitwise OR combination)
  - `mappedAtCreation` (Boolean): Whether buffer starts mapped for writing (default: `false`)
  - `label` (String, optional): Debug label

**Returns:** `GPUBuffer`

**Example:**
```javascript
const buffer = device.createBuffer({
  size: 16,  // 4 × Float32 = 16 bytes
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
  mappedAtCreation: true  // Start mapped for initial data
})
```

## Buffer Usage Flags

Combine usage flags with bitwise OR (`|`):

```javascript
const usage = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ
```

### Available Flags

| Constant | Value | Description |
|----------|-------|-------------|
| `MAP_READ` | 0x0001 | Can be mapped for CPU reading |
| `MAP_WRITE` | 0x0002 | Can be mapped for CPU writing |
| `COPY_SRC` | 0x0004 | Can be source of copy operation |
| `COPY_DST` | 0x0008 | Can be destination of copy operation |
| `INDEX` | 0x0010 | Can be used as index buffer in rendering |
| `VERTEX` | 0x0020 | Can be used as vertex buffer in rendering |
| `UNIFORM` | 0x0040 | Can be used as uniform buffer in shaders |
| `STORAGE` | 0x0080 | Can be used as storage buffer in shaders |
| `INDIRECT` | 0x0100 | Can be used for indirect draw/dispatch |
| `QUERY_RESOLVE` | 0x0200 | Can store query results |

## Properties

### `buffer.size()`

Returns the size of the buffer in bytes.

**Returns:** `Number`

**Example:**
```javascript
const size = buffer.size()
console.log(`Buffer size: ${size} bytes`)
```

### `buffer.usage()`

Returns the usage flags of the buffer as a bitwise combination.

**Returns:** `Number`

**Example:**
```javascript
const usage = buffer.usage()
if (usage & GPUBufferUsage.STORAGE) {
  console.log('Buffer can be used for storage')
}
```

### `buffer.mapState()`

Returns the current mapping state of the buffer.

**Returns:** `String` - One of:
- `"unmapped"` - Buffer is not mapped
- `"pending"` - `mapAsync()` in progress
- `"mapped"` - Buffer is currently mapped

**Example:**
```javascript
console.log(buffer.mapState())  // "unmapped"

const promise = buffer.mapAsync('READ')
console.log(buffer.mapState())  // "pending"

await promise
console.log(buffer.mapState())  // "mapped"
```

## Methods

### `buffer.mapAsync(mode)`

Asynchronously maps the buffer for CPU access.

**Parameters:**
- `mode` (String): Mapping mode
  - `"READ"` - Map for reading (requires `MAP_READ` usage)
  - `"WRITE"` - Map for writing (requires `MAP_WRITE` usage)

**Returns:** `Promise<void>`

**Example:**
```javascript
// Map for reading
await buffer.mapAsync('READ')
const data = buffer.getMappedRange()
const view = new Float32Array(data)
console.log(view[0])
buffer.unmap()

// Map for writing
await buffer.mapAsync('WRITE')
const writeData = buffer.getMappedRange()
const writeView = new Float32Array(writeData)
writeView[0] = 42.0
buffer.unmap()
```

### `buffer.getMappedRange([offset], [size])`

Returns an ArrayBuffer representing the mapped memory range.

**Parameters:**
- `offset` (Number, optional): Byte offset into buffer (default: 0)
  - Must be multiple of 8
- `size` (Number, optional): Number of bytes (default: remaining bytes)
  - Must be multiple of 4

**Returns:** `ArrayBuffer`

**Throws:** Error if:
- Buffer is not in "mapped" state
- Offset is not multiple of 8
- Size is not multiple of 4
- Range exceeds buffer bounds
- Range overlaps with another active `getMappedRange()` call

**Example:**
```javascript
// Get full range
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.MAP_WRITE,
  mappedAtCreation: true
})

const range = buffer.getMappedRange()
const view = new Float32Array(range)
view[0] = 1.0
view[1] = 2.0
buffer.unmap()

// Get partial range
const range1 = buffer.getMappedRange(0, 8)   // First 8 bytes
const range2 = buffer.getMappedRange(8, 8)   // Next 8 bytes (non-overlapping OK)
```

### `buffer.unmap()`

Unmaps the buffer and flushes any changes to GPU.

**Example:**
```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST,
  mappedAtCreation: true
})

const range = buffer.getMappedRange()
const view = new Float32Array(range)
view.set([1, 2, 3, 4])

buffer.unmap()  // Flushes changes to GPU
```

### `buffer.destroy()`

Destroys the buffer and releases GPU resources.

**Example:**
```javascript
buffer.destroy()
```

## Writing Data to Buffers

### Method 1: mappedAtCreation (Recommended for Initial Data)

Best for writing initial data when creating the buffer.

```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE,
  mappedAtCreation: true
})

// Buffer starts in "mapped" state
const arrayBuffer = buffer.getMappedRange()
const view = new Float32Array(arrayBuffer)
view.set([1.0, 2.0, 3.0, 4.0])

buffer.unmap()  // Now ready for GPU use
```

### Method 2: queueWriteBuffer (Recommended for Updates)

Best for updating buffer contents after creation.

```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queue.writeBuffer(
  buffer,
  0,                            // Offset in buffer
  data.buffer,                  // ArrayBuffer
  0,                            // Offset in source
  data.byteLength               // Size
)
```

### Method 3: mapAsync + getMappedRange

For advanced use cases with MAP_WRITE buffers.

```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.MAP_WRITE
})

await buffer.mapAsync('WRITE')
const arrayBuffer = buffer.getMappedRange()
const view = new Float32Array(arrayBuffer)
view.set([1.0, 2.0, 3.0, 4.0])
buffer.unmap()
```

## Reading Data from Buffers

Buffers must have `MAP_READ` and `COPY_DST` usage flags.

```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ
})

// Write some data
const input = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queue.writeBuffer(buffer, 0, input.buffer)

// Wait for GPU operations to complete
device.queue.submit([])

// Map for reading
await buffer.mapAsync('READ')
const arrayBuffer = buffer.getMappedRange()
const view = new Float32Array(arrayBuffer)

console.log('Data:', Array.from(view))
// Output: Data: [1, 2, 3, 4]

buffer.unmap()
```

## State Machine

Buffer mapping follows a strict state machine:

```
    unmapped
       ↓
  mapAsync() called
       ↓
    pending
       ↓
  Promise resolves
       ↓
    mapped ←→ getMappedRange() OK
       ↓
   unmap() called
       ↓
    unmapped
```

**Rules:**
- `getMappedRange()` only works in "mapped" state
- Calling `getMappedRange()` in "unmapped" or "pending" state throws error
- `unmap()` transitions from "mapped" → "unmapped"
- All `getMappedRange()` calls are invalidated on `unmap()`

## Overlapping Range Detection

**WebGPU Standard Requirement:** Multiple `getMappedRange()` calls on the same buffer must not overlap.

```javascript
const buffer = device.createBuffer({
  size: 64,
  usage: GPUBufferUsage.STORAGE,
  mappedAtCreation: true
})

// ✅ OK: Non-overlapping ranges
const range1 = buffer.getMappedRange(0, 32)
const range2 = buffer.getMappedRange(32, 32)

// ❌ ERROR: Overlapping ranges
const range3 = buffer.getMappedRange(16, 32)  // Overlaps with range1!
// Throws: "getMappedRange() range [16, 48) overlaps with existing range [0, 32)"
```

## Alignment Requirements

**WebGPU Specification:**
- `offset` must be multiple of 8
- `size` must be multiple of 4

```javascript
// ✅ Valid alignments
buffer.getMappedRange(0, 16)
buffer.getMappedRange(8, 24)
buffer.getMappedRange(16, 32)

// ❌ Invalid alignments
buffer.getMappedRange(5, 16)   // offset not multiple of 8
buffer.getMappedRange(0, 17)   // size not multiple of 4
```

## Buffer Types and Use Cases

### Storage Buffer (Read/Write)

For dynamic data in compute shaders.

```javascript
const buffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
})
```

**WGSL Shader:**
```wgsl
@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
  data[id.x] = data[id.x] * 2.0;
}
```

### Uniform Buffer (Read-Only Constants)

For shader constants (faster than storage, max 64KB).

```javascript
const buffer = device.createBuffer({
  size: 16,  // Must be multiple of 16 for uniforms
  usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
})

// Update uniforms
const uniforms = new Float32Array([
  Date.now() / 1000,  // time
  1920, 1080,         // resolution
  0                   // padding
])
device.queue.writeBuffer(buffer, 0, uniforms.buffer)
```

**WGSL Shader:**
```wgsl
struct Uniforms {
  time: f32,
  resolution: vec2<f32>
}

@group(0) @binding(0) var<uniform> u: Uniforms;
```

### Vertex Buffer

For vertex attributes in rendering.

```javascript
const vertices = new Float32Array([
  // Position (x, y)    Color (r, g, b)
  -0.5, -0.5,           1.0, 0.0, 0.0,
   0.5, -0.5,           0.0, 1.0, 0.0,
   0.0,  0.5,           0.0, 0.0, 1.0
])

const buffer = device.createBuffer({
  size: vertices.byteLength,
  usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
  mappedAtCreation: true
})

const range = buffer.getMappedRange()
new Float32Array(range).set(vertices)
buffer.unmap()
```

### Index Buffer

For indexed rendering (triangles, quads).

```javascript
const indices = new Uint16Array([
  0, 1, 2,  // Triangle 1
  0, 2, 3   // Triangle 2
])

const buffer = device.createBuffer({
  size: indices.byteLength,
  usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST,
  mappedAtCreation: true
})

const range = buffer.getMappedRange()
new Uint16Array(range).set(indices)
buffer.unmap()
```

## Copying Buffers

### Buffer to Buffer

```javascript
const srcBuffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC
})

const dstBuffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

const encoder = device.createCommandEncoder()
encoder.copyBufferToBuffer(
  srcBuffer,
  0,         // Source offset
  dstBuffer,
  0,         // Destination offset
  1024       // Size
)

const commandBuffer = encoder.finish()
device.queue.submit([commandBuffer])
```

## Best Practices

### 1. Use Correct Usage Flags

Only specify flags you actually need. Extra flags may hurt performance.

```javascript
// ❌ Don't: Unnecessary flags
const buffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.MAP_WRITE |
         GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST |
         GPUBufferUsage.STORAGE | GPUBufferUsage.UNIFORM
})

// ✅ Do: Only what you need
const buffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})
```

### 2. Prefer mappedAtCreation for Initial Data

```javascript
// ✅ Efficient: One allocation, immediate write
const buffer = device.createBuffer({
  size: dataSize,
  usage: GPUBufferUsage.STORAGE,
  mappedAtCreation: true
})
const range = buffer.getMappedRange()
new Float32Array(range).set(initialData)
buffer.unmap()
```

### 3. Use queueWriteBuffer for Updates

```javascript
// ✅ Simplest way to update buffer contents
device.queue.writeBuffer(buffer, 0, newData.buffer)
```

### 4. Align Uniform Buffer Sizes

Uniform buffers should be multiples of 16 bytes.

```javascript
// ❌ Don't: 13 bytes (unaligned)
const buffer = device.createBuffer({
  size: 13,
  usage: GPUBufferUsage.UNIFORM
})

// ✅ Do: 16 bytes (aligned)
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
})
```

### 5. Always Unmap After getMappedRange

```javascript
// ✅ Proper pattern
const buffer = device.createBuffer({...mappedAtCreation: true})
const range = buffer.getMappedRange()
// ... write to range ...
buffer.unmap()  // MUST call before GPU use

// ❌ Don't forget to unmap!
// Buffer won't be usable by GPU while mapped
```

### 6. Wait for GPU Before Reading

```javascript
// ✅ Correct: Wait for GPU to finish
device.queue.submit([commandBuffer])
await device.queue.onSubmittedWorkDone()  // Wait
await buffer.mapAsync('READ')
// Now safe to read

// ❌ Don't: Read before GPU finishes
device.queue.submit([commandBuffer])
await buffer.mapAsync('READ')  // May read stale data!
```

## Complete Example

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

async function bufferExample() {
  // Setup
  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  // Create buffer with initial data
  const buffer = device.createBuffer({
    size: 16,  // 4 floats
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    mappedAtCreation: true
  })

  // Write initial data
  const arrayBuffer = buffer.getMappedRange()
  const view = new Float32Array(arrayBuffer)
  view.set([1.0, 2.0, 3.0, 4.0])
  buffer.unmap()

  console.log('Buffer state:', buffer.mapState())  // "unmapped"

  // Update with queueWriteBuffer
  const newData = new Float32Array([5.0, 6.0, 7.0, 8.0])
  device.queue.writeBuffer(buffer, 0, newData.buffer)

  // Wait for GPU
  await device.queue.onSubmittedWorkDone()

  // Read back
  console.log('Mapping for read...')
  await buffer.mapAsync('READ')
  console.log('Buffer state:', buffer.mapState())  // "mapped"

  const readBuffer = buffer.getMappedRange()
  const readView = new Float32Array(readBuffer)
  console.log('Data:', Array.from(readView))
  // Output: Data: [5, 6, 7, 8]

  buffer.unmap()
  console.log('Buffer state:', buffer.mapState())  // "unmapped"

  // Cleanup
  buffer.destroy()
}

bufferExample()
```

## TypeScript

```typescript
import { Gpu, GPUBuffer, GPUBufferUsage } from '@sylphx/webgpu'

const buffer: GPUBuffer = device.createBuffer({
  size: 1024,
  usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
  mappedAtCreation: false
})

// Type-safe usage checks
const usage: number = buffer.usage()
const hasStorage: boolean = (usage & GPUBufferUsage.STORAGE) !== 0

// Type-safe state checks
const state: 'unmapped' | 'pending' | 'mapped' = buffer.mapState()
```

## Troubleshooting

### Error: "Buffer must be mapped before calling getMappedRange()"

**Cause:** Called `getMappedRange()` when buffer is not in "mapped" state.

**Solution:**
```javascript
// ✅ Ensure buffer is mapped first
if (descriptor.mappedAtCreation) {
  // Can call immediately
  const range = buffer.getMappedRange()
} else {
  // Must call mapAsync first
  await buffer.mapAsync('READ')
  const range = buffer.getMappedRange()
}
```

### Error: "Offset must be a multiple of 8"

**Cause:** `offset` parameter not aligned to 8 bytes.

**Solution:**
```javascript
// ✅ Use aligned offsets
buffer.getMappedRange(0, 16)
buffer.getMappedRange(8, 16)
buffer.getMappedRange(16, 16)
```

### Error: "getMappedRange() range overlaps with existing range"

**Cause:** Called `getMappedRange()` with overlapping ranges.

**Solution:**
```javascript
// ❌ Overlapping
const range1 = buffer.getMappedRange(0, 32)
const range2 = buffer.getMappedRange(16, 32)  // Overlaps!

// ✅ Non-overlapping
const range1 = buffer.getMappedRange(0, 32)
const range2 = buffer.getMappedRange(32, 32)  // OK
```

## See Also

- [GPUDevice](/api/device) - Creating and managing buffers
- [GPUQueue](/api/queue) - Writing buffer data
- [Buffer Guide](/guide/buffers) - Detailed buffer usage guide
- [Compute Shaders](/guide/compute) - Using buffers in compute
- [Rendering](/guide/rendering) - Using vertex and index buffers
