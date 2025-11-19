# WebGPU Test Suite

Complete unit test suite for @sylphx/webgpu using Bun's built-in test runner.

## 📊 Test Coverage

| Module | Tests | Coverage |
|--------|-------|----------|
| **GPU Instance** | 9 tests | GPU creation, adapter enumeration, device requests |
| **Buffer** | 6 tests | Buffer creation, read/write, copy operations |
| **Compute** | 6 tests | Shader compilation, pipelines, compute execution |
| **Texture** | 9 tests | Texture creation, upload/download, samplers |
| **Render** | 7 tests | Render pipelines, triangle rendering, MSAA, bundles |
| **Total** | **37 tests** | **16,538 assertions** |

## 🚀 Running Tests

### Run All Tests
```bash
bun test
```

### Run Specific Test File
```bash
bun test test/gpu.test.ts
bun test test/buffer.test.ts
bun test test/compute.test.ts
bun test test/texture.test.ts
bun test test/render.test.ts
```

### Watch Mode (Auto Re-run)
```bash
bun test --watch
```

### With Coverage
```bash
bun test --coverage
```

### npm Scripts
```bash
npm test              # Run all tests
npm run test:watch    # Watch mode
npm run test:coverage # Coverage report
npm run test:node     # Legacy Node.js test
```

## 📝 Test Structure

### test/gpu.test.ts
Tests for GPU instance and adapter functionality:
- ✅ GPU instance creation
- ✅ Adapter enumeration
- ✅ Adapter information (name, backend, device type)
- ✅ Adapter features and limits
- ✅ Device creation

### test/buffer.test.ts
Tests for buffer operations:
- ✅ Buffer usage constants
- ✅ Buffer creation with various usage flags
- ✅ Write and read buffer data
- ✅ Buffer-to-buffer copy operations
- ✅ Vertex, index, uniform, storage buffers

### test/compute.test.ts
Tests for compute pipelines:
- ✅ Shader module creation
- ✅ Bind group layouts
- ✅ Pipeline layouts
- ✅ Compute pipeline creation
- ✅ Vector addition compute shader
- ✅ Multi-workgroup compute execution

### test/texture.test.ts
Tests for texture and sampler operations:
- ✅ Texture usage constants
- ✅ Texture creation (various formats)
- ✅ Texture views
- ✅ Depth textures
- ✅ Texture upload (buffer → texture)
- ✅ Texture download (texture → buffer)
- ✅ Sampler creation with various modes

### test/render.test.ts
Tests for render pipelines and rendering:
- ✅ Render pipeline creation
- ✅ Triangle rendering
- ✅ Indexed drawing (quads)
- ✅ Blend modes (replace, alpha, additive, premultiplied)
- ✅ MSAA (1x, 4x)
- ✅ Render bundle creation
- ✅ Render bundle execution

## 🎯 Test Examples

### Basic GPU Test
```typescript
import { describe, test, expect } from 'bun:test'
import { Gpu } from '../index.js'

describe('GPU', () => {
  test('should create GPU instance', () => {
    const gpu = Gpu()
    expect(gpu).toBeDefined()
  })
})
```

### Async Test with Device
```typescript
test('should compute on GPU', async () => {
  const gpu = Gpu()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  // Your GPU code here

  device.destroy()
})
```

### Buffer Read/Write Test
```typescript
test('should write and read buffer', async () => {
  const buffer = device.createBuffer(16, usage.copyDst | usage.mapRead, false)

  const data = new Float32Array([1, 2, 3, 4])
  device.queueWriteBuffer(buffer, 0, Buffer.from(data.buffer))

  const encoder = device.createCommandEncoder()
  device.queueSubmit(encoder.finish())
  device.poll(true)

  const result = await buffer.mapRead()
  const floats = new Float32Array(result.buffer, result.byteOffset, 4)

  expect(floats[0]).toBe(1)
  buffer.unmap()
  buffer.destroy()
})
```

## 🔧 Configuration

### bunfig.toml
```toml
[test]
timeout = 30000  # 30 second timeout per test
```

## 📈 Test Results

```
$ bun test

bun test v1.3.2

 37 pass
 0 fail
 16538 expect() calls
Ran 37 tests across 6 files. [61.00ms]
```

## ✨ Features Tested

### Core API
- [x] GPU instance creation
- [x] Adapter enumeration and selection
- [x] Device creation and destruction
- [x] Buffer creation and management
- [x] Texture creation and operations
- [x] Shader module compilation

### Compute
- [x] Compute pipeline creation
- [x] Bind group layouts
- [x] Compute shader execution
- [x] Buffer read/write operations
- [x] Multi-workgroup dispatch

### Rendering
- [x] Render pipeline creation
- [x] Vertex buffer setup
- [x] Index buffer setup
- [x] Triangle rendering
- [x] Texture rendering
- [x] Blend modes
- [x] MSAA anti-aliasing
- [x] Render bundles

### Advanced
- [x] Buffer-to-buffer copies
- [x] Buffer-to-texture copies
- [x] Texture-to-buffer copies
- [x] Command encoder operations
- [x] Queue submission
- [x] GPU polling

## 🐛 Debugging Tests

### Run with Verbose Output
```bash
bun test --verbose
```

### Run Single Test
```bash
bun test test/gpu.test.ts -t "should create GPU instance"
```

### Debug with RUST_BACKTRACE
```bash
RUST_BACKTRACE=1 bun test
```

## 📚 Writing New Tests

### Template
```typescript
import { describe, test, expect, beforeAll, afterAll } from 'bun:test'
import { Gpu, bufferUsage, textureUsage } from '../index.js'

describe('Your Feature', () => {
  let device: any

  beforeAll(async () => {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    device = await adapter.requestDevice()
  })

  afterAll(() => {
    device?.destroy()
  })

  test('should work correctly', () => {
    // Your test code
    expect(true).toBe(true)
  })
})
```

## 🎓 Best Practices

1. **Always destroy resources**: Call `.destroy()` on buffers, textures, etc.
2. **Poll after submission**: Call `device.poll(true)` after `queueSubmit()`
3. **Unmap after reading**: Call `buffer.unmap()` after `mapRead()`
4. **Use proper alignment**: Texture bytes per row must be 256-byte aligned
5. **Test error cases**: Not just happy paths

## 🔗 Related Documentation

- [Bun Test Documentation](https://bun.sh/docs/cli/test)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [wgpu Documentation](https://docs.rs/wgpu/)

## 🎉 Contributing

To add new tests:
1. Create or edit a `.test.ts` file in `/test`
2. Follow the existing patterns
3. Run `bun test` to verify
4. All tests must pass before committing
