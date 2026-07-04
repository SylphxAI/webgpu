# Why This Library?

## The Problem with Existing Solutions

### Dawn (Google's WebGPU - via @kmamal/gpu)
- **Binary size**: 50-150MB (per platform)
- **Build time**: 1-3 hours (requires depot_tools, Chromium build system)
- **Dependencies**: Entire Chromium toolchain
- **Complexity**: Non-standard custom API

### Other Bindings
- Incomplete API coverage
- Poor documentation
- Limited platform support
- Non-standard APIs

## Our Solution

### Built with Rust + wgpu
`@sylphx/webgpu` uses Mozilla's battle-tested `wgpu` implementation (same as Firefox, Deno, Bevy), providing:

- **20-50x smaller binary**: 1.9-4.6MB vs 50-150MB (actual measured sizes)
- **120x faster builds**: ~30 seconds vs 1-3 hours (actual clean build times)
- **WebGPU-standard API**: Browser-compatible API shape for portable code
- **Modern architecture**: Pure Rust, no C++ complexity

### Production Ready

✅ **Documented API coverage**: Covered package surface is implemented and tested
✅ **Cross-platform**: macOS, Linux, Windows (x64 + ARM64)
✅ **Runtime support**: Node.js 18+ and Bun 1.0+
✅ **Well tested**: 58-test local suite with real GPU operations
✅ **Minimal dependencies**: Just native bindings, no bloat

## Role in the SylphxAI Python-Class TypeScript Stack

`@sylphx/webgpu` is the GPU substrate, not the NumPy or PyTorch API layer.
That boundary is intentional:

- this package owns WebGPU adapter/device behavior, command submission, buffer
  and texture primitives, native platform artifacts, examples, and release
  readback;
- numerical libraries own Python-compatible syntax, tensor/array semantics,
  autograd semantics, benchmark admission, and release claims;
- benchmark reports may cite this backend only when they record package version,
  platform package, adapter/backend info, and workload shape.

This keeps the WebGPU package reusable across rendering, compute, scientific
workloads, and ML backends without hardcoding one consumer's benchmark story.
For the durable boundary decision, see
[Python Performance Backend Contract](/adr/001-python-performance-backend-contract).

### Performance

**Binary Sizes (Actual Prebuilt Binaries):**
- macOS ARM64: 1.9MB
- macOS x64: 2.2MB
- Linux ARM64: 3.0MB
- Linux x64: 3.5MB
- Windows ARM64: 4.1MB
- Windows x64: 4.6MB

**Build Time (Measured Clean Build):**
- This library: 29 seconds (Cargo release build)
- @kmamal/gpu: 1-3 hours (Dawn + depot_tools)

## Design Philosophy

### Lightweight
No unnecessary abstractions. Thin binding layer that exposes WebGPU API directly.

### Fast
Built with Rust and napi-rs for maximum performance. Zero-copy buffer operations where possible.

### Reliable
Comprehensive error handling. Validation at API boundaries. Clear error messages.

### Modern
ES modules, TypeScript definitions, async/await. Built for modern JavaScript.

## When to Use This

✅ **Machine learning inference** on GPU
✅ **Backend kernels** for TypeScript numerical libraries
✅ **Image/video processing** pipelines
✅ **Scientific computing** in Node.js
✅ **Data visualization** rendering
✅ **Cryptography** and parallel algorithms

## When NOT to Use This

❌ **Web browsers** - Use native WebGPU API
❌ **Training large models** - Use PyTorch/TensorFlow
❌ **Game development** - Use game engines

## Comparison Table

| Feature | @kmamal/gpu (Dawn) | This Library |
|---------|-------------------|--------------|
| **WebGPU API** | ⚠️ Custom API | ✅ Standards-aligned browser API |
| **Binary Size** | 50-150MB | 1.9-4.6MB |
| **Build Time** | 1-3 hours | ~30 seconds |
| **API Coverage** | Partial | Documented package surface |
| **Browser Compatible** | ❌ Node.js only | ✅ Shares code |
| **TypeScript** | ❌ | ✅ Full definitions |
| **Bun Support** | ⚠️ Limited | ✅ Native support |
| **Status** | 0.x Pre-release | v1.0 Production |

## Next Steps

Ready to try it? Head to [Getting Started](/guide/getting-started) →
