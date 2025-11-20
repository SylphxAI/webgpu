# @sylphx/webgpu

[![npm version](https://img.shields.io/npm/v/@sylphx/webgpu.svg)](https://www.npmjs.com/package/@sylphx/webgpu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Production-ready WebGPU for Node.js & Bun** - 100% standard-compliant, built with Rust + wgpu

## ✨ What is @sylphx/webgpu?

**The modern, lightweight WebGPU implementation for Node.js.** Use the same WebGPU API in both Node.js and browsers - write once, run everywhere.

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

// Initialize GPU (identical to browser API)
const gpu = Gpu()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()

// Create buffer (100% WebGPU standard)
const buffer = device.createBuffer({
    size: 256,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
})

// Run compute shader
const encoder = device.createCommandEncoder()
const pass = encoder.beginComputePass()
pass.setPipeline(pipeline)
pass.setBindGroup(0, bindGroup)
pass.dispatchWorkgroups(64)
pass.end()
device.queue.submit([encoder.finish()])
```

## 🚀 Why Choose @sylphx/webgpu?

| Feature | @sylphx/webgpu | @kmamal/gpu (Dawn) |
|---------|---------------|-------------------|
| **WebGPU Standard** | ✅ 100% compliant | ⚠️ Custom API |
| **Binary Size** | ~10MB | 50-150MB |
| **Build Time** | 5-15 min | 1-3 hours |
| **Code Portability** | ✅ Browser compatible | ❌ Node.js only |
| **Implementation** | Firefox's wgpu (Rust) | Chrome's Dawn (C++) |
| **Toolchain** | Simple (Cargo) | Complex (depot_tools) |
| **Status** | v1.0 - Production ready | 0.x - Pre-release |

### Key Advantages

✅ **100% WebGPU Standard** - Share code between Node.js and browsers
✅ **Production Ready** - v1.0.1 stable release with 58 tests, 100% pass rate
✅ **Lightweight** - 10MB binaries vs 100MB+ alternatives
✅ **Modern Stack** - Rust + wgpu (used by Firefox, Deno, Bevy)
✅ **Cross-Platform** - 6 prebuilt platforms (macOS, Linux, Windows, ARM64)
✅ **Well Tested** - Comprehensive test suite covering all features

## 📦 Installation

```bash
npm install @sylphx/webgpu
```

**Requirements:**
- Node.js 18+ or Bun 1.0+
- No build tools needed (prebuilt binaries included)

**Supported Platforms:**
- macOS (x64, ARM64/M1/M2/M3)
- Linux (x64, ARM64)
- Windows (x64, ARM64)
- FreeBSD, Android (via source build)

## 🎯 Quick Start

### Basic GPU Setup

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function main() {
    // Create GPU instance
    const gpu = Gpu()

    // Request adapter (automatically selects best GPU)
    const adapter = await gpu.requestAdapter({
        powerPreference: 'high-performance'
    })

    console.log('GPU:', adapter.info.name)
    console.log('Backend:', adapter.info.backend)

    // Request device
    const device = await adapter.requestDevice()

    console.log('✅ WebGPU ready!')
}

main()
```

### Compute Shader Example

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

async function runCompute() {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Create buffers
    const size = 256
    const input = device.createBuffer({
        size,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
    })

    const output = device.createBuffer({
        size,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC
    })

    // Create compute shader (WGSL)
    const shader = device.createShaderModule({
        code: `
            @group(0) @binding(0) var<storage, read> input: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) id: vec3<u32>) {
                output[id.x] = input[id.x] * 2.0;
            }
        `
    })

    // Create bind group layout
    const layout = device.createBindGroupLayout({
        entries: [
            { binding: 0, visibility: 4, buffer: { type: 'read-only-storage' } },
            { binding: 1, visibility: 4, buffer: { type: 'storage' } }
        ]
    })

    // Create bind group
    const bindGroup = device.createBindGroup({
        layout,
        entries: [
            { binding: 0, resource: { buffer: input } },
            { binding: 1, resource: { buffer: output } }
        ]
    })

    // Create pipeline
    const pipeline = device.createComputePipeline({
        layout: device.createPipelineLayout({ bindGroupLayouts: [layout] }),
        compute: { module: shader, entryPoint: 'main' }
    })

    // Execute compute shader
    const encoder = device.createCommandEncoder()
    const pass = encoder.beginComputePass()
    pass.setPipeline(pipeline)
    pass.setBindGroup(0, bindGroup)
    pass.dispatchWorkgroups(4) // 4 * 64 = 256 threads
    pass.end()

    device.queue.submit([encoder.finish()])

    console.log('✅ Compute shader executed!')
}

runCompute()
```

### Render Pipeline Example

```javascript
const { Gpu, GPUBufferUsage, GPUTextureUsage } = require('@sylphx/webgpu')

async function renderTriangle() {
    const gpu = Gpu()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Create vertex buffer
    const vertices = new Float32Array([
        // x,    y,     r,   g,   b
         0.0,  0.5,   1.0, 0.0, 0.0,  // top (red)
        -0.5, -0.5,   0.0, 1.0, 0.0,  // bottom left (green)
         0.5, -0.5,   0.0, 0.0, 1.0   // bottom right (blue)
    ])

    const vertexBuffer = device.createBuffer({
        size: vertices.byteLength,
        usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        mappedAtCreation: true
    })

    new Float32Array(vertexBuffer.getMappedRange()).set(vertices)
    vertexBuffer.unmap()

    // Create shader
    const shader = device.createShaderModule({
        code: `
            struct VertexInput {
                @location(0) position: vec2f,
                @location(1) color: vec3f
            }

            struct VertexOutput {
                @builtin(position) position: vec4f,
                @location(0) color: vec3f
            }

            @vertex
            fn vs_main(in: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                out.position = vec4f(in.position, 0.0, 1.0);
                out.color = in.color;
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4f {
                return vec4f(in.color, 1.0);
            }
        `
    })

    // Create render pipeline
    const pipeline = device.createRenderPipeline({
        layout: 'auto',
        vertex: {
            module: shader,
            entryPoint: 'vs_main',
            buffers: [{
                arrayStride: 20,
                attributes: [
                    { shaderLocation: 0, offset: 0, format: 'float32x2' },
                    { shaderLocation: 1, offset: 8, format: 'float32x3' }
                ]
            }]
        },
        fragment: {
            module: shader,
            entryPoint: 'fs_main',
            targets: [{ format: 'rgba8unorm' }]
        }
    })

    // Create texture for rendering
    const texture = device.createTexture({
        size: { width: 512, height: 512 },
        format: 'rgba8unorm',
        usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC
    })

    // Render triangle
    const encoder = device.createCommandEncoder()
    const pass = encoder.beginRenderPass({
        colorAttachments: [{
            view: texture.createView(),
            loadOp: 'clear',
            storeOp: 'store',
            clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }
        }]
    })

    pass.setPipeline(pipeline)
    pass.setVertexBuffer(0, vertexBuffer)
    pass.draw(3)
    pass.end()

    device.queue.submit([encoder.finish()])

    console.log('✅ Triangle rendered!')
}

renderTriangle()
```

## 📚 Complete Examples

Check the [`examples/`](examples/) directory for more:

- **[basic.js](examples/basic.js)** - GPU setup and adapter info
- **[compute.js](examples/compute.js)** - Vector addition compute shader
- **[triangle.js](examples/triangle.js)** - Render colored triangle
- **[texture-upload.js](examples/texture-upload.js)** - Upload and sample textures
- **[textured-quad.js](examples/textured-quad.js)** - Render textured quad with samplers
- **[cube.js](examples/cube.js)** - 3D cube with depth testing
- **[transparency.js](examples/transparency.js)** - Alpha blending
- **[msaa.js](examples/msaa.js)** - Multi-sample anti-aliasing
- **[mrt.js](examples/mrt.js)** - Multiple render targets (G-buffer)
- **[indirect-draw.js](examples/indirect-draw.js)** - GPU-driven rendering
- **[indirect-compute.js](examples/indirect-compute.js)** - GPU-driven compute
- **[render-bundle.js](examples/render-bundle.js)** - Reusable render bundles
- **[timestamp-queries.js](examples/timestamp-queries.js)** - GPU performance profiling

Run any example:
```bash
node examples/compute.js
# or with Bun (2x faster startup!)
bun examples/compute.js
```

## 🎓 API Documentation

### Full WebGPU Standard API

The API is 100% compliant with the [W3C WebGPU specification](https://gpuweb.github.io/gpuweb/). Code written for browsers works identically in Node.js.

**Core Objects:**
- `Gpu` - Entry point (equivalent to `navigator.gpu`)
- `GPUAdapter` - Physical GPU representation
- `GPUDevice` - Logical device for GPU operations
- `GPUBuffer` - GPU memory buffer
- `GPUTexture` - GPU texture (images)
- `GPUSampler` - Texture sampling configuration
- `GPUShaderModule` - Compiled WGSL shader
- `GPUBindGroup` - Resource bindings
- `GPUPipelineLayout` - Pipeline resource layout
- `GPUComputePipeline` - Compute shader pipeline
- `GPURenderPipeline` - Render pipeline
- `GPUCommandEncoder` - Command recording
- `GPUComputePassEncoder` - Compute pass recording
- `GPURenderPassEncoder` - Render pass recording
- `GPUQueue` - Command submission queue

**Constants (WebGPU Standard):**
```javascript
const {
    GPUBufferUsage,    // Buffer usage flags
    GPUTextureUsage,   // Texture usage flags
    GPUMapMode,        // Buffer map modes
    GPUShaderStage     // Shader stage flags
} = require('@sylphx/webgpu')
```

### Key Differences from Browser

**Entry Point:**
```javascript
// Browser
const adapter = await navigator.gpu.requestAdapter()

// Node.js (@sylphx/webgpu)
const { Gpu } = require('@sylphx/webgpu')
const gpu = Gpu()
const adapter = await gpu.requestAdapter()
```

**Everything else is identical!** All methods, properties, and descriptors match the browser API exactly.

## 🌐 Browser Compatibility

Share code between Node.js and browsers:

```javascript
// Universal WebGPU code (works in both!)
export async function initGPU() {
    // Detect environment
    const gpu = typeof navigator !== 'undefined'
        ? navigator.gpu
        : require('@sylphx/webgpu').Gpu()

    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // All code below is identical!
    const buffer = device.createBuffer({
        size: 256,
        usage: GPUBufferUsage.STORAGE
    })

    return { device, buffer }
}
```

## 🔧 Advanced Features

### GPU Profiling with Timestamp Queries

```javascript
const querySet = device.createQuerySet({
    type: 'timestamp',
    count: 2
})

const encoder = device.createCommandEncoder()
encoder.writeTimestamp(querySet, 0)

// ... GPU work ...

encoder.writeTimestamp(querySet, 1)
device.queue.submit([encoder.finish()])

// Read timing results
const timings = await readTimestamps(querySet)
console.log(`GPU time: ${(timings[1] - timings[0]) / 1e6}ms`)
```

### Indirect Draw (GPU-Driven Rendering)

```javascript
// GPU generates its own draw commands
const indirectBuffer = device.createBuffer({
    size: 20,
    usage: GPUBufferUsage.INDIRECT | GPUBufferUsage.STORAGE
})

// Compute shader writes draw commands
// Render pass reads from buffer
pass.drawIndirect(indirectBuffer, 0)
```

### Multiple Render Targets (Deferred Rendering)

```javascript
const pass = encoder.beginRenderPass({
    colorAttachments: [
        { view: positionTexture.createView(), ... },   // G-buffer position
        { view: normalTexture.createView(), ... },     // G-buffer normal
        { view: albedoTexture.createView(), ... }      // G-buffer albedo
    ]
})
```

## 🧪 Testing

```bash
# Run all tests
npm test

# Watch mode
npm run test:watch

# Coverage report
npm run test:coverage
```

**Test Suite:**
- 58 comprehensive tests
- 100% pass rate
- Covers all WebGPU features
- Real GPU operations (not mocked)

## 🏗️ Building from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/SylphxAI/webgpu.git
cd webgpu

# Install dependencies
npm install

# Build native bindings (5-15 minutes)
npm run build

# Run tests
npm test
```

## 📊 Performance

**Binary Size:**
- @sylphx/webgpu: ~10MB (optimized release)
- @kmamal/gpu: 50-150MB (Dawn binaries)

**Build Time:**
- @sylphx/webgpu: 5-15 minutes (Cargo)
- @kmamal/gpu: 1-3 hours (Dawn + depot_tools)

**Runtime Performance:**
- GPU operations: Zero overhead (thin wrapper)
- CPU overhead: <10% for descriptor transformation
- Compute/Render: Limited by GPU, not bindings

## 🛠️ Architecture

```
User Code (WebGPU Standard API)
    ↓
webgpu.js (JavaScript wrapper - transforms descriptors)
    ↓
index.js (napi-rs native bindings)
    ↓
Rust (wgpu implementation)
    ↓
GPU Drivers (Metal/Vulkan/DX12)
```

The JavaScript wrapper provides 100% standard WebGPU API while the Rust layer uses optimized flat signatures for napi-rs compatibility.

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Areas for Contribution:**
- 📚 Additional examples and tutorials
- 🧪 More test cases and benchmarks
- 🔌 Integration with frameworks (Three.js, Babylon.js, etc.)
- 📖 Documentation improvements
- 🐛 Bug reports and fixes

## 📄 License

MIT © [SylphxAI](https://github.com/SylphxAI)

## 🔗 Resources

- **[API Documentation](https://sylphxai.github.io/webgpu/)** - Complete API reference
- **[WebGPU Specification](https://gpuweb.github.io/webgpu/)** - Official W3C spec
- **[wgpu-rs](https://github.com/gfx-rs/wgpu)** - Underlying Rust implementation
- **[Examples](examples/)** - 13 working examples
- **[CHANGELOG](CHANGELOG.md)** - Version history
- **[ROADMAP](ROADMAP.md)** - Project status and future plans

## ⭐ Star History

If you find this project useful, please consider giving it a star on GitHub!

---

**Ready to use WebGPU in Node.js?**

```bash
npm install @sylphx/webgpu
```

**v1.0.1 - Production Ready** 🚀
