# GPU & Adapters

## GPU Instance

The GPU instance is your entry point to WebGPU. It provides methods to discover and request GPU adapters.

### Creating GPU Instance

```javascript
const { Gpu } = require('@sylphx/webgpu')
const gpu = Gpu.create()
```

The GPU instance is a singleton. Creating multiple instances is safe and returns the same underlying GPU.

## Adapters

An adapter represents a physical GPU in your system. Most systems have one adapter, but some have multiple (integrated + discrete GPU).

### Enumerating Adapters

```javascript
const adapters = gpu.enumerateAdapters()
console.log(`Found ${adapters.length} GPU(s)`)

adapters.forEach((adapter, i) => {
  const info = adapter.getInfo()
  console.log(`GPU ${i}: ${info.name}`)
  console.log(`  Backend: ${info.backend}`)
  console.log(`  Vendor: ${info.vendorName || 'Unknown'}`)
})
```

### Requesting Adapter

```javascript
// Request default adapter
const adapter = gpu.requestAdapter({
  powerPreference: 'high-performance'
})

// Or use first enumerated adapter
const [adapter] = gpu.enumerateAdapters()
```

### Power Preference

- `'high-performance'` - Prefer discrete GPU
- `'low-power'` - Prefer integrated GPU
- `undefined` - Let system decide

```javascript
const adapter = gpu.requestAdapter({
  powerPreference: 'low-power' // Save battery
})
```

## Adapter Info

```javascript
const info = adapter.getInfo()

console.log('Name:', info.name)              // "NVIDIA GeForce RTX 3080"
console.log('Backend:', info.backend)        // "vulkan", "metal", "dx12"
console.log('Vendor:', info.vendorName)      // "NVIDIA"
console.log('Device ID:', info.deviceId)     // Hardware identifier
console.log('Vendor ID:', info.vendorId)     // Vendor identifier
```

### Backend Types

| Platform | Backend | Description |
|----------|---------|-------------|
| macOS | `metal` | Apple Metal API |
| Linux | `vulkan` | Vulkan API |
| Windows | `dx12` or `vulkan` | DirectX 12 or Vulkan |

## Adapter Limits

Limits define what your GPU can do. Check limits before creating resources.

```javascript
const limits = adapter.getLimits()

console.log('Max buffer size:', limits.maxBufferSize)
console.log('Max texture dimension:', limits.maxTextureDimension2D)
console.log('Max compute workgroup size:', limits.maxComputeWorkgroupSizeX)
console.log('Max bind groups:', limits.maxBindGroups)
```

### Common Limits

```javascript
const limits = adapter.getLimits()

// Buffer limits
limits.maxBufferSize                    // Usually 4GB
limits.maxStorageBufferBindingSize     // Usually 128MB

// Texture limits
limits.maxTextureDimension2D            // Usually 8192 or 16384
limits.maxTextureDimension3D            // Usually 2048

// Compute limits
limits.maxComputeWorkgroupSizeX         // Usually 256
limits.maxComputeWorkgroupSizeY         // Usually 256
limits.maxComputeWorkgroupSizeZ         // Usually 64
limits.maxComputeInvocationsPerWorkgroup // Usually 256

// Binding limits
limits.maxBindGroups                    // Usually 4
limits.maxStorageBuffersPerShaderStage  // Usually 8
```

### Checking Limits

```javascript
const limits = adapter.getLimits()

// Check before creating large buffer
const bufferSize = 1024 * 1024 * 500 // 500 MB
if (bufferSize > limits.maxBufferSize) {
  throw new Error(`Buffer too large: ${bufferSize} > ${limits.maxBufferSize}`)
}

// Check texture dimensions
const textureSize = 16384
if (textureSize > limits.maxTextureDimension2D) {
  throw new Error(`Texture too large: ${textureSize} > ${limits.maxTextureDimension2D}`)
}
```

## Features

Features are optional GPU capabilities. Check support before using.

```javascript
const features = adapter.getFeatures()
console.log('Supported features:', features)

// Check specific feature
if (features.includes('texture-compression-bc')) {
  console.log('✅ BC texture compression supported')
}
```

### Common Features

- `'texture-compression-bc'` - BC texture compression (Windows)
- `'texture-compression-etc2'` - ETC2 compression (Mobile)
- `'texture-compression-astc'` - ASTC compression (Mobile)
- `'depth-clip-control'` - Depth clipping control
- `'timestamp-query'` - GPU timing queries

## Creating Device

Once you have an adapter, create a device to start GPU operations.

```javascript
const device = adapter.requestDevice({
  label: 'My GPU Device',
  requiredLimits: {
    maxBufferSize: 1024 * 1024 * 100 // Request 100 MB
  },
  requiredFeatures: []
})
```

See [Device](/api/device) for more details.

## Best Practices

### 1. Check Adapter Availability

```javascript
const adapters = gpu.enumerateAdapters()
if (adapters.length === 0) {
  throw new Error('No GPU found')
}
```

### 2. Validate Limits

```javascript
const limits = adapter.getLimits()
const requiredBufferSize = 1024 * 1024 * 500

if (requiredBufferSize > limits.maxBufferSize) {
  console.warn('Required buffer size exceeds GPU limit')
  // Fall back to smaller buffer or CPU processing
}
```

### 3. Log Adapter Info

```javascript
const info = adapter.getInfo()
console.log(`Using GPU: ${info.name} (${info.backend})`)
```

### 4. Request Appropriate Power Preference

```javascript
// For servers/workstations
const adapter = gpu.requestAdapter({
  powerPreference: 'high-performance'
})

// For laptops/battery devices
const adapter = gpu.requestAdapter({
  powerPreference: 'low-power'
})
```

## Example: Selecting Best GPU

```javascript
const { Gpu } = require('@sylphx/webgpu')

function selectBestGPU() {
  const gpu = Gpu.create()
  const adapters = gpu.enumerateAdapters()

  if (adapters.length === 0) {
    throw new Error('No GPU found')
  }

  // Prefer discrete GPU (usually has more VRAM)
  const discrete = adapters.find(adapter => {
    const info = adapter.getInfo()
    return info.deviceType === 'discrete-gpu'
  })

  return discrete || adapters[0]
}

const adapter = selectBestGPU()
const info = adapter.getInfo()
console.log(`Selected: ${info.name}`)
```

## Next Steps

- Learn about [Buffers](/guide/buffers) →
- Explore [Device API](/api/device) →
