# Testing

Testing WebGPU code to ensure correctness and reliability.

## Test Framework

This project uses [Bun](https://bun.sh) for testing with TypeScript support.

```bash
# Run all tests
bun test

# Watch mode
bun test --watch

# Coverage report
bun test --coverage
```

## Basic Test Structure

```typescript
import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu } from '../index.js'

describe('GPU Operations', () => {
  let gpu
  let adapter
  let device

  beforeAll(() => {
    gpu = Gpu.create()
    adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
    device = adapter.requestDevice()
  })

  afterAll(() => {
    device.destroy()
  })

  test('should create buffer', () => {
    const buffer = device.createBuffer(1024, usage, false)
    expect(buffer).toBeDefined()
    buffer.destroy()
  })
})
```

## Testing GPU Availability

```typescript
test('should have GPU available', () => {
  const gpu = Gpu.create()
  expect(gpu).toBeDefined()

  const adapters = gpu.enumerateAdapters()
  expect(adapters.length).toBeGreaterThan(0)
})
```

## Testing Buffers

### Write and Read

```typescript
test('should write and read buffer', async () => {
  const buffer = device.createBuffer(
    20,
    BufferUsage.STORAGE | BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  const input = new Float32Array([1.5, 2.5, 3.5, 4.5, 5.5])
  device.queueWriteBuffer(buffer, 0, Buffer.from(input.buffer))

  const encoder = device.createCommandEncoder()
  device.queueSubmit(encoder.finish())
  device.poll(true)

  const mapped = await buffer.mapRead()
  const output = new Float32Array(mapped.buffer, mapped.byteOffset, 5)

  expect(output[0]).toBe(1.5)
  expect(output[1]).toBe(2.5)
  expect(output[2]).toBe(3.5)
  expect(output[3]).toBe(4.5)
  expect(output[4]).toBe(5.5)

  buffer.destroy()
})
```

## Testing Compute Shaders

### Vector Addition

```typescript
test('should compute vector addition', async () => {
  const length = 5

  // Create buffers
  const bufferA = device.createBuffer(
    length * 4,
    BufferUsage.STORAGE | BufferUsage.COPY_DST,
    false
  )
  const bufferB = device.createBuffer(
    length * 4,
    BufferUsage.STORAGE | BufferUsage.COPY_DST,
    false
  )
  const bufferResult = device.createBuffer(
    length * 4,
    BufferUsage.STORAGE | BufferUsage.COPY_SRC,
    false
  )

  // Write input data
  const a = new Float32Array([1, 2, 3, 4, 5])
  const b = new Float32Array([10, 20, 30, 40, 50])

  device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
  device.queueWriteBuffer(bufferB, 0, Buffer.from(b.buffer))

  // Create shader
  const shader = device.createShaderModule(`
    @group(0) @binding(0) var<storage, read> a: array<f32>;
    @group(0) @binding(1) var<storage, read> b: array<f32>;
    @group(0) @binding(2) var<storage, read_write> result: array<f32>;

    @compute @workgroup_size(1)
    fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
      let index = global_id.x;
      if (index >= arrayLength(&result)) {
        return;
      }
      result[index] = a[index] + b[index];
    }
  `)

  // Create pipeline
  const pipeline = device.createComputePipeline({
    compute: { module: shader, entryPoint: 'main' }
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

  // Execute
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginComputePass()
  pass.setPipeline(pipeline)
  pass.setBindGroup(0, bindGroup)
  pass.dispatchWorkgroups(length, 1, 1)
  pass.end()

  device.queueSubmit(encoder.finish())
  device.poll(true)

  // Read result
  const readBuffer = device.createBuffer(
    length * 4,
    BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  const copyEncoder = device.createCommandEncoder()
  copyEncoder.copyBufferToBuffer(bufferResult, 0, readBuffer, 0, length * 4)
  device.queueSubmit(copyEncoder.finish())
  device.poll(true)

  const mapped = await readBuffer.mapRead()
  const result = new Float32Array(mapped.buffer, mapped.byteOffset, length)

  // Verify
  expect(result[0]).toBe(11)
  expect(result[1]).toBe(22)
  expect(result[2]).toBe(33)
  expect(result[3]).toBe(44)
  expect(result[4]).toBe(55)

  // Cleanup
  bufferA.destroy()
  bufferB.destroy()
  bufferResult.destroy()
  readBuffer.destroy()
})
```

