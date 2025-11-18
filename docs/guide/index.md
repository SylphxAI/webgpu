# What is WebGPU?

WebGPU is a modern graphics and compute API that provides unified access to GPU hardware across different platforms. It's the next generation of web graphics, designed to replace WebGL and enable high-performance GPU computing.

## Key Features

### 🎮 Modern Graphics API
- Lower overhead than WebGL
- Better multi-threading support
- Explicit control over GPU resources
- Built-in compute shader support

### 🚀 High Performance
- Direct access to GPU hardware
- Minimal driver overhead
- Efficient memory management
- Support for modern GPU features

### 🌐 Cross-Platform
- Works on Metal (macOS/iOS)
- Works on Vulkan (Linux/Android)
- Works on DirectX 12 (Windows)
- Consistent API across platforms

## Why Use WebGPU in Node.js?

WebGPU isn't just for browsers! Running WebGPU in Node.js enables:

### Server-Side Rendering
```javascript
// Render images on the server
const texture = renderScene(device)
const pixels = await readTexture(texture)
// Save to file or send to client
```

### GPU Compute
```javascript
// Parallel data processing
const results = await gpuCompute(largeDataset)
// 100x faster than CPU for parallel workloads
```

### Machine Learning
```javascript
// Run ML inference on GPU
const prediction = await runModel(input)
// Much faster than CPU-only inference
```

### Image Processing
```javascript
// Process images with shaders
const processed = await processImage(image, filter)
// Real-time image manipulation
```

## @sylphx/webgpu vs Browser WebGPU

| Feature | Browser WebGPU | @sylphx/webgpu |
|---------|---------------|----------------|
| **Environment** | Browser only | Node.js & Bun |
| **Canvas** | Yes | No (headless) |
| **Server-side** | No | ✅ Yes |
| **File I/O** | Limited | ✅ Full access |
| **Native modules** | No | ✅ Yes |
| **Performance** | Good | ✅ Better (no browser overhead) |

## Architecture

```
┌─────────────────────────────────────┐
│  Your Application (JavaScript/TS)  │
├─────────────────────────────────────┤
│  @sylphx/webgpu (Node.js bindings) │
├─────────────────────────────────────┤
│  wgpu-rs (Rust implementation)     │
├─────────────────────────────────────┤
│  GPU Drivers (Metal/Vulkan/DX12)   │
└─────────────────────────────────────┘
```

### Components

**wgpu-rs**: Mozilla's Rust implementation of WebGPU
- Battle-tested in Firefox
- High performance
- Memory safe
- Cross-platform

**napi-rs**: Native Node.js bindings
- Zero-cost abstractions
- TypeScript support
- Async/await support
- Cross-platform builds

**Your Code**: Simple JavaScript/TypeScript
- Familiar API (same as browser WebGPU)
- No complex build setup
- Full TypeScript support

## Use Cases

### ✅ Perfect For

- **Image/Video Processing** - Filters, transcoding, manipulation
- **Scientific Computing** - Simulations, data analysis
- **Machine Learning** - Inference, training
- **Crypto Mining** - Parallel hash computation
- **3D Rendering** - Server-side rendering, thumbnails
- **Data Visualization** - Generate charts, graphs

### ❌ Not Ideal For

- **Interactive 3D Apps** - Use browser WebGPU instead
- **Real-time Games** - Better suited for browser/native
- **Simple Tasks** - Use CPU for simple operations

## Next Steps

Ready to get started? Check out:

- [Getting Started](/guide/getting-started) - Install and setup
- [First Steps](/guide/first-steps) - Your first WebGPU program
- [Examples](/examples/) - Learn from working examples
