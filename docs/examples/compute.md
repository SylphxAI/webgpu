# Compute Example

Vector addition using compute shaders.

## Code

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

async function vectorAdd() {
  // Setup GPU
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
  const device = adapter.requestDevice()

  console.log('Using GPU:', adapter.getInfo().name)

  // Input data
  const length = 1000
  const a = new Float32Array(length).fill(1.0)
  const b = new Float32Array(length).fill(2.0)

  // Create buffers
  const bufferA = device.createBuffer(
    a.byteLength,
    BufferUsage.STORAGE | BufferUsage.COPY_DST,
    false
  )

  const bufferB = device.createBuffer(
    b.byteLength,
    BufferUsage.STORAGE | BufferUsage.COPY_DST,
    false
  )

  const bufferResult = device.createBuffer(
    length * 4,
    BufferUsage.STORAGE | BufferUsage.COPY_SRC,
    false
  )

  // Write input data
  device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
  device.queueWriteBuffer(bufferB, 0, Buffer.from(b.buffer))

  // Create shader
  const shader = device.createShaderModule(`
    @group(0) @binding(0) var<storage, read> a: array<f32>;
    @group(0) @binding(1) var<storage, read> b: array<f32>;
    @group(0) @binding(2) var<storage, read_write> result: array<f32>;

    @compute @workgroup_size(64)
    fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
      let index = global_id.x;

      // Bounds check
      if (index >= arrayLength(&result)) {
        return;
      }

      // Vector addition
      result[index] = a[index] + b[index];
    }
  `, 'Vector Add Shader')

  // Create pipeline
  const pipeline = device.createComputePipeline({
    label: 'Vector Add Pipeline',
    compute: {
      module: shader,
      entryPoint: 'main'
    }
  })

  // Create bind group
  const bindGroup = device.createBindGroup({
    layout: pipeline.getBindGroupLayout(0),
    entries: [
      { binding: 0, resource: { buffer: bufferA } },
      { binding: 1, resource: { buffer: bufferB } },
      { binding: 2, resource: { buffer: bufferResult } }
    ]
  })

  // Encode and execute
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginComputePass({ label: 'Vector Add Pass' })

  pass.setPipeline(pipeline)
  pass.setBindGroup(0, bindGroup)
  pass.dispatchWorkgroups(Math.ceil(length / 64), 1, 1)
  pass.end()

  device.queueSubmit(encoder.finish())
  device.poll(true)

  console.log('✅ Computation complete')
  console.log('   Input A: 1000 elements, all 1.0')
  console.log('   Input B: 1000 elements, all 2.0')
  console.log('   Result:  1000 elements, all 3.0 (expected)')

  // Cleanup
  bufferA.destroy()
  bufferB.destroy()
  bufferResult.destroy()
  device.destroy()
}

vectorAdd().catch(console.error)
```

## Running

```bash
node examples/compute.js
```

## Expected Output

```
Using GPU: Apple M1 Pro
✅ Computation complete
   Input A: 1000 elements, all 1.0
   Input B: 1000 elements, all 2.0
   Result:  1000 elements, all 3.0 (expected)
```

## How It Works

### 1. Setup

Create GPU instance and device:
```javascript
const gpu = Gpu.create()
const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
const device = adapter.requestDevice()
```

### 2. Create Buffers

Three buffers: two inputs (read-only) and one output (read-write):
```javascript
const bufferA = device.createBuffer(
  a.byteLength,
  BufferUsage.STORAGE | BufferUsage.COPY_DST,
  false
)
// bufferB, bufferResult similarly
```

### 3. Upload Data

Write input data to GPU:
```javascript
device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
device.queueWriteBuffer(bufferB, 0, Buffer.from(b.buffer))
```

### 4. Create Shader

WGSL compute shader:
```wgsl
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;
  if (index >= arrayLength(&result)) {
    return;
  }
  result[index] = a[index] + b[index];
}
```

- `@workgroup_size(64)`: 64 threads per workgroup
- `global_id.x`: Global thread ID
- Bounds check prevents out-of-bounds access

### 5. Create Pipeline

Compute pipeline with shader:
```javascript
const pipeline = device.createComputePipeline({
  compute: {
    module: shader,
    entryPoint: 'main'
  }
})
```

### 6. Bind Resources

Bind buffers to shader bindings:
```javascript
const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: { buffer: bufferA } },      // @binding(0)
    { binding: 1, resource: { buffer: bufferB } },      // @binding(1)
    { binding: 2, resource: { buffer: bufferResult } }  // @binding(2)
  ]
})
```

### 7. Execute

Dispatch workgroups:
```javascript
pass.dispatchWorkgroups(Math.ceil(length / 64), 1, 1)
```

- `length = 1000`, `workgroup_size = 64`
- Need `ceil(1000 / 64) = 16` workgroups
- Total threads: `16 * 64 = 1024` (1000 used, 24 exit early)

### 8. Submit and Wait

```javascript
device.queueSubmit(encoder.finish())
device.poll(true)  // Wait for GPU to finish
```

## Variations

### Reading Results

To verify the computation:

```javascript
// Create staging buffer
const readBuffer = device.createBuffer(
  length * 4,
  BufferUsage.COPY_DST | BufferUsage.MAP_READ,
  false
)

// Copy result to staging buffer
const copyEncoder = device.createCommandEncoder()
copyEncoder.copyBufferToBuffer(bufferResult, 0, readBuffer, 0, length * 4)
device.queueSubmit(copyEncoder.finish())
device.poll(true)

// Read data
const mapped = await readBuffer.mapRead()
const result = new Float32Array(mapped.buffer, mapped.byteOffset, length)

console.log('First 10 results:', Array.from(result.slice(0, 10)))
// Output: [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]

readBuffer.destroy()
```

### Different Operation

Matrix multiplication:

```wgsl
@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
  let row = id.y;
  let col = id.x;

  if (row >= M || col >= N) {
    return;
  }

  var sum = 0.0;
  for (var i = 0u; i < K; i++) {
    sum += a[row * K + i] * b[i * N + col];
  }

  result[row * N + col] = sum;
}
```

Dispatch:
```javascript
pass.dispatchWorkgroups(
  Math.ceil(N / 8),  // columns
  Math.ceil(M / 8),  // rows
  1
)
```

## Performance Tips

1. **Choose optimal workgroup size**
   - Powers of 2 (32, 64, 128, 256)
   - Test different sizes for your GPU

2. **Minimize memory transfers**
   - Upload once, compute many times
   - Only download final results

3. **Coalesce memory access**
   - Sequential access pattern is fastest
   - Avoid random access

4. **Use shared memory for repeated access**
   - See [Compute Guide](/guide/compute) for details

## See Also

- [Compute Guide](/guide/compute)
- [Pipeline API](/api/pipeline)
- [More Examples](/examples/)
