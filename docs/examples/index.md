# Examples

Learn by example! All examples are tested and verified working.

## Getting Started

Clone the repository to run examples locally:

```bash
git clone https://github.com/SylphxAI/webgpu.git
cd webgpu
npm install
npm run build

# Run any example
node examples/compute.js
bun examples/triangle.js
```

## Example Categories

### 🧮 Compute

GPU computation and data processing:

- [Vector Addition](/examples/compute) - Add two arrays on GPU
- [Indirect Compute](https://github.com/SylphxAI/webgpu/blob/main/examples/indirect-compute.js) - GPU-driven dispatch

### 🎨 Rendering

Graphics and rendering:

- [Triangle](/examples/triangle) - Render a simple triangle
- [Textured Quad](/examples/texture-upload) - Texture mapping
- [MSAA](https://github.com/SylphxAI/webgpu/blob/main/examples/msaa.js) - Anti-aliasing
- [Render Bundle](/examples/render-bundle) - Reusable command recording

### 📦 Advanced

Advanced features:

- [Timestamp Queries](https://github.com/SylphxAI/webgpu/blob/main/examples/timestamp-queries.js) - GPU profiling
- [Indirect Draw](https://github.com/SylphxAI/webgpu/blob/main/examples/indirect-draw.js) - GPU-driven rendering
- [Cube](https://github.com/SylphxAI/webgpu/blob/main/examples/cube.js) - 3D rendering with depth
- [Transparency](https://github.com/SylphxAI/webgpu/blob/main/examples/transparency.js) - Alpha blending

## Quick Examples

### Hello GPU

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function main() {
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const info = adapter.getInfo()

  console.log('GPU:', info.name)
  console.log('Backend:', info.backend)
}

main()
```

### Simple Compute

```javascript
const { Gpu, bufferUsage } = require('@sylphx/webgpu')

async function compute() {
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()
  const usage = bufferUsage()

  // Your compute code here
  const shader = device.createShaderModule(\`
    @compute @workgroup_size(1)
    fn main() { /* ... */ }
  \`)

  device.destroy()
}

compute()
```

### Simple Render

```javascript
const { Gpu, bufferUsage, textureUsage } = require('@sylphx/webgpu')

async function render() {
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()
  const device = await adapter.requestDevice()

  const shader = device.createShaderModule(\`
    @vertex
    fn vs_main(@location(0) pos: vec3f) -> @builtin(position) vec4f {
      return vec4f(pos, 1.0);
    }

    @fragment
    fn fs_main() -> @location(0) vec4f {
      return vec4f(1.0, 0.0, 0.0, 1.0);
    }
  \`)

  // Create pipeline and render
  device.destroy()
}

render()
```

## Running Examples

All examples can run with Node.js or Bun:

```bash
# Node.js
node examples/compute.js

# Bun (faster!)
bun examples/triangle.js
```

## Example Output

Each example prints verification output:

```
🧮 WebGPU Compute Example: Vector Addition

✓ Device ready
✓ Input data written to buffers
✓ Shader compiled
✓ Bind group layout created
✓ Pipeline layout created
✓ Compute pipeline created
✓ Bind group created
✓ Commands encoded
✓ GPU work complete

Expected: [ 11, 22, 33, 44, 55 ]
Actual:   [ 11, 22, 33, 44, 55 ]

✅ All results correct! GPU compute verified!
```

## Browse All Examples

Visit our [GitHub repository](https://github.com/SylphxAI/webgpu/tree/main/examples) to see all 12 examples:

1. compute.js - Vector addition
2. triangle.js - Render triangle
3. texture-upload.js - Upload texture
4. textured-quad.js - Textured rendering
5. cube.js - 3D cube with depth
6. transparency.js - Alpha blending
7. msaa.js - Anti-aliasing
8. mrt.js - Multiple render targets
9. timestamp-queries.js - GPU profiling
10. indirect-draw.js - Indirect rendering
11. indirect-compute.js - Indirect compute
12. render-bundle.js - Command bundles

## Next Steps

- Read the [API Reference](/api/)
- Learn [Core Concepts](/guide/gpu-adapters)
- View source on [GitHub](https://github.com/SylphxAI/webgpu)
