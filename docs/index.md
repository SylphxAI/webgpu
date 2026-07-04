---
layout: home

hero:
  name: "@sylphx/webgpu"
  text: "WebGPU for Node.js & Bun"
  tagline: Rust/wgpu native GPU substrate for server-side graphics, compute, and TypeScript numerical backends.
  image:
    src: /logo.svg
    alt: WebGPU
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/SylphxAI/webgpu

features:
  - icon: ⚡
    title: Native Rust Backend
    details: Built with Rust and wgpu. Published prebuilt platform packages are 1.9-4.6MB and avoid a local Dawn/depot_tools build step for consumers.

  - icon: 🎯
    title: WebGPU-style API
    details: Browser-style naming and descriptor shapes for Node.js and Bun. Keep shared code on the documented package surface.

  - icon: 🚀
    title: Production Ready (v1.0.4)
    details: Stable v1.0.4 release with a 58-test local suite covering the documented package surface.

  - icon: 🔧
    title: Easy to Use
    details: 5-minute setup. No depot_tools, no Dawn source code, just npm install and go.

  - icon: 📦
    title: Cross Platform
    details: 6 prebuilt platforms. macOS (x64/ARM64), Linux, Windows. Metal, Vulkan, DX12 backends.

  - icon: ✅
    title: Fully Tested
    details: Comprehensive test suite with real GPU operations. All compute, render, and texture features verified.

  - icon: 🧠
    title: Numerical Backend Substrate
    details: Consumer-neutral GPU package for higher-level libraries such as @sylphx/numpy and @sylphx/torch. Python API parity is proven in those libraries, not in this backend.
---

## Quick Start

::: code-group

```bash [npm]
npm install @sylphx/webgpu
```

```bash [bun]
bun add @sylphx/webgpu
```

```bash [pnpm]
pnpm add @sylphx/webgpu
```

:::

## Simple Example

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function main() {
  // Create GPU instance with the WebGPU-standard API shape
  const gpu = Gpu()

  // Request adapter
  const adapter = await gpu.requestAdapter()
  console.log('GPU:', adapter.info.name)

  // Request device
  const device = await adapter.requestDevice()

  // Ready to use WebGPU!
  console.log('WebGPU ready!')
}

main()
```

Run with:

```bash
# Node.js
node example.js

# Bun (faster startup)
bun example.js
```

## Why This Library?

| Feature | @sylphx/webgpu | @kmamal/gpu (Dawn) |
|---------|----------------|-------------------|
| **WebGPU API** | ✅ Standards-aligned browser API | ⚠️ Custom API |
| **Build Time** | ~30 seconds | 1-3 hours |
| **Binary Size** | 1.9-4.6MB | 50-150MB |
| **Implementation** | wgpu (Rust) | Dawn (C++) |
| **Status** | v1.0 - Production ready | 0.x - Pre-release |
| **Platform Support** | 6 prebuilt platforms | Limited prebuilts |

## Role in TypeScript Numerical Computing

`@sylphx/webgpu` is the WebGPU substrate underneath Python-familiar TypeScript
numerical libraries. It owns GPU adapter/device behavior, command execution,
native artifacts, and package readback. It does not own NumPy or PyTorch API
parity, tensor semantics, autograd semantics, or model training claims.

Use this package directly for WebGPU compute/render workloads. Use higher-level
libraries such as `@sylphx/numpy` or `@sylphx/torch` when you want Python-style
array or tensor APIs backed by an accelerator.

## Features

- ✅ **WebGPU-standard API** - Browser-compatible API shape
- ✅ **GPU Compute** - Run shaders on GPU for parallel computation
- ✅ **Rendering** - Full render pipeline with depth, MSAA, MRT
- ✅ **Textures & Samplers** - All formats and operations
- ✅ **Render Bundles** - Reusable command recording
- ✅ **Indirect Draw/Dispatch** - GPU-driven execution
- ✅ **Query Sets** - Timestamp queries for profiling
- ✅ **TypeScript** - Full type definitions included

## What's New in v1.0

::: info v1.0.4 - Production Ready
- 🎉 **Stable Release** - Production-ready v1.0.4
- ✅ **58 Tests Passing** - Local suite covers the documented package surface
- 🚀 **Bun Support** - Tested with Bun 1.0+
- 📚 **Documentation** - Guides and API reference
- 🌐 **WebGPU-style API** - Keep shared code on documented descriptor shapes
:::

## Performance

```
Platform packages: 1.9-4.6MB prebuilt native binaries
Consumer install:  No local Dawn/depot_tools build step expected
Tests:            58 pass local package suite
```

## Community

- [GitHub Issues](https://github.com/SylphxAI/webgpu/issues) - Report bugs
- [GitHub Discussions](https://github.com/SylphxAI/webgpu/discussions) - Ask questions
- [Examples](https://github.com/SylphxAI/webgpu/tree/main/examples) - Learn from examples
