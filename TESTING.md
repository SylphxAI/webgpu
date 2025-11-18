# WebGPU Testing Guide

Complete guide to testing @sylphx/webgpu with Bun's built-in test runner.

---

## 🎯 Quick Start

```bash
# Run all tests
bun test

# Watch mode (auto re-run on changes)
bun test --watch

# Coverage report
bun test --coverage

# Run specific test file
bun test test/compute.test.ts
```

---

## 📊 Test Suite Overview

### Test Statistics

| Metric | Value |
|--------|-------|
| **Total Tests** | 37 |
| **Total Assertions** | 16,538 |
| **Test Files** | 5 TypeScript files |
| **Execution Time** | ~70ms |
| **Pass Rate** | 100% ✅ |

### Module Coverage

```
┌─────────────┬───────┬──────────────────────────────────────┐
│ Module      │ Tests │ Coverage                             │
├─────────────┼───────┼──────────────────────────────────────┤
│ GPU         │   9   │ Instance, adapters, devices          │
│ Buffer      │   6   │ Create, read, write, copy            │
│ Compute     │   6   │ Shaders, pipelines, execution        │
│ Texture     │   9   │ Create, upload, download, samplers   │
│ Render      │   7   │ Pipelines, rendering, MSAA, bundles  │
└─────────────┴───────┴──────────────────────────────────────┘
```

---

## 🧪 Test Files

### 1. test/gpu.test.ts - GPU Instance & Adapter

Tests the foundation of WebGPU API:

```typescript
✅ GPU instance creation
✅ Adapter enumeration
✅ Adapter information (name, vendor, backend)
✅ Adapter features
✅ Adapter limits
✅ Device creation (high-performance, low-power)
```

**Example:**
```typescript
test('should enumerate adapters', () => {
  const gpu = Gpu.create()
  const adapters = gpu.enumerateAdapters()
  expect(adapters.length).toBeGreaterThan(0)
})
```

---

### 2. test/buffer.test.ts - Buffer Operations

Tests GPU memory management:

```typescript
✅ Buffer usage constants
✅ Buffer creation with various flags
✅ Write and read operations
✅ Buffer-to-buffer copies
✅ Vertex, index, uniform, storage buffers
```

**Example:**
```typescript
test('should write and read buffer data', async () => {
  const buffer = device.createBuffer(20, usage.copyDst | usage.mapRead, false)
  const data = new Float32Array([1, 2, 3, 4, 5])

  device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))
  device.poll(true)

  const result = await buffer.mapRead()
  const floats = new Float32Array(result.buffer, result.byteOffset, 5)

  expect(floats[0]).toBe(1.0)
})
```

---

### 3. test/compute.test.ts - Compute Pipelines

Tests GPU compute functionality:

```typescript
✅ Shader module creation
✅ Bind group layouts
✅ Pipeline layouts
✅ Compute pipeline creation
✅ Vector addition (a + b = result)
✅ Multi-workgroup execution
```

**Example:**
```typescript
test('should execute compute shader', async () => {
  const shaderCode = `
    @group(0) @binding(0) var<storage, read> a: array<f32>;
    @group(0) @binding(1) var<storage, read> b: array<f32>;
    @group(0) @binding(2) var<storage, read_write> result: array<f32>;

    @compute @workgroup_size(1)
    fn main(@builtin(global_invocation_id) id: vec3<u32>) {
      result[id.x] = a[id.x] + b[id.x];
    }
  `

  // Create pipeline, execute compute, verify results
  // Input: [1,2,3,4,5] + [10,20,30,40,50]
  // Expected: [11,22,33,44,55]
})
```

---

### 4. test/texture.test.ts - Textures & Samplers

Tests texture operations:

```typescript
✅ Texture usage constants
✅ Texture creation (rgba8unorm, bgra8unorm, depth24plus, etc.)
✅ Texture views
✅ Depth textures
✅ Texture upload (CPU → GPU)
✅ Texture download (GPU → CPU)
✅ Sampler creation (linear, nearest, repeat, clamp)
```

**Example:**
```typescript
test('should upload and download texture', async () => {
  const texture = device.createTexture({
    width: 64, height: 64,
    format: 'rgba8unorm',
    usage: texUsage.copyDst | texUsage.copySrc
  })

  // Upload red texture
  const data = new Uint8Array(64 * 64 * 4)
  for (let i = 0; i < 64 * 64; i++) {
    data[i * 4] = 255     // R
    data[i * 4 + 1] = 0   // G
    data[i * 4 + 2] = 0   // B
    data[i * 4 + 3] = 255 // A
  }

  // Upload and verify...
})
```

---

### 5. test/render.test.ts - Rendering

Tests graphics rendering:

```typescript
✅ Render pipeline creation
✅ Triangle rendering (vertex shader + fragment shader)
✅ Indexed drawing (quads with index buffer)
✅ Blend modes (replace, alpha, additive, premultiplied)
✅ MSAA (1x, 4x anti-aliasing)
✅ Render bundle creation
✅ Render bundle execution
```

