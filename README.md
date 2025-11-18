# @sylphx/webgpu

[![npm version](https://img.shields.io/npm/v/@sylphx/webgpu.svg)](https://www.npmjs.com/package/@sylphx/webgpu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> WebGPU for Node.js via [wgpu-rs](https://github.com/gfx-rs/wgpu) - A modern, lightweight alternative to Dawn

## Why This Instead of Dawn?

| Feature | @sylphx/webgpu (wgpu) | @kmamal/gpu (Dawn) |
|---------|----------------------|-------------------|
| **Build Time** | 5-15 minutes | 1-3 hours |
| **Binary Size** | ~10MB | 50-150MB |
| **Dependencies** | Cargo only | depot_tools (1GB) + Dawn source (8GB) |
| **Implementation** | Firefox's wgpu (Rust) | Chrome's Dawn (C++) |
| **Toolchain** | Modern (Cargo) | Complex (gclient, ninja, cmake) |
| **Platform Support** | 18+ platforms via napi-rs | Limited prebuilt binaries |

## Features

- ✅ **Lightweight**: ~10MB binary vs 50-150MB Dawn
- ✅ **Fast builds**: Minutes instead of hours
- ✅ **Modern toolchain**: Rust + Cargo, no depot_tools needed
- ✅ **Type-safe**: Rust guarantees with N-API bindings
- ✅ **Cross-platform**: Supports 18+ platforms out of the box
- ✅ **Production-ready**: Built with napi-rs (same tech as Vercel's Next.js)

## Installation

```bash
npm install @sylphx/webgpu
```

**Prerequisites:**
- Node.js 18+
- No build tools needed (prebuilt binaries provided)

## Quick Start

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function main() {
    // Create GPU instance
    const gpu = Gpu.create()

    // Request adapter
    const adapter = await gpu.requestAdapter('high-performance')

    // Get adapter info
    const info = adapter.getInfo()
    console.log('GPU:', info.name)
    console.log('Backend:', info.backend)

    // Request device
    const device = await adapter.requestDevice()

    // Ready to use WebGPU!
    console.log('WebGPU ready!')
}

main()
```

## Examples

### Basic Usage

```javascript
const { Gpu, bufferUsage } = require('@sylphx/webgpu')

async function example() {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()
    const device = await adapter.requestDevice()

    // Create a buffer
    const buffer = device.createBuffer(
        256,  // size in bytes
        bufferUsage.UNIFORM | bufferUsage.COPY_DST,
        false // not mapped at creation
    )

    // Create a shader
    const shader = device.createShaderModule(`
        @vertex
        fn vs_main() -> @builtin(position) vec4<f32> {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
    `)

    // Cleanup
    buffer.destroy()
    device.destroy()
}
```

### Enumerate Adapters

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapters = gpu.enumerateAdapters()

console.log('Available GPU adapters:')
adapters.forEach((adapter, i) => {
    console.log(`${i + 1}. ${adapter}`)
})
```

### Adapter Information

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function showAdapterInfo() {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()

    const info = adapter.getInfo()
    console.log('Name:', info.name)
    console.log('Vendor:', info.vendor)
    console.log('Device Type:', info.deviceType)
    console.log('Backend:', info.backend)

    const limits = adapter.getLimits()
    console.log('Max Texture 2D:', limits.maxTextureDimension2d)
    console.log('Max Buffer Size:', limits.maxBufferSize)

    const features = adapter.getFeatures()
    console.log('Features:', features)
}
```

## API Reference

### `Gpu`

#### `Gpu.create(): Gpu`
Create a new GPU instance.

#### `gpu.requestAdapter(powerPreference?: string): Promise<GpuAdapter>`
Request a GPU adapter. Power preference can be:
- `'low-power'` - Prefer battery life
- `'high-performance'` - Prefer performance

#### `gpu.enumerateAdapters(): string[]`
List all available GPU adapters.

### `GpuAdapter`

#### `adapter.getInfo(): AdapterInfo`
Get adapter information (name, vendor, device type, backend).

#### `adapter.getFeatures(): string[]`
Get supported WebGPU features.

#### `adapter.getLimits(): AdapterLimits`
Get adapter limits (max texture size, buffer size, etc.).

#### `adapter.requestDevice(): Promise<GpuDevice>`
Request a GPU device.

### `GpuDevice`

#### `device.createBuffer(size: number, usage: number, mappedAtCreation?: boolean): GpuBuffer`
Create a GPU buffer.

#### `device.createShaderModule(code: string): GpuShaderModule`
Create a shader module from WGSL code.

#### `device.createCommandEncoder(): GpuCommandEncoder`
Create a command encoder.

#### `device.queueSubmit(commandBuffer: GpuCommandBuffer): void`
Submit commands to the GPU queue.

#### `device.poll(forceWait?: boolean): void`
Poll the device for completed operations.

#### `device.destroy(): void`
Destroy the device and free resources.

### Constants

#### `bufferUsage`
Buffer usage flags:
- `COPY_SRC` - Can be copied from
- `COPY_DST` - Can be copied to
- `STORAGE` - Can be used as storage buffer
- `UNIFORM` - Can be used as uniform buffer
- `VERTEX` - Can be used as vertex buffer
- `INDEX` - Can be used as index buffer
- `MAP_READ` - Can be mapped for reading
- `MAP_WRITE` - Can be mapped for writing

## Building from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/SylphxAI/webgpu.git
cd webgpu

# Install dependencies
npm install

# Build (takes 5-15 minutes)
npm run build

# Run tests
npm test

# Run examples
node examples/basic.js
```

## Architecture

```
┌─────────────────────────────────────┐
│         Node.js Application         │
└──────────────┬──────────────────────┘
               │ JavaScript
               ↓
┌─────────────────────────────────────┐
│      N-API Bindings (napi-rs)       │
└──────────────┬──────────────────────┘
               │ Rust
               ↓
┌─────────────────────────────────────┐
│       wgpu-rs (Rust WebGPU)         │
└──────────────┬──────────────────────┘
               │ Native
               ↓
┌─────────────────────────────────────┐
│   GPU Drivers (Vulkan/Metal/DX12)  │
└─────────────────────────────────────┘
```

## Supported Platforms

Pre-built binaries available for:
- macOS x64, ARM64
- Linux x64, ARM64, musl
- Windows x64, x86
- FreeBSD x64
- Android ARM, ARM64

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).

## Comparison with Alternatives

### vs @kmamal/gpu (Dawn)
- ✅ **10x smaller** binaries (10MB vs 100MB+)
- ✅ **100x faster** builds (minutes vs hours)
- ✅ **Simpler** toolchain (Cargo vs depot_tools)
- ⚠️ Different backend (wgpu vs Dawn)

### vs Pure WebGPU in Browser
- ✅ Server-side rendering
- ✅ Headless compute
- ✅ No browser needed
- ⚠️ No DOM integration

## License

MIT © [SylphxAI](https://github.com/SylphxAI)

## Acknowledgments

- [wgpu-rs](https://github.com/gfx-rs/wgpu) - The WebGPU implementation
- [napi-rs](https://github.com/napi-rs/napi-rs) - Node.js addon framework
- [WebGPU Spec](https://gpuweb.github.io/gpuweb/) - The standard

---

**Note**: This is a modern alternative to Dawn-based solutions. Both implementations conform to the WebGPU specification, but use different underlying engines (wgpu vs Dawn).
