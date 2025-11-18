# First Steps

Learn WebGPU fundamentals by building a simple GPU compute program.

## Your First GPU Program

Let's build a program that adds two arrays using the GPU:

```javascript
const { Gpu, bufferUsage } = require('@sylphx/webgpu')

async function vectorAdd() {
  // Setup
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()
  const usage = bufferUsage()

  // Input data
  const a = new Float32Array([1, 2, 3, 4, 5])
  const b = new Float32Array([10, 20, 30, 40, 50])
  const size = a.length * 4 // 5 floats * 4 bytes

  // Create buffers
  const bufferA = device.createBuffer(size, usage.storage | usage.copyDst, false)
  const bufferB = device.createBuffer(size, usage.storage | usage.copyDst, false)
  const bufferResult = device.createBuffer(size, usage.storage | usage.copySrc, false)
  const bufferRead = device.createBuffer(size, usage.copyDst | usage.mapRead, false)

  // Upload data
  device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
  device.queueWriteBuffer(bufferB, 0, Buffer.from(b.buffer))

  // Create shader
  const shader = device.createShaderModule(`
    @group(0) @binding(0) var<storage, read> a: array<f32>;
    @group(0) @binding(1) var<storage, read> b: array<f32>;
    @group(0) @binding(2) var<storage, read_write> result: array<f32>;

    @compute @workgroup_size(1)
    fn main(@builtin(global_invocation_id) id: vec3<u32>) {
      result[id.x] = a[id.x] + b[id.x];
    }
  `)

  // Create pipeline
  const layout = device.createBindGroupLayout({
    entries: [
      { binding: 0, visibility: 4, bufferType: 'read-only-storage' },
      { binding: 1, visibility: 4, bufferType: 'read-only-storage' },
      { binding: 2, visibility: 4, bufferType: 'storage' },
    ],
  })
  const pipelineLayout = device.createPipelineLayout('Layout', [layout])
  const pipeline = device.createComputePipeline('Pipeline', pipelineLayout, shader, 'main')
  const bindGroup = device.createBindGroupBuffers('BindGroup', layout,
    [bufferA, bufferB, bufferResult])

  // Execute
  const encoder = device.createCommandEncoder()
  encoder.computePass(pipeline, [bindGroup], 5, 1, 1) // 5 workgroups
  device.copyBufferToBuffer(encoder, bufferResult, 0, bufferRead, 0, size)
  device.queueSubmit(encoder.finish())
  device.poll(true)

  // Read results
  const data = await bufferRead.mapRead()
  const result = new Float32Array(data.buffer, data.byteOffset, 5)

  console.log('A:      ', Array.from(a))
  console.log('B:      ', Array.from(b))
  console.log('A + B = ', Array.from(result))

  // Cleanup
  bufferRead.unmap()
  bufferA.destroy()
  bufferB.destroy()
  bufferResult.destroy()
  bufferRead.destroy()
  device.destroy()
}

vectorAdd()
```

Output:
```
A:       [ 1, 2, 3, 4, 5 ]
B:       [ 10, 20, 30, 40, 50 ]
A + B =  [ 11, 22, 33, 44, 55 ]
```

## Understanding the Code

### 1. Setup

```javascript
const gpu = Gpu.create()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()
```

- `Gpu.create()` - Creates GPU instance
- `requestAdapter()` - Gets GPU adapter (like getting a handle to your GPU)
- `requestDevice()` - Creates logical device for GPU operations

### 2. Create Buffers

```javascript
const bufferA = device.createBuffer(size, usage.storage | usage.copyDst, false)
```

Buffers are GPU memory. Parameters:
- `size` - Size in bytes
- `usage` - How buffer will be used (bitflags)
- `mappedAtCreation` - Map immediately? (false = no)

### 3. Upload Data

```javascript
device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
```

Copies data from CPU to GPU.

### 4. Create Shader

```wgsl
@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
  result[id.x] = a[id.x] + b[id.x];
}
```

WGSL (WebGPU Shading Language) shader:
- `@compute` - Compute shader
- `@workgroup_size(1)` - 1 thread per workgroup
- `id.x` - Thread index

### 5. Create Pipeline

```javascript
const pipeline = device.createComputePipeline('Pipeline', pipelineLayout, shader, 'main')
```

Pipeline combines shader + layout + configuration.

### 6. Bind Resources

```javascript
const bindGroup = device.createBindGroupBuffers('BindGroup', layout,
  [bufferA, bufferB, bufferResult])
```

Connects buffers to shader bindings.

### 7. Execute

```javascript
encoder.computePass(pipeline, [bindGroup], 5, 1, 1)
```

Runs shader on GPU:
- 5 workgroups in X dimension
- 1 in Y and Z
- Total: 5 threads

### 8. Read Results

```javascript
const data = await bufferRead.mapRead()
const result = new Float32Array(data.buffer, data.byteOffset, 5)
```

Copy data back from GPU to CPU.

## Common Patterns

### Buffer Usage Flags

```javascript
const usage = bufferUsage()

// Read-only storage
usage.storage

// Can write to from CPU
usage.copyDst

// Can read back to CPU
usage.copySrc | usage.mapRead

// Vertex buffer
usage.vertex | usage.copyDst

// Combine flags with |
const flags = usage.storage | usage.copyDst | usage.copySrc
```

### Error Handling

```javascript
try {
  const adapter = await gpu.requestAdapter()
  if (!adapter) {
    throw new Error('No GPU adapter found')
  }
  // ... rest of code
} catch (error) {
  console.error('GPU error:', error)
}
```

### Resource Cleanup

```javascript
// Always clean up!
buffer.destroy()
texture.destroy()
device.destroy()

// Or use a pattern:
try {
  // ... GPU code
} finally {
  buffer?.destroy()
  texture?.destroy()
  device?.destroy()
}
```

## Next Steps

- [Buffers](/guide/buffers) - Deep dive into GPU memory
- [Shaders](/guide/shaders) - Learn WGSL
- [Compute Guide](/guide/compute) - More compute examples
- [Examples](/examples/compute) - See complete examples