**Example:**
```typescript
test('should render triangle', async () => {
  const shaderCode = `
    @vertex
    fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
      return vec4f(pos, 1.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4f {
      return vec4f(1.0, 0.0, 0.0, 1.0); // Red
    }
  `

  // Create pipeline, render, verify center pixel is red
})
```

---

## 🚀 Running Tests

### Basic Commands

```bash
# Run all tests
bun test

# Run with verbose output
bun test --verbose

# Run specific test
bun test test/gpu.test.ts

# Run tests matching pattern
bun test -t "should render"
```

### Watch Mode

Automatically re-run tests when files change:

```bash
bun test --watch
```

### Coverage Report

Generate code coverage report:

```bash
bun test --coverage
```

### npm Scripts

```bash
npm test              # Run all tests (bun test)
npm run test:node     # Legacy Node.js test
npm run test:watch    # Watch mode
npm run test:coverage # Coverage report
```

---

## 🔧 Configuration

### bunfig.toml

```toml
[test]
# Test timeout (30 seconds)
timeout = 30000
```

### package.json

```json
{
  "scripts": {
    "test": "bun test",
    "test:node": "node test/basic.test.js",
    "test:watch": "bun test --watch",
    "test:coverage": "bun test --coverage"
  }
}
```

---

## 📝 Writing Tests

### Basic Structure

```typescript
import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, bufferUsage } from '../index.js'

describe('Feature Name', () => {
  let device: any

  beforeAll(async () => {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should work correctly', () => {
    // Your test
    expect(true).toBe(true)
  })
})
```

### Best Practices

1. **Clean up resources**
   ```typescript
   afterAll(() => {
     buffer?.destroy()
     texture?.destroy()
     device?.destroy()
   })
   ```

2. **Poll after GPU operations**
   ```typescript
   device.queueSubmit(encoder.finish())
   device.poll(true) // Wait for GPU
   ```

3. **Unmap after reading**
   ```typescript
   const data = await buffer.mapRead()
   // Use data...
   buffer.unmap()
   ```

4. **Use proper alignment**
   ```typescript
   // Texture bytes per row must be 256-aligned
   const bytesPerRow = Math.ceil(width * 4 / 256) * 256
   ```

5. **Test both success and failure**
   ```typescript
   test('should handle invalid input', () => {
     expect(() => device.createBuffer(-1, 0, false)).toThrow()
   })
   ```

---

## 🐛 Debugging

### Enable Rust Backtraces

```bash
RUST_BACKTRACE=1 bun test
```

### Run Single Test

```bash
bun test test/compute.test.ts -t "should execute compute shader"
```

### Verbose Output

```bash
bun test --verbose
```

### Check GPU Adapter

```typescript
test('debug adapter info', () => {
  const gpu = Gpu.create()
  const adapters = gpu.enumerateAdapters()
  console.log('Available adapters:', adapters)
})
```

---

## 📊 Test Results Example

```
$ bun test

bun test v1.3.2

test/gpu.test.ts:
 ✓ should create GPU instance
 ✓ should enumerate adapters
 ✓ should request adapter
 ✓ should get adapter info
 ✓ should get adapter features
 ✓ should get adapter limits
 ✓ should request device
 ✓ should request high-performance adapter
 ✓ should request low-power adapter

test/buffer.test.ts:
 ✓ should get buffer usage constants
 ✓ should create buffer
 ✓ should create mapped buffer
 ✓ should write and read buffer data
 ✓ should copy buffer to buffer
 ✓ should handle buffer with different usage flags

test/compute.test.ts:
 ✓ should create shader module
 ✓ should create bind group layout
 ✓ should create pipeline layout
 ✓ should create compute pipeline
 ✓ should execute compute shader (vector addition)
 ✓ should execute compute with multiple workgroups

test/texture.test.ts:
 ✓ should get texture usage constants
 ✓ should create texture
 ✓ should create texture with different formats
 ✓ should create texture view
 ✓ should create depth texture
 ✓ should upload texture data
 ✓ should read texture data
 ✓ should create sampler
 ✓ should create sampler with different modes

test/render.test.ts:
 ✓ should create render pipeline
 ✓ should render triangle
 ✓ should render with indexed drawing
 ✓ should render with blend modes
 ✓ should render with MSAA
 ✓ should create render bundle
 ✓ should execute render bundles

 37 pass
 0 fail
 16538 expect() calls
Ran 37 tests across 5 files. [73.00ms]
```

---

## 🎓 Advanced Topics

### Async GPU Operations

```typescript
test('async GPU work', async () => {
  // Create and submit work
  const encoder = device.createCommandEncoder()
  // ... encode commands ...
  device.queueSubmit(encoder.finish())

  // Wait for GPU
  device.poll(true)

  // Read results
  const data = await buffer.mapRead()
})
```

### Testing Error Conditions

```typescript
test('should handle errors gracefully', () => {
  expect(() => {
    device.createBuffer(0, 0, false) // Invalid size
  }).toThrow()
})
```

### Performance Testing

```typescript
test('should render 1000 triangles quickly', () => {
  const start = performance.now()

  for (let i = 0; i < 1000; i++) {
    // Render triangle
  }

  const elapsed = performance.now() - start
  expect(elapsed).toBeLessThan(1000) // Under 1 second
})
```

---

## 🔗 Resources

- [Bun Test Documentation](https://bun.sh/docs/cli/test)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [wgpu Documentation](https://docs.rs/wgpu/)
- [Test Examples](./test/)

---

## ✅ CI/CD Integration

Tests run automatically on:
- Push to main/master
- Pull requests
- Supports: Ubuntu, macOS, Windows

See `.github/workflows/test.yml` for CI configuration.

---

**All tests passing! 🎉**
