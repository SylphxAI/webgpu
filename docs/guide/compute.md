# Compute Shaders

Compute shaders enable general-purpose GPU programming (GPGPU) for parallel computation.

## Basic Compute Shader

```wgsl
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;

  if (index >= arrayLength(&output)) {
    return;
  }

  output[index] = input[index] * 2.0;
}
```

## Workgroups

Compute work is divided into workgroups, which execute in parallel.

### Workgroup Size

```wgsl
@compute @workgroup_size(64)      // 64 threads per workgroup
@compute @workgroup_size(8, 8)    // 8x8 = 64 threads
@compute @workgroup_size(4, 4, 4) // 4x4x4 = 64 threads
```

### Dispatching Workgroups

```javascript
const arrayLength = 1000
const workgroupSize = 64
const numWorkgroups = Math.ceil(arrayLength / workgroupSize)

pass.dispatchWorkgroups(numWorkgroups, 1, 1)
```

### Invocation IDs

```wgsl
@compute @workgroup_size(64)
fn main(
  @builtin(global_invocation_id) global_id: vec3<u32>,
  @builtin(local_invocation_id) local_id: vec3<u32>,
  @builtin(workgroup_id) workgroup_id: vec3<u32>,
  @builtin(num_workgroups) num_workgroups: vec3<u32>
) {
  // global_id = workgroup_id * workgroup_size + local_id
  let index = global_id.x;
}
```

## Vector Addition Example

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

async function vectorAdd() {
  // Setup
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
  const device = adapter.requestDevice()

  // Data
  const length = 1000
  const a = new Float32Array(length).fill(1.0)
  const b = new Float32Array(length).fill(2.0)

  // Buffers
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

  device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
  device.queueWriteBuffer(bufferB, 0, Buffer.from(b.buffer))

  // Shader
  const shader = device.createShaderModule(`
    @group(0) @binding(0) var<storage, read> a: array<f32>;
    @group(0) @binding(1) var<storage, read> b: array<f32>;
    @group(0) @binding(2) var<storage, read_write> result: array<f32>;

    @compute @workgroup_size(64)
    fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
      let index = global_id.x;
      if (index >= arrayLength(&result)) {
        return;
      }
      result[index] = a[index] + b[index];
    }
  `)

  // Pipeline
  const pipeline = device.createComputePipeline({
    compute: { module: shader, entryPoint: 'main' }
  })

  // Bind group
  const bindGroup = device.createBindGroup({
    layout: pipeline.getBindGroupLayout(0),
    entries: [
      { binding: 0, resource: { buffer: bufferA } },
      { binding: 1, resource: { buffer: bufferB } },
      { binding: 2, resource: { buffer: bufferResult } }
    ]
  })

  // Dispatch
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginComputePass()
  pass.setPipeline(pipeline)
  pass.setBindGroup(0, bindGroup)
  pass.dispatchWorkgroups(Math.ceil(length / 64), 1, 1)
  pass.end()

  device.queueSubmit(encoder.finish())
  device.poll(true)

  console.log('✅ Computed: 1000 elements, result should be 3.0 for each')
}

vectorAdd()
```

## Matrix Multiplication

```wgsl
@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> result: array<f32>;

struct Dims {
  m: u32,
  n: u32,
  k: u32
}
@group(0) @binding(3) var<uniform> dims: Dims;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let row = global_id.y;
  let col = global_id.x;

  if (row >= dims.m || col >= dims.n) {
    return;
  }

  var sum = 0.0;
  for (var i = 0u; i < dims.k; i++) {
    sum += a[row * dims.k + i] * b[i * dims.n + col];
  }

  result[row * dims.n + col] = sum;
}
```

Dispatch for M×N matrix:

```javascript
pass.dispatchWorkgroups(
  Math.ceil(n / 8),  // columns
  Math.ceil(m / 8),  // rows
  1
)
```

## Image Processing

Blur filter example:

```wgsl
@group(0) @binding(0) var inputTexture: texture_2d<f32>;
@group(0) @binding(1) var outputTexture: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let dims = textureDimensions(inputTexture);
  let coords = vec2<i32>(global_id.xy);

  if (coords.x >= i32(dims.x) || coords.y >= i32(dims.y)) {
    return;
  }

  // 3x3 box blur
  var color = vec4<f32>(0.0);
  for (var y = -1; y <= 1; y++) {
    for (var x = -1; x <= 1; x++) {
      let sample_coords = coords + vec2(x, y);
      color += textureLoad(inputTexture, sample_coords, 0);
    }
  }
  color /= 9.0;

  textureStore(outputTexture, coords, color);
}
```

## Particle Simulation

```wgsl
struct Particle {
  position: vec2<f32>,
  velocity: vec2<f32>
}

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> deltaTime: f32;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;

  if (index >= arrayLength(&particles)) {
    return;
  }

  var particle = particles[index];

  // Apply gravity
  particle.velocity.y += -9.8 * deltaTime;

  // Update position
  particle.position += particle.velocity * deltaTime;

  // Bounce off ground
  if (particle.position.y < 0.0) {
    particle.position.y = 0.0;
    particle.velocity.y *= -0.8;  // 80% bounce
  }

  particles[index] = particle;
}
```

## Shared Memory (Workgroup Memory)

Shared memory is faster but limited per workgroup:

```wgsl
var<workgroup> shared_data: array<f32, 64>;

