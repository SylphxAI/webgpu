# Performance

Optimization strategies for getting the most out of your GPU.

## Profiling

### Measure GPU Time

```javascript
const start = Date.now()

device.queueSubmit(encoder.finish())
device.poll(true)  // Wait for GPU to finish

const elapsed = Date.now() - start
console.log(`GPU time: ${elapsed}ms`)
```

### Identify Bottlenecks

```javascript
// Test different workgroup sizes
for (const size of [32, 64, 128, 256]) {
  const start = Date.now()
  runCompute(size)
  const elapsed = Date.now() - start
  console.log(`Workgroup ${size}: ${elapsed}ms`)
}
```

## Buffer Optimization

### 1. Minimize Transfers

```javascript
// ❌ Don't: Transfer every frame
for (let i = 0; i < 1000; i++) {
  device.queueWriteBuffer(buffer, 0, data)
  // ... use buffer
}

// ✅ Do: Transfer once
device.queueWriteBuffer(buffer, 0, data)
for (let i = 0; i < 1000; i++) {
  // ... use buffer
}
```

### 2. Use Appropriate Usage Flags

```javascript
// ❌ Don't: Unnecessary flags
const buffer = device.createBuffer(
  size,
  BufferUsage.STORAGE | BufferUsage.MAP_READ | BufferUsage.MAP_WRITE,
  false
)

// ✅ Do: Only what you need
const buffer = device.createBuffer(
  size,
  BufferUsage.STORAGE,
  false
)
```

### 3. Batch Updates

```javascript
// ❌ Don't: Many small writes
for (const item of items) {
  device.queueWriteBuffer(buffer, item.offset, item.data)
}

// ✅ Do: Single large write
const combinedData = combineData(items)
device.queueWriteBuffer(buffer, 0, combinedData)
```

### 4. Reuse Buffers

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

## Compute Optimization

### 1. Choose Optimal Workgroup Size

Test different sizes to find optimal for your GPU:

```javascript
const sizes = [32, 64, 128, 256]
let bestSize = 64
let bestTime = Infinity

for (const size of sizes) {
  const time = benchmarkWorkgroupSize(size)
  if (time < bestTime) {
    bestTime = time
    bestSize = size
  }
}

console.log(`Optimal workgroup size: ${bestSize}`)
```

### 2. Coalesce Memory Access

```wgsl
// ✅ Do: Sequential access (fast)
@compute @workgroup_size(64)
fn good(@builtin(global_invocation_id) id: vec3<u32>) {
  let index = id.x;
  output[index] = input[index];
}

// ❌ Don't: Strided access (slow)
@compute @workgroup_size(64)
fn bad(@builtin(global_invocation_id) id: vec3<u32>) {
  let index = id.x * 13u;  // Non-sequential
  output[index] = input[index];
}
```

### 3. Use Shared Memory

```wgsl
var<workgroup> shared: array<f32, 256>;

@compute @workgroup_size(256)
fn optimized(@builtin(local_invocation_id) local_id: vec3<u32>) {
  let tid = local_id.x;

  // Load into fast shared memory
  shared[tid] = input[global_id.x];
  workgroupBarrier();

  // Access from shared memory (faster)
  let left = shared[max(tid - 1u, 0u)];
  let right = shared[min(tid + 1u, 255u)];
  let value = shared[tid];

  output[global_id.x] = (left + value + right) / 3.0;
}
```

### 4. Minimize Divergence

```wgsl
// ❌ Don't: Divergent branches
if (id.x % 2u == 0u) {
  // Half threads do expensive work
  output[id.x] = expensiveComputation();
} else {
  // Other half idle
  output[id.x] = 0.0;
}

// ✅ Do: Uniform work
// All threads do same work
output[id.x] = computation(id.x);
```

### 5. Reduce Register Pressure

```wgsl
// ❌ Don't: Too many temporaries
let temp1 = a + b;
let temp2 = c + d;
let temp3 = e + f;
let temp4 = temp1 + temp2;
let temp5 = temp3 + temp4;
return temp5;

// ✅ Do: Reuse variables
var result = a + b;
result += c + d;
result += e + f;
return result;
```

## Render Optimization

### 1. Minimize State Changes

