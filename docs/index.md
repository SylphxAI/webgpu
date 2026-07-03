---
layout: home

hero:
  name: "@sylphx/webgpu"
  text: "WebGPU for Node.js & Bun"
  tagline: Modern, lightweight alternative to Dawn. Built with Rust + wgpu.
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
    title: Lightning Fast
    details: Built with Rust and wgpu. Ultra-small binaries (1.9-4.6MB vs 50-150MB), 120x faster build (~30s vs 1-3 hours).

  - icon: 🎯
    title: WebGPU-Standard API
    details: Browser-compatible API shape for Node.js and Bun. Share WebGPU code across runtime boundaries with minimal changes.

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
- 🚀 **Bun Support** - Works perfectly with Bun 1.0+
- 📚 **Complete Documentation** - Full guides and API reference
- 🌐 **Browser Compatible** - Share code between Node.js and browsers
:::

## Performance

```
Binary Size:     1.9-4.6MB  (20-50x smaller than Dawn)
Build Time:      ~30s       (120x faster than Dawn)
Tests:           58 pass    (local package suite)
```

## Community

- [GitHub Issues](https://github.com/SylphxAI/webgpu/issues) - Report bugs
- [GitHub Discussions](https://github.com/SylphxAI/webgpu/discussions) - Ask questions
- [Examples](https://github.com/SylphxAI/webgpu/tree/main/examples) - Learn from examples