@compute @workgroup_size(64)
fn main(
  @builtin(local_invocation_id) local_id: vec3<u32>,
  @builtin(global_invocation_id) global_id: vec3<u32>
) {
  // Load into shared memory
  shared_data[local_id.x] = input[global_id.x];

  // Synchronize workgroup
  workgroupBarrier();

  // Access shared data (all threads see same data)
  let value = shared_data[local_id.x];

  // Process...
}
```

### Parallel Reduction

Sum all values in shared memory:

```wgsl
var<workgroup> shared: array<f32, 256>;

@compute @workgroup_size(256)
fn reduce(@builtin(local_invocation_id) local_id: vec3<u32>) {
  let tid = local_id.x;

  // Load data
  shared[tid] = input[global_id.x];
  workgroupBarrier();

  // Reduce
  for (var s = 128u; s > 0u; s >>= 1u) {
    if (tid < s) {
      shared[tid] += shared[tid + s];
    }
    workgroupBarrier();
  }

  // Thread 0 has sum
  if (tid == 0u) {
    output[workgroup_id.x] = shared[0];
  }
}
```

## Performance Tips

### 1. Choose Optimal Workgroup Size

```wgsl
// ✅ Good: Power of 2, typical values
@compute @workgroup_size(64)
@compute @workgroup_size(256)

// ⚠️ Suboptimal: Too small
@compute @workgroup_size(8)

// ⚠️ Suboptimal: Too large
@compute @workgroup_size(1024)  // May exceed GPU limits
```

### 2. Coalesce Memory Access

```wgsl
// ✅ Do: Sequential access (coalesced)
@compute @workgroup_size(64)
fn good(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;
  output[index] = input[index];
}

// ❌ Don't: Random access (slower)
@compute @workgroup_size(64)
fn bad(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x * 17u + 42u;  // Random pattern
  output[index] = input[index];
}
```

### 3. Minimize Divergence

```wgsl
// ❌ Don't: Divergent branches
if (index % 2u == 0u) {
  // Half threads do this
} else {
  // Other half do different work
}

// ✅ Do: Uniform work
// All threads do same work
```

### 4. Use Shared Memory for Repeated Access

```wgsl
// ✅ Do: Load once into shared memory
var<workgroup> cached: array<f32, 64>;

@compute @workgroup_size(64)
fn main(@builtin(local_invocation_id) local_id: vec3<u32>) {
  cached[local_id.x] = input[global_id.x];
  workgroupBarrier();

  // Reuse from fast shared memory
  let value = cached[local_id.x];
  // ... multiple uses
}
```

### 5. Bounds Checking

```wgsl
// ✅ Always check bounds
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;

  if (index >= arrayLength(&output)) {
    return;  // Early exit
  }

  // Safe to access
  output[index] = compute(index);
}
```

## Debugging

### 1. Output Debug Values

```wgsl
@group(0) @binding(0) var<storage, read_write> debug: array<f32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let value = compute_something();
  debug[global_id.x] = value;  // Write for inspection
}
```

### 2. Start Simple

```javascript
// Test with small data first
const length = 10  // Not 1,000,000
const workgroupSize = 1  // Not 256

// Verify results match CPU
const gpuResult = await runOnGPU()
const cpuResult = runOnCPU()
console.assert(arraysEqual(gpuResult, cpuResult))
```

### 3. Check Dispatch Size

```javascript
console.log(`Array length: ${length}`)
console.log(`Workgroup size: ${workgroupSize}`)
console.log(`Num workgroups: ${Math.ceil(length / workgroupSize)}`)
```

## Complete Example

See [Compute Example](/examples/compute) for a full working example.

## Next Steps

- Learn about [Rendering](/guide/rendering) →
- Explore [Performance](/guide/performance) →
- See [Examples](/examples/) →