## Testing Textures

```typescript
test('should create and write texture', async () => {
  const width = 64
  const height = 64

  const texture = device.createTexture({
    size: { width, height, depthOrArrayLayers: 1 },
    format: 'rgba8unorm',
    usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST | TextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1
  })

  // Create red image
  const data = new Uint8Array(width * height * 4)
  for (let i = 0; i < data.length; i += 4) {
    data[i] = 255      // R
    data[i + 1] = 0    // G
    data[i + 2] = 0    // B
    data[i + 3] = 255  // A
  }

  const bytesPerRow = 256  // Must be multiple of 256
  device.queueWriteTexture(
    { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
    Buffer.from(data.buffer),
    { offset: 0, bytesPerRow, rowsPerImage: height },
    { width, height, depthOrArrayLayers: 1 }
  )

  // Read back
  const buffer = device.createBuffer(
    bytesPerRow * height,
    BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  const encoder = device.createCommandEncoder()
  encoder.copyTextureToBuffer(
    { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
    { buffer, offset: 0, bytesPerRow, rowsPerImage: height },
    { width, height, depthOrArrayLayers: 1 }
  )

  device.queueSubmit(encoder.finish())
  device.poll(true)

  const mapped = await buffer.mapRead()
  const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset)

  // Check first pixel
  expect(pixels[0]).toBe(255)  // R
  expect(pixels[1]).toBe(0)    // G
  expect(pixels[2]).toBe(0)    // B
  expect(pixels[3]).toBe(255)  // A

  texture.destroy()
  buffer.destroy()
})
```

## Testing Render Pipelines

