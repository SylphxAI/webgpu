# GPU

The GPU instance is the entry point for all WebGPU operations. It provides methods to discover and request GPU adapters.

## Overview

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = await gpu.requestAdapter()
const device = await adapter.requestDevice()

// Now ready for GPU operations
```

## Creating GPU Instance

### `Gpu.create()`

Creates a GPU instance for WebGPU operations.

**Returns:** `Gpu`

**Example:**
```javascript
const { Gpu } = require('@sylphx/webgpu')
const gpu = Gpu.create()
```

## Methods

### `gpu.requestAdapter(powerPreference?)`

Requests a GPU adapter with specified preferences.

**Parameters:**
- `powerPreference` (String, optional): Power preference hint
  - `'high-performance'` - Prefer discrete GPU with maximum performance
  - `'low-power'` - Prefer integrated GPU with lower power consumption
  - `undefined` - Let the system decide (default)

**Returns:** `Promise<GPUAdapter>`

**Throws:** Error if no suitable adapter is found

**Example:**
```javascript
// Request high-performance GPU (discrete if available)
const adapter = await gpu.requestAdapter('high-performance')

if (!adapter) {
  throw new Error('No GPU adapter found')
}

// Request low-power GPU (integrated)
const adapter2 = await gpu.requestAdapter('low-power')

// Let system decide (default)
const adapter3 = await gpu.requestAdapter()
```

### `gpu.enumerateAdapters()`

Returns all available GPU adapters.

**Returns:** `Array<String>` - List of adapter names with backend info

**Example:**
```javascript
const adapters = gpu.enumerateAdapters()

console.log(`Found ${adapters.length} GPU(s):`)
adapters.forEach((adapter, i) => {
  console.log(`  ${i}: ${adapter}`)
})

// Example output:
// Found 2 GPU(s):
//   0: NVIDIA GeForce RTX 3080 (Vulkan)
//   1: Intel UHD Graphics 630 (Vulkan)
```

## Power Preference

### `'high-performance'`

Prefer discrete GPU with maximum performance.

**Use when:**
- Running compute-intensive workloads (ML, rendering, simulation)
- Maximum performance is critical
- Power consumption is not a concern
- Workload can utilize discrete GPU effectively

**Characteristics:**
- Typically selects discrete GPU (dedicated graphics card)
- Higher power consumption
- Better thermal requirements
- Maximum compute/rendering throughput

**Example:**
```javascript
const adapter = await gpu.requestAdapter('high-performance')
const info = adapter.getInfo()
console.log('Selected GPU:', info.name)  // e.g., "NVIDIA GeForce RTX 3080"
console.log('Type:', info.deviceType)    // "discrete-gpu"
```

### `'low-power'`

Prefer integrated GPU with lower power consumption.

**Use when:**
- Running on battery-powered devices (laptops, mobile)
- Workload is not compute-intensive
- Power efficiency is important
- Extended runtime is priority

**Characteristics:**
- Typically selects integrated GPU (CPU-integrated)
- Lower power consumption
- Better battery life
- Adequate performance for most tasks

**Example:**
```javascript
const adapter = await gpu.requestAdapter('low-power')
const info = adapter.getInfo()
console.log('Selected GPU:', info.name)  // e.g., "Intel UHD Graphics 630"
console.log('Type:', info.deviceType)    // "integrated-gpu"
```

### `undefined` (default)

Let the system decide based on system policy.

**Use when:**
- No specific performance requirements
- Want system to optimize automatically
- Supporting various hardware configurations

**Characteristics:**
- System selects based on platform policy
- May prefer integrated GPU for efficiency
- May prefer discrete GPU for performance workloads
- Platform-dependent behavior

## Platform-Specific Behavior

### macOS

```javascript
const adapters = gpu.enumerateAdapters()
// Usually 1 adapter: "Apple M1 (Metal)" or "AMD Radeon Pro (Metal)"

const adapter = await gpu.requestAdapter()
const info = adapter.getInfo()
console.log('Backend:', info.backend)  // "Metal"
```

**Characteristics:**
- Usually single Metal adapter
- Integrated and discrete GPUs may share same adapter
- System automatically chooses GPU based on workload
- Excellent performance with Metal backend

### Linux

```javascript
const adapters = gpu.enumerateAdapters()
// May have multiple: "NVIDIA GeForce RTX 3080 (Vulkan)", "Intel HD Graphics (Vulkan)"

const adapter = await gpu.requestAdapter('high-performance')
const info = adapter.getInfo()
console.log('Backend:', info.backend)  // "Vulkan"
```

**Characteristics:**
- May have multiple Vulkan adapters
- Separate adapters for integrated and discrete GPUs
- Manual selection may be required
- Vulkan backend

### Windows

```javascript
const adapters = gpu.enumerateAdapters()
// May have multiple: "NVIDIA GeForce RTX 3080 (Dx12)", "Intel UHD Graphics (Dx12)"

const adapter = await gpu.requestAdapter('high-performance')
const info = adapter.getInfo()
console.log('Backend:', info.backend)  // "Dx12" or "Vulkan"
```

**Characteristics:**
- May have multiple adapters (DirectX 12 or Vulkan)
- Separate adapters for integrated and discrete GPUs
- `powerPreference` helps select appropriate GPU
- DirectX 12 or Vulkan backend

## Complete Examples

### Basic Setup

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function setupWebGPU() {
  // Create GPU instance
  const gpu = Gpu.create()

  // Request adapter
  const adapter = await gpu.requestAdapter()

  if (!adapter) {
    throw new Error('No GPU adapter available')
  }

  // Get adapter info
  const info = adapter.getInfo()
  console.log('GPU:', info.name)
  console.log('Backend:', info.backend)
  console.log('Type:', info.deviceType)

  // Request device
  const device = await adapter.requestDevice()

  return { gpu, adapter, device }
}

setupWebGPU().then(({ device }) => {
  console.log('✅ WebGPU initialized!')
  // Ready for GPU operations
})
```

