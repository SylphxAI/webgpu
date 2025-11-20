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
- **100% WebGPU Standard**: Browser-compatible API, code works everywhere
- **Modern architecture**: Pure Rust, no C++ complexity

### Production Ready

✅ **Complete API coverage**: All WebGPU features implemented
✅ **Cross-platform**: macOS, Linux, Windows (x64 + ARM64)
✅ **Runtime support**: Node.js 18+ and Bun 1.0+
✅ **Well tested**: 58 tests, 100% pass rate, real GPU operations
✅ **Minimal dependencies**: Just native bindings, no bloat

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
| **WebGPU Standard** | ⚠️ Custom API | ✅ 100% compliant |
| **Binary Size** | 50-150MB | 1.9-4.6MB |
| **Build Time** | 1-3 hours | ~30 seconds |
| **API Coverage** | Partial | 100% WebGPU |
| **Browser Compatible** | ❌ Node.js only | ✅ Shares code |
| **TypeScript** | ❌ | ✅ Full definitions |
| **Bun Support** | ⚠️ Limited | ✅ Native support |
| **Status** | 0.x Pre-release | v1.0 Production |

## Next Steps

Ready to try it? Head to [Getting Started](/guide/getting-started) →