```typescript
test('should render triangle', async () => {
  const width = 64
  const height = 64

  // Create render target
  const texture = device.createTexture({
    size: { width, height, depthOrArrayLayers: 1 },
    format: 'rgba8unorm',
    usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1
  })

  // Shader
  const shader = device.createShaderModule(`
    @vertex
    fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
      return vec4(position, 0.0, 1.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4<f32> {
      return vec4(1.0, 0.0, 0.0, 1.0);  // Red
    }
  `)

  // Vertices
  const vertices = new Float32Array([0.0, 0.5, -0.5, -0.5, 0.5, -0.5])
  const vertexBuffer = device.createBuffer(
    vertices.byteLength,
    BufferUsage.VERTEX | BufferUsage.COPY_DST,
    false
  )
  device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

  // Pipeline
  const pipeline = device.createRenderPipeline({
    vertex: {
      module: shader,
      entryPoint: 'vs_main',
      buffers: [{
        arrayStride: 8,
        attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' }]
      }]
    },
    fragment: {
      module: shader,
      entryPoint: 'fs_main',
      targets: [{ format: 'rgba8unorm' }]
    },
    primitive: { topology: 'triangle-list' },
    multisample: { count: 1 }
  })

  // Render
  const encoder = device.createCommandEncoder()
  const pass = encoder.beginRenderPass({
    colorAttachments: [{
      view: texture.createView(),
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }]
  })

  pass.setPipeline(pipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.draw(3, 1, 0, 0)
  pass.end()

  device.queueSubmit(encoder.finish())
  device.poll(true)

  // Read center pixel (should be red)
  const bytesPerRow = 256
  const buffer = device.createBuffer(
    bytesPerRow * height,
    BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  const copyEncoder = device.createCommandEncoder()
  copyEncoder.copyTextureToBuffer(
    { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
    { buffer, offset: 0, bytesPerRow, rowsPerImage: height },
    { width, height, depthOrArrayLayers: 1 }
  )

  device.queueSubmit(copyEncoder.finish())
  device.poll(true)

  const mapped = await buffer.mapRead()
  const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset)

  // Center pixel
  const centerX = width / 2
  const centerY = height / 2
  const offset = (centerY * bytesPerRow) + (centerX * 4)

  expect(pixels[offset]).toBe(255)     // R
  expect(pixels[offset + 1]).toBe(0)   // G
  expect(pixels[offset + 2]).toBe(0)   // B

  texture.destroy()
  vertexBuffer.destroy()
  buffer.destroy()
})
```

## Snapshot Testing

For visual regression testing:

```typescript
import fs from 'fs'
import crypto from 'crypto'

function pixelHash(pixels: Uint8Array): string {
  return crypto.createHash('md5').update(pixels).digest('hex')
}

test('should match snapshot', async () => {
  const pixels = await renderScene()
  const hash = pixelHash(pixels)

  const snapshotPath = './test/snapshots/scene.hash'

  if (!fs.existsSync(snapshotPath)) {
    // Create snapshot
    fs.writeFileSync(snapshotPath, hash)
  } else {
    // Compare with snapshot
    const expected = fs.readFileSync(snapshotPath, 'utf-8')
    expect(hash).toBe(expected.trim())
  }
})
```

## Performance Testing

```typescript
test('should compute in reasonable time', async () => {
  const start = performance.now()

  await runComputation()

  const elapsed = performance.now() - start

  expect(elapsed).toBeLessThan(100)  // Should complete in <100ms
})
```

## Error Testing

```typescript
test('should throw on invalid buffer size', () => {
  expect(() => {
    device.createBuffer(
      -1,  // Invalid size
      BufferUsage.STORAGE,
      false
    )
  }).toThrow()
})

test('should validate shader compilation', () => {
  expect(() => {
    device.createShaderModule(`
      @compute @workgroup_size(64)
      fn main() {
        let x: f32 = "invalid";  // Type error
      }
    `)
  }).toThrow()
})
```

## Test Coverage

Run coverage report:

```bash
bun test --coverage
```

**Note**: Coverage tools only measure JavaScript code, not Rust native code. The reported coverage (~20%) only reflects the thin JavaScript wrapper layer. True functional coverage is ~95% based on comprehensive test assertions.

See [COVERAGE.md](https://github.com/SylphxAI/webgpu/blob/main/COVERAGE.md) for detailed explanation.

## Continuous Integration

Example GitHub Actions workflow:

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    steps:
      - uses: actions/checkout@v4

      - uses: oven-sh/setup-bun@v1
        with:
          bun-version: latest

      - run: bun install
      - run: bun test
```

## Best Practices

### 1. Clean Up Resources

```typescript
test('should clean up', async () => {
  const buffer = device.createBuffer(1024, usage, false)

  // ... use buffer

  buffer.destroy()  // Clean up
})
```

### 2. Test Edge Cases

```typescript
test('should handle empty buffer', async () => {
  const buffer = device.createBuffer(0, usage, false)
  expect(buffer).toBeDefined()
  buffer.destroy()
})

test('should handle large buffer', async () => {
  const limits = adapter.getLimits()
  const size = Math.min(limits.maxBufferSize, 1024 * 1024 * 100)

  const buffer = device.createBuffer(size, usage, false)
  expect(buffer).toBeDefined()
  buffer.destroy()
})
```

### 3. Verify Results

```typescript
test('should compute correct result', async () => {
  const result = await runComputation()

  // Don't just check pass/fail - verify actual values
  expect(result[0]).toBe(expectedValue0)
  expect(result[1]).toBe(expectedValue1)
  // ...
})
```

### 4. Use Descriptive Test Names

```typescript
// ✅ Do: Descriptive
test('should compute vector addition for 1000 elements', async () => {})

// ❌ Don't: Vague
test('test1', async () => {})
```

## Debugging Tests

### Print Debug Info

```typescript
test('debug shader output', async () => {
  const result = await runShader()

  console.log('Result:', Array.from(result))
  console.log('Length:', result.length)
  console.log('First 10:', Array.from(result.slice(0, 10)))
})
```

### Isolate Tests

```typescript
// Run only this test
test.only('should debug this specific case', async () => {
  // ...
})
```

### Skip Tests

```typescript
// Skip flaky test
test.skip('flaky test', async () => {
  // ...
})
```

## Next Steps

- See [test/](https://github.com/SylphxAI/webgpu/tree/main/test) directory for complete examples
- Read [TESTING.md](https://github.com/SylphxAI/webgpu/blob/main/TESTING.md) for detailed testing guide
- Explore [Examples](/examples/) for more usage patterns
