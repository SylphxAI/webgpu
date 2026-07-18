# @sylphx/webgpu

<p align="center">
  <img src="https://mark.sylphx.com/api/v1/banner?type=liquid&theme=tokyonight&text=webgpu&desc=WebGPU+for+Node.js+via+wgpu-rs+%28modern%2C+lightweight+alternative+to+Dawn%29&height=200&animation=rise&credit=0" alt="webgpu — Sylphx Mark banner" width="100%" />
</p>

[![npm version](https://img.shields.io/npm/v/@sylphx/webgpu.svg)](https://www.npmjs.com/package/@sylphx/webgpu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Production-ready WebGPU for Node.js & Bun** - standards-aligned, lightweight, built with Rust + wgpu

## ✨ What is @sylphx/webgpu?

**The modern, lightweight WebGPU implementation for Node.js.** Use a
browser-compatible WebGPU API in Node.js and Bun, backed by Rust/wgpu native
bindings.

```javascript
const { Gpu, GPUBufferUsage } = require('@sylphx/webgpu')

// Initialize GPU (same async shape as browser WebGPU)
const gpu = Gpu()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()

// Create buffer (WebGPU-standard descriptor shape)
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

## Role in Python-Class TypeScript ML

`@sylphx/webgpu` is the native GPU substrate for the broader SylphxAI
Python-to-TypeScript stack. It is not the NumPy or PyTorch API layer itself;
instead, it gives libraries such as `@sylphx/numpy` and `@sylphx/torch` a
consumer-neutral Rust/wgpu execution path for kernels, tensors, and GPU-backed
workloads.

The boundary matters:

- numerical libraries should expose Python-familiar synchronous hot-path APIs;
- backend selection and GPU synchronization should be explicit, sparse async
  boundaries;
- this package owns WebGPU package behavior, native artifacts, and examples,
  not one consumer's benchmark story.

See `docs/adr/001-python-performance-backend-contract.md` for the backend
contract.

## Published Package Status

`@sylphx/webgpu` is published and registry-readback verified at `1.0.4`.
Release run `28689182140` built all six native targets, published through the
central Sylphx release workflow, and completed post-publish readback.

Registry readback is currently verified for:

- `@sylphx/webgpu@1.0.4`
- `@sylphx/webgpu-darwin-arm64@1.0.4`
- `@sylphx/webgpu-darwin-x64@1.0.4`
- `@sylphx/webgpu-linux-arm64-gnu@1.0.4`
- `@sylphx/webgpu-linux-x64-gnu@1.0.4`
- `@sylphx/webgpu-win32-arm64-msvc@1.0.4`
- `@sylphx/webgpu-win32-x64-msvc@1.0.4`

This proves package availability and native artifact publication. Python API
parity and NumPy/PyTorch-style performance claims still belong to the consumer
numerical library using this backend.

## 🚀 Why Choose @sylphx/webgpu?

| Feature | @sylphx/webgpu | @kmamal/gpu (Dawn) |
|---------|---------------|-------------------|
| **WebGPU API** | ✅ Standards-aligned browser API | ⚠️ Custom API |
| **Binary Size** | 1.9-4.6MB | 50-150MB |
| **Build Time** | ~30 seconds | 1-3 hours |
| **Code Portability** | ✅ Browser compatible | ❌ Node.js only |
| **Implementation** | Firefox's wgpu (Rust) | Chrome's Dawn (C++) |
| **Toolchain** | Simple (Cargo) | Complex (depot_tools) |
| **Status** | v1.0 - Production ready | 0.x - Pre-release |

### Key Advantages

✅ **WebGPU-standard API** - Share code between Node.js and browsers
✅ **Production Ready** - v1.0.4 stable release with a 58-test local suite
✅ **Ultra Lightweight** - 2-5MB prebuilt platform binaries
✅ **Modern Stack** - Rust + wgpu (used by Firefox, Deno, Bevy)
✅ **Cross-Platform** - 6 prebuilt platforms (macOS, Linux, Windows, ARM64)
✅ **Well Tested** - Comprehensive test suite covering the documented package surface

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
# or with Bun (faster startup)
bun examples/compute.js
```

## 🎓 API Documentation

### WebGPU Standard API

The API follows the [W3C WebGPU specification](https://gpuweb.github.io/gpuweb/)
shape closely enough that browser-oriented WebGPU code can be ported to Node.js
and Bun with minimal changes. Full conformance claims require the package's test
and compatibility evidence, not README wording alone.

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

The wrapper intentionally follows browser WebGPU naming and descriptor shapes,
but conformance claims must be backed by the package's tests and compatibility
evidence.

## 🌐 Browser Compatibility

Share code between Node.js and browsers:

```javascript
// Universal WebGPU-style code
export async function initGPU() {
    // Detect environment
    const gpu = typeof navigator !== 'undefined'
        ? navigator.gpu
        : require('@sylphx/webgpu').Gpu()

    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Keep shared code on documented WebGPU descriptor shapes.
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
- Current suite has passed 58/58 in local validation
- Covers the documented WebGPU package surface
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

# Build native bindings (~30 seconds clean build)
npm run build

# Run tests
npm test
```

## 📊 Performance

**Binary Size (Actual Prebuilt Binaries):**
- @sylphx/webgpu: 1.9-4.6MB (macOS: 1.9-2.2MB, Linux: 3.0-3.5MB, Windows: 4.1-4.6MB)
- @kmamal/gpu: 50-150MB (Dawn binaries)

**Build Time (Clean Build):**
- @sylphx/webgpu: ~30 seconds (Cargo release build)
- @kmamal/gpu: 1-3 hours (Dawn + depot_tools)

**Runtime Performance:**
- GPU work is submitted through the native Rust/wgpu backend.
- JavaScript wrapper overhead depends on descriptor shape, buffer movement, and
  synchronization points.
- Consumer benchmarks should record package version, platform package,
  adapter/backend info, and workload shape before making performance claims.

## 🛠️ Architecture

```
User Code (WebGPU-style API)
    ↓
webgpu.js (JavaScript wrapper - transforms descriptors)
    ↓
index.js (napi-rs native bindings)
    ↓
Rust (wgpu implementation)
    ↓
GPU Drivers (Metal/Vulkan/DX12)
```

The JavaScript wrapper provides the WebGPU-style package API while the Rust
layer uses optimized flat signatures for napi-rs compatibility.

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

**v1.0.4 - Production Ready**

## Project Control and Release Proof

This repository dogfoods [GroundAtlas](https://github.com/SylphxAI/groundatlas)
through CI. The vendor-neutral project facts live in `project.manifest.json`;
Sylphx-specific governance facts stay in `.doctrine/project.json`; generated
`.groundatlas*` reports and JSON/Markdown GroundAtlas reports are
evidence/navigation read models only, not source of truth.

Package releases run through the shared Sylphx release workflow and are complete
only after native build artifacts, the Release workflow, `release:readback`, npm
registry readback for `@sylphx/webgpu` and platform optional dependencies, and
consumer smoke evidence for changed native behavior.


## GroundAtlas

GroundAtlas package dogfood is **retired** (Control Plane ADR-0014). Do not re-add required groundatlas CI jobs.