```javascript
// ❌ Don't: Frequent pipeline changes
for (const obj of objects) {
  pass.setPipeline(obj.pipeline)
  pass.setBindGroup(0, obj.bindGroup)
  pass.draw(obj.vertexCount, 1, 0, 0)
}

// ✅ Do: Sort by pipeline
const sorted = sortByPipeline(objects)
for (const [pipeline, objs] of sorted) {
  pass.setPipeline(pipeline)
  for (const obj of objs) {
    pass.setBindGroup(0, obj.bindGroup)
    pass.draw(obj.vertexCount, 1, 0, 0)
  }
}
```

### 2. Use Instancing

```javascript
// ❌ Don't: Many draw calls
for (let i = 0; i < 1000; i++) {
  updateUniforms(i)
  pass.draw(vertexCount, 1, 0, 0)
}

// ✅ Do: Single instanced draw
pass.draw(vertexCount, 1000, 0, 0)
```

### 3. Frustum Culling

```javascript
function isInFrustum(obj, camera) {
  // Check if object is visible
  return camera.frustum.contains(obj.boundingBox)
}

const visible = objects.filter(obj => isInFrustum(obj, camera))

for (const obj of visible) {
  // Only render visible objects
  renderObject(obj)
}
```

### 4. Level of Detail (LOD)

```javascript
function selectLOD(obj, camera) {
  const distance = length(obj.position - camera.position)

  if (distance < 10) return obj.lodHigh
  if (distance < 50) return obj.lodMed
  return obj.lodLow
}

for (const obj of objects) {
  const lod = selectLOD(obj, camera)
  pass.setVertexBuffer(0, lod.buffer)
  pass.draw(lod.vertexCount, 1, 0, 0)
}
```

### 5. Occlusion Culling

```javascript
// Don't render objects behind other objects
const sorted = sortFrontToBack(objects, camera)
const occluded = new Set()

for (const obj of sorted) {
  if (occluded.has(obj)) continue

  renderObject(obj)

  // Mark objects behind this as occluded
  if (obj.isOccluder) {
    for (const other of objectsBehind(obj, camera)) {
      occluded.add(other)
    }
  }
}
```

## Texture Optimization

### 1. Use Appropriate Formats

```javascript
// ❌ Don't: Waste memory
const texture = device.createTexture({
  format: 'rgba32float',  // 16 bytes/pixel - overkill
  // ...
})

// ✅ Do: Use efficient format
const texture = device.createTexture({
  format: 'rgba8unorm',  // 4 bytes/pixel
  // ...
})
```

### 2. Generate Mipmaps

```javascript
// Mipmap generation (conceptual - requires compute shader)
function generateMipmaps(texture) {
  const levels = Math.floor(Math.log2(Math.max(width, height))) + 1

  for (let i = 1; i < levels; i++) {
    // Downsample previous level to current level
    downsampleLevel(texture, i - 1, i)
  }
}
```

### 3. Use Texture Compression

Check for compression support:

```javascript
const features = adapter.getFeatures()

let format = 'rgba8unorm'
if (features.includes('texture-compression-bc')) {
  format = 'bc1-rgba-unorm'  // 4:1 compression
}
```

### 4. Minimize Texture Uploads

```javascript
// ✅ Upload once at startup
const texture = createAndUploadTexture(imageData)

// Reuse in render loop
for (let frame = 0; frame < 1000; frame++) {
  renderWithTexture(texture)
}
```

## Pipeline Optimization

### 1. Cache Pipelines

```javascript
const pipelineCache = new Map()

function getPipeline(config) {
  const key = JSON.stringify(config)

  if (!pipelineCache.has(key)) {
    pipelineCache.set(key, device.createRenderPipeline(config))
  }

  return pipelineCache.get(key)
}
```

### 2. Use Render Bundles

Pre-record static geometry:

```javascript
// Create once
const bundle = createRenderBundle(staticGeometry)

// Reuse every frame
for (let frame = 0; frame < 1000; frame++) {
  const pass = encoder.beginRenderPass({ /* ... */ })
  pass.executeBundles([bundle])
  pass.end()
}
```

## Command Encoding

### 1. Minimize Command Encoders

