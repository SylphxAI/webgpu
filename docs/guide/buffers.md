# Buffers

Buffers are contiguous blocks of GPU memory used to store data like vertices, indices, uniform values, or compute shader results.

## Creating Buffers

```javascript
const buffer = device.createBuffer(
  size,        // Size in bytes
  usage,       // Usage flags
  mappedAtCreation  // Optional: true to get initial write access
)
```

### Usage Flags

Combine flags with bitwise OR (`|`):

```javascript
const { BufferUsage } = require('@sylphx/webgpu')

const usage = BufferUsage.STORAGE | BufferUsage.COPY_SRC
```

| Flag | Description |
|------|-------------|
| `MAP_READ` | Can be mapped for reading |
| `MAP_WRITE` | Can be mapped for writing |
| `COPY_SRC` | Can be copied from |
| `COPY_DST` | Can be copied to |
| `STORAGE` | Can be used in shaders as storage buffer |
| `UNIFORM` | Can be used in shaders as uniform buffer |
| `VERTEX` | Can be used as vertex buffer |
| `INDEX` | Can be used as index buffer |

### Example: Storage Buffer

```javascript
const { BufferUsage } = require('@sylphx/webgpu')

const buffer = device.createBuffer(
  1024,  // 1 KB
  BufferUsage.STORAGE | BufferUsage.COPY_SRC | BufferUsage.COPY_DST,
  false
)
```

## Writing to Buffers

### Queue Write (Easiest)

```javascript
const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queueWriteBuffer(
  buffer,
  0,                           // Offset in buffer
  Buffer.from(data.buffer)     // Data to write
)
```

### Mapped Write (For Initial Data)

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

Buffers must have `MAP_READ` usage and be mapped before reading.

### Async Mapping

```javascript
const buffer = device.createBuffer(
  16,
  BufferUsage.MAP_READ | BufferUsage.COPY_DST,
  false
)

// Write data first
const data = new Float32Array([1.0, 2.0, 3.0, 4.0])
device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))

// Submit and wait for GPU
const encoder = device.createCommandEncoder()
device.queueSubmit(encoder.finish())
device.poll(true)

// Map for reading
const mapped = await buffer.mapRead()
const result = new Float32Array(mapped.buffer, mapped.byteOffset, 4)
console.log('Result:', Array.from(result))
```

## Buffer Types

### Storage Buffer

For read/write access in compute shaders:

```wgsl
@group(0) @binding(0) var<storage, read_write> data: array<f32>;
```

```javascript
const buffer = device.createBuffer(
  1024,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
```

### Uniform Buffer

For constant data in shaders (faster but smaller):

```wgsl
struct Uniforms {
  time: f32,
  resolution: vec2<f32>
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
```

```javascript
const buffer = device.createBuffer(
  16,  // Must be multiple of 16 bytes
  BufferUsage.UNIFORM | BufferUsage.COPY_DST,
  false
)
```

### Vertex Buffer

For vertex data in render pipelines:

```javascript
const vertices = new Float32Array([
  -1.0, -1.0,  // Position 1
   1.0, -1.0,  // Position 2
   0.0,  1.0   // Position 3
])

const buffer = device.createBuffer(
  vertices.byteLength,
  BufferUsage.VERTEX | BufferUsage.COPY_DST,
  false
)

device.queueWriteBuffer(buffer, 0, Buffer.from(vertices.buffer))
```

### Index Buffer

For indexed rendering:

```javascript
const indices = new Uint16Array([0, 1, 2])

const buffer = device.createBuffer(
  indices.byteLength,
  BufferUsage.INDEX | BufferUsage.COPY_DST,
  false
)

device.queueWriteBuffer(buffer, 0, Buffer.from(indices.buffer))
```

## Buffer Copying

```javascript
const srcBuffer = device.createBuffer(
  1024,
  BufferUsage.COPY_SRC,
  false
)

const dstBuffer = device.createBuffer(
  1024,
  BufferUsage.COPY_DST,
  false
)

const encoder = device.createCommandEncoder()
encoder.copyBufferToBuffer(srcBuffer, 0, dstBuffer, 0, 1024)
device.queueSubmit(encoder.finish())
```

## Best Practices

### 1. Choose Correct Usage Flags

Only specify flags you need. Unnecessary flags may reduce performance:

```javascript
// ❌ Don't: Too many flags
const buffer = device.createBuffer(
  1024,
  BufferUsage.MAP_READ | BufferUsage.MAP_WRITE |
  BufferUsage.COPY_SRC | BufferUsage.COPY_DST |
  BufferUsage.STORAGE | BufferUsage.UNIFORM,
  false
)

// ✅ Do: Only what you need
const buffer = device.createBuffer(
  1024,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
```

### 2. Align Buffer Sizes

Uniform buffers must be 16-byte aligned:

```javascript
// ❌ Don't: Unaligned size
const buffer = device.createBuffer(
  13,  // Not multiple of 16
  BufferUsage.UNIFORM,
  false
)

// ✅ Do: Aligned size
const buffer = device.createBuffer(
  16,  // Aligned to 16 bytes
  BufferUsage.UNIFORM,
  false
)
```

### 3. Reuse Buffers

Create buffers once, reuse many times:

```javascript
// ✅ Do: Create once
const buffer = device.createBuffer(1024, usage, false)

// Reuse in loop
for (let i = 0; i < 1000; i++) {
  const data = generateData(i)
  device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))
  // ... use buffer
}
```

### 4. Clean Up

Buffers are automatically freed, but explicit cleanup helps:

```javascript
const buffer = device.createBuffer(1024, usage, false)
// ... use buffer
buffer.destroy()
```

## Complete Example

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

async function bufferExample() {
  // Setup
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
  const device = adapter.requestDevice()

  // Create buffer
  const buffer = device.createBuffer(
    16,
    BufferUsage.STORAGE | BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  // Write data
  const input = new Float32Array([1.0, 2.0, 3.0, 4.0])
  device.queueWriteBuffer(buffer, 0, Buffer.from(input.buffer))

  // Wait for GPU
  const encoder = device.createCommandEncoder()
  device.queueSubmit(encoder.finish())
  device.poll(true)

  // Read data
  const mapped = await buffer.mapRead()
  const output = new Float32Array(mapped.buffer, mapped.byteOffset, 4)
  console.log('Data:', Array.from(output))

  // Cleanup
  buffer.destroy()
  device.destroy()
}

bufferExample()
```

## Next Steps

- Learn about [Textures](/guide/textures) →
- Explore [Compute Shaders](/guide/compute) →