### Selecting Best GPU

```javascript
const { Gpu } = require('@sylphx/webgpu')

function selectBestGPU() {
  const gpu = Gpu.create()
  const adapters = gpu.enumerateAdapters()

  if (adapters.length === 0) {
    throw new Error('No GPU found')
  }

  console.log('Available GPUs:')
  adapters.forEach((adapter, i) => {
    console.log(`  ${i}: ${adapter}`)
  })

  // Prefer high-performance adapter
  const adapter = gpu.requestAdapter('high-performance')

  if (!adapter) {
    console.warn('No high-performance GPU, falling back to default')
    return gpu.requestAdapter()
  }

  return adapter
}

async function main() {
  const adapter = await selectBestGPU()
  const info = adapter.getInfo()

  console.log('Selected:', info.name)
  console.log('Backend:', info.backend)

  const device = await adapter.requestDevice()
  // Ready for GPU operations
}

main()
```

### Listing All GPUs

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function listAllGPUs() {
  const gpu = Gpu.create()
  const adapters = gpu.enumerateAdapters()

  console.log(`Found ${adapters.length} GPU(s):\n`)

  adapters.forEach((adapterName, i) => {
    console.log(`GPU ${i}:`)
    console.log(`  Name: ${adapterName}`)
  })
}

listAllGPUs()
```

## Error Handling

```javascript
const { Gpu } = require('@sylphx/webgpu')

async function initializeWebGPU() {
  try {
    const gpu = Gpu.create()
    const adapter = await gpu.requestAdapter()

    if (!adapter) {
      throw new Error('No GPU adapter available. WebGPU not supported.')
    }

    const device = await adapter.requestDevice()

    return device

  } catch (error) {
    console.error('WebGPU initialization failed:', error.message)

    // Fallback strategies:
    // 1. Try software renderer
    // 2. Fall back to CPU processing
    // 3. Show error to user

    throw error
  }
}

initializeWebGPU()
  .then(device => console.log('✅ GPU ready'))
  .catch(error => console.error('❌ GPU initialization failed'))
```

## Best Practices

### 1. Always Check for Adapter

```javascript
const adapter = await gpu.requestAdapter()

if (!adapter) {
  // Handle case where WebGPU is not available
  throw new Error('WebGPU not supported')
}
```

### 2. List Available GPUs in Development

```javascript
if (process.env.NODE_ENV === 'development') {
  const adapters = gpu.enumerateAdapters()
  console.log('Available GPUs:', adapters)
}
```

### 3. Use Power Preference Wisely

```javascript
// For compute-intensive workloads
const adapter = await gpu.requestAdapter('high-performance')

// For battery-powered devices
const adapter = await gpu.requestAdapter('low-power')

// For general use
const adapter = await gpu.requestAdapter()  // Let system decide
```

### 4. Singleton Pattern for GPU Instance

```javascript
let gpuInstance = null

function getGPU() {
  if (!gpuInstance) {
    gpuInstance = Gpu.create()
  }
  return gpuInstance
}

// Use throughout application
const gpu = getGPU()
const adapter = await gpu.requestAdapter()
```

## Troubleshooting

### Error: "No GPU adapter available"

**Cause:** No compatible GPU found on the system.

**Solutions:**
1. Update graphics drivers
2. Ensure WebGPU-compatible GPU is present
3. Check if GPU is enabled in BIOS
4. Try software renderer (if available)

**Example:**
```javascript
const adapter = await gpu.requestAdapter()

if (!adapter) {
  console.error('No GPU found. Possible causes:')
  console.error('  - No WebGPU-compatible GPU')
  console.error('  - Outdated graphics drivers')
  console.error('  - GPU disabled in BIOS')
  throw new Error('WebGPU not supported')
}
```

### Multiple GPUs Available

**Solution:** Use `enumerateAdapters()` to list all GPUs, then select specific one with `powerPreference`.

```javascript
// List all GPUs
const adapters = gpu.enumerateAdapters()
console.log('Available:', adapters)

// Select high-performance (discrete) GPU
const adapter = await gpu.requestAdapter('high-performance')
```

### Wrong GPU Selected

**Solution:** Try explicit power preference or check enumeration.

```javascript
// Try explicit high-performance
const adapter = await gpu.requestAdapter('high-performance')
const info = adapter.getInfo()

if (info.deviceType !== 'discrete-gpu') {
  console.warn('Expected discrete GPU but got:', info.deviceType)
}
```

## TypeScript

```typescript
import { Gpu, type GPUAdapter } from '@sylphx/webgpu'

const gpu: Gpu = Gpu.create()

const adapter: GPUAdapter = await gpu.requestAdapter('high-performance')

if (!adapter) {
  throw new Error('No adapter found')
}

const adapters: string[] = gpu.enumerateAdapters()
```

## See Also

- [Adapter](/api/adapter) - GPU adapter capabilities and device creation
- [Device](/api/device) - Main GPU interface
- [Getting Started Guide](/guide/getting-started) - WebGPU setup tutorial