```javascript
// ❌ Don't: Many encoders
for (const obj of objects) {
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginComputePass()
  // ... render obj
  pass.end()
  device.queueSubmit(encoder.finish())
}

// ✅ Do: Single encoder
const encoder = device.createCommandEncoder()
for (const obj of objects) {
  const pass = encoder.beginComputePass()
  // ... render obj
  pass.end()
}
device.queueSubmit(encoder.finish())
```

### 2. Batch Submissions

```javascript
// ❌ Don't: Submit every iteration
for (let i = 0; i < 100; i++) {
  const encoder = device.createCommandEncoder()
  // ... commands
  device.queueSubmit(encoder.finish())
}

// ✅ Do: Batch submissions
const commands = []
for (let i = 0; i < 100; i++) {
  const encoder = device.createCommandEncoder()
  // ... commands
  commands.push(encoder.finish())
}
device.queueSubmit(...commands)
```

## Memory Management

### 1. Destroy Unused Resources

```javascript
// ✅ Explicit cleanup
const buffer = device.createBuffer(size, usage, false)
// ... use buffer
buffer.destroy()

const texture = device.createTexture({ /* ... */ })
// ... use texture
texture.destroy()
```

### 2. Pool Allocations

```javascript
class BufferPool {
  constructor(device, size, usage) {
    this.device = device
    this.size = size
    this.usage = usage
    this.free = []
    this.used = new Set()
  }

  acquire() {
    if (this.free.length === 0) {
      const buffer = this.device.createBuffer(this.size, this.usage, false)
      this.free.push(buffer)
    }

    const buffer = this.free.pop()
    this.used.add(buffer)
    return buffer
  }

  release(buffer) {
    this.used.delete(buffer)
    this.free.push(buffer)
  }
}

const pool = new BufferPool(device, 1024, BufferUsage.STORAGE)

// Use
const buffer = pool.acquire()
// ... use buffer
pool.release(buffer)
```

## Benchmarking

### Create Benchmark Suite

```javascript
class Benchmark {
  constructor(name) {
    this.name = name
    this.samples = []
  }

  async run(fn, iterations = 10) {
    for (let i = 0; i < iterations; i++) {
      const start = performance.now()
      await fn()
      const elapsed = performance.now() - start
      this.samples.push(elapsed)
    }

    return this.report()
  }

  report() {
    const avg = this.samples.reduce((a, b) => a + b) / this.samples.length
    const min = Math.min(...this.samples)
    const max = Math.max(...this.samples)

    console.log(`${this.name}:`)
    console.log(`  Average: ${avg.toFixed(2)}ms`)
    console.log(`  Min: ${min.toFixed(2)}ms`)
    console.log(`  Max: ${max.toFixed(2)}ms`)

    return { avg, min, max }
  }
}

// Usage
const bench = new Benchmark('Vector Addition')
await bench.run(() => runVectorAdd(), 100)
```

## Platform-Specific Tips

### macOS (Metal)

- Metal backend is highly optimized
- Unified memory architecture benefits buffer access
- Lower CPU overhead than Vulkan

### Linux (Vulkan)

- Vulkan may have higher CPU overhead
- Good multi-threading support
- Test on target hardware

### Windows (DX12/Vulkan)

- DX12 may be faster on Windows
- Vulkan more portable
- Test both backends if available

## Profiling Tools

### Built-in Timing

```javascript
async function profileGPU(name, fn) {
  const start = performance.now()

  await fn()
  device.poll(true)

  const elapsed = performance.now() - start
  console.log(`${name}: ${elapsed.toFixed(2)}ms`)
}

await profileGPU('Compute Shader', async () => {
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginComputePass()
  // ... compute work
  pass.end()
  device.queueSubmit(encoder.finish())
})
```

## Summary

**Key Performance Principles:**

1. **Minimize transfers** between CPU and GPU
2. **Batch operations** to reduce overhead
3. **Reuse resources** instead of recreating
4. **Sort draw calls** to minimize state changes
5. **Use instancing** for repeated geometry
6. **Profile first**, optimize bottlenecks
7. **Test on target hardware** - results vary by GPU

## Next Steps

- Learn about [Testing](/guide/testing) →
- See [Examples](/examples/) →
