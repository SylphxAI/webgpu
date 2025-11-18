# Getting Started

Get up and running with @sylphx/webgpu in 5 minutes.

## Installation

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

## Prerequisites

- **Node.js 18+** or **Bun 1.0+**
- **No build tools needed** - prebuilt binaries provided
- **GPU Required** - Modern GPU with Metal/Vulkan/DirectX 12 support

### Supported Platforms

| Platform | Architecture | Backend |
|----------|-------------|---------|
| macOS | Apple Silicon (M1/M2/M3/M4) | Metal |
| macOS | Intel x64 | Metal |
| Linux | x64, ARM64 | Vulkan |
| Windows | x64, x86, ARM64 | DirectX 12 |
| Android | ARM64, ARM7 | Vulkan |

## Quick Start

Create a file `example.js`:

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function main() {
  // 1. Create GPU instance
  const gpu = Gpu.create()

  // 2. Request adapter
  const adapter = await gpu.requestAdapter()

  // 3. Get GPU info
  const info = adapter.getInfo()
  console.log('GPU:', info.name)
  console.log('Backend:', info.backend)

  // 4. Request device
  const device = await adapter.requestDevice()

  console.log('✅ WebGPU ready!')

  // 5. Clean up
  device.destroy()
}

main().catch(console.error)
```

Run it:

::: code-group

```bash [Node.js]
node example.js
```

```bash [Bun]
bun example.js
```

:::

Expected output:

```
GPU: Apple M4
Backend: Metal
✅ WebGPU ready!
```

## TypeScript Support

Full TypeScript definitions are included:

```typescript
import { Gpu, type AdapterInfo } from '@sylphx/webgpu'

async function main(): Promise<void> {
  const gpu = Gpu.create()
  const adapter = await gpu.requestAdapter()

  const info: AdapterInfo = adapter.getInfo()
  console.log(info.name)
}

main()
```

Run with:

```bash
bun run example.ts  # Bun has built-in TypeScript support
```

## Verify Installation

Run this to verify everything works:

```javascript
const { Gpu, bufferUsage } = require('@sylphx/webgpu')

async function verify() {
  console.log('🔍 Verifying @sylphx/webgpu installation...\n')

  // Check GPU
  const gpu = Gpu.create()
  console.log('✅ GPU instance created')

  // Check adapters
  const adapters = gpu.enumerateAdapters()
  console.log(`✅ Found ${adapters.length} adapter(s):`)
  adapters.forEach(a => console.log(`   - ${a}`))

  // Check adapter info
  const adapter = await gpu.requestAdapter()
  const info = adapter.getInfo()
  console.log(`✅ Adapter: ${info.name} (${info.backend})`)

  // Check features
  const features = adapter.getFeatures()
  console.log(`✅ Supported features: ${features.length}`)

  // Check limits
  const limits = adapter.getLimits()
  console.log(`✅ Max texture 2D: ${limits.maxTextureDimension2D}`)

  // Check device
  const device = await adapter.requestDevice()
  console.log('✅ Device created')

  // Check buffer
  const usage = bufferUsage()
  const buffer = device.createBuffer(256, usage.copyDst, false)
  console.log('✅ Buffer created')

  buffer.destroy()
  device.destroy()

  console.log('\n🎉 All checks passed! WebGPU is ready to use!')
}

verify().catch(console.error)
```

## Troubleshooting

### "No suitable GPU adapter found"

**Problem**: Can't find a GPU adapter.

**Solutions**:
1. Make sure you have a GPU (integrated or discrete)
2. Update your GPU drivers
3. Check if your OS supports Metal/Vulkan/DirectX 12

### "Module not found"

**Problem**: Import error.

**Solutions**:
1. Run `npm install` or `bun install`
2. Make sure you're using Node.js 18+ or Bun 1.0+
3. Try removing `node_modules` and reinstalling

### Build from Source

If prebuilt binaries don't work, build from source:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
npm run build
```

## Next Steps

Now that you have WebGPU installed:

- [First Steps](/guide/first-steps) - Write your first GPU program
- [Buffers](/guide/buffers) - Learn about GPU memory
- [Examples](/examples/) - See working code
