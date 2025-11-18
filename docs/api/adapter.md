# Adapter

An Adapter represents a physical GPU and provides information about its capabilities.

## Getting Adapter

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = gpu.requestAdapter({
  powerPreference: 'high-performance'
})
```

## Methods

### `adapter.requestDevice(descriptor)`

Creates a Device for GPU operations.

**Parameters:**
- `descriptor` (Object, optional)
  - `label` (String, optional): Debug label
  - `requiredLimits` (Object, optional): Required limit values
  - `requiredFeatures` (Array, optional): Required features

**Returns:** `Device`

**Example:**
```javascript
const device = adapter.requestDevice({
  label: 'My GPU Device',
  requiredLimits: {
    maxBufferSize: 1024 * 1024 * 100  // 100 MB
  },
  requiredFeatures: []
})
```

### `adapter.getInfo()`

Returns information about the adapter.

**Returns:** `AdapterInfo`

**AdapterInfo Properties:**
- `name` (String): GPU name (e.g., "NVIDIA GeForce RTX 3080")
- `backend` (String): Backend API ("metal", "vulkan", "dx12")
- `vendorId` (Number): Vendor identifier
- `deviceId` (Number): Device identifier
- `vendorName` (String): Vendor name (e.g., "NVIDIA", "AMD", "Apple")
- `deviceType` (String): Device type ("discrete-gpu", "integrated-gpu", "cpu", "unknown")

**Example:**
```javascript
const info = adapter.getInfo()

console.log('GPU:', info.name)
console.log('Backend:', info.backend)
console.log('Vendor:', info.vendorName)
console.log('Type:', info.deviceType)
```

### `adapter.getLimits()`

Returns the adapter's hardware limits.

**Returns:** `AdapterLimits`

**AdapterLimits Properties:**

**Buffer Limits:**
- `maxBufferSize` (Number): Maximum buffer size in bytes
- `maxStorageBufferBindingSize` (Number): Maximum storage buffer binding size
- `maxUniformBufferBindingSize` (Number): Maximum uniform buffer binding size

**Texture Limits:**
- `maxTextureDimension1D` (Number): Maximum 1D texture dimension
- `maxTextureDimension2D` (Number): Maximum 2D texture dimension
- `maxTextureDimension3D` (Number): Maximum 3D texture dimension
- `maxTextureArrayLayers` (Number): Maximum texture array layers

**Compute Limits:**
- `maxComputeWorkgroupSizeX` (Number): Maximum X dimension of workgroup
- `maxComputeWorkgroupSizeY` (Number): Maximum Y dimension of workgroup
- `maxComputeWorkgroupSizeZ` (Number): Maximum Z dimension of workgroup
- `maxComputeInvocationsPerWorkgroup` (Number): Maximum threads per workgroup
- `maxComputeWorkgroupsPerDimension` (Number): Maximum workgroups per dimension

**Binding Limits:**
- `maxBindGroups` (Number): Maximum bind groups
- `maxBindingsPerBindGroup` (Number): Maximum bindings per bind group
- `maxStorageBuffersPerShaderStage` (Number): Maximum storage buffers per stage
- `maxUniformBuffersPerShaderStage` (Number): Maximum uniform buffers per stage
- `maxSamplersPerShaderStage` (Number): Maximum samplers per stage
- `maxStorageTexturesPerShaderStage` (Number): Maximum storage textures per stage
- `maxSampledTexturesPerShaderStage` (Number): Maximum sampled textures per stage

**Example:**
```javascript
const limits = adapter.getLimits()

console.log('Max buffer size:', limits.maxBufferSize)
console.log('Max texture 2D:', limits.maxTextureDimension2D)
console.log('Max workgroup size:', limits.maxComputeWorkgroupSizeX)

// Validate before creating resources
const bufferSize = 1024 * 1024 * 500  // 500 MB
if (bufferSize > limits.maxBufferSize) {
  throw new Error(`Buffer too large: ${bufferSize} > ${limits.maxBufferSize}`)
}
```

### `adapter.getFeatures()`

Returns supported optional features.

**Returns:** `Array<String>`

**Common Features:**
- `'texture-compression-bc'` - BC texture compression (Windows)
- `'texture-compression-etc2'` - ETC2 compression (Mobile)
- `'texture-compression-astc'` - ASTC compression (Mobile)
- `'depth-clip-control'` - Depth clipping control
- `'timestamp-query'` - GPU timing queries
- `'indirect-first-instance'` - First instance in indirect draws
- `'depth32float-stencil8'` - 32-bit float depth + 8-bit stencil

**Example:**
```javascript
const features = adapter.getFeatures()

console.log('Supported features:', features)

if (features.includes('texture-compression-bc')) {
  console.log('✅ BC compression supported')
}

if (features.includes('timestamp-query')) {
  console.log('✅ GPU timing queries supported')
}
```

## Backend Types

| Backend | Platform | Description |
|---------|----------|-------------|
| `metal` | macOS, iOS | Apple Metal API |
| `vulkan` | Linux, Windows, Android | Vulkan API |
| `dx12` | Windows | DirectX 12 |

## Device Types

| Type | Description |
|------|-------------|
| `discrete-gpu` | Dedicated GPU card |
| `integrated-gpu` | Integrated GPU (CPU) |
| `cpu` | CPU-based software renderer |
| `unknown` | Unknown type |

## Example: Complete Adapter Info

```javascript
const { Gpu } = require('@sylphx/webgpu')

const gpu = Gpu.create()
const adapter = gpu.requestAdapter()

if (!adapter) {
  throw new Error('No adapter found')
}

// Basic info
const info = adapter.getInfo()
console.log('=== GPU Information ===')
console.log('Name:', info.name)
console.log('Backend:', info.backend)
console.log('Vendor:', info.vendorName || 'Unknown')
console.log('Type:', info.deviceType)

// Limits
const limits = adapter.getLimits()
console.log('\n=== Limits ===')
console.log('Max buffer size:', (limits.maxBufferSize / 1024 / 1024).toFixed(0), 'MB')
console.log('Max texture 2D:', limits.maxTextureDimension2D)
console.log('Max workgroup size:', limits.maxComputeWorkgroupSizeX)
console.log('Max bind groups:', limits.maxBindGroups)

// Features
const features = adapter.getFeatures()
console.log('\n=== Features ===')
if (features.length === 0) {
  console.log('No optional features supported')
} else {
  features.forEach(feature => {
    console.log('✅', feature)
  })
}
```

## Checking Compatibility

```javascript
function checkCompatibility(adapter) {
  const limits = adapter.getLimits()

  const requirements = {
    maxBufferSize: 1024 * 1024 * 100,  // 100 MB
    maxTextureDimension2D: 4096,
    maxComputeWorkgroupSizeX: 256
  }

  for (const [key, required] of Object.entries(requirements)) {
    const available = limits[key]
    if (available < required) {
      throw new Error(
        `GPU does not meet requirements: ${key} (${available} < ${required})`
      )
    }
  }

  console.log('✅ GPU meets all requirements')
}

checkCompatibility(adapter)
```

## TypeScript

```typescript
import { Adapter, AdapterInfo, AdapterLimits } from '@sylphx/webgpu'

const adapter: Adapter = gpu.requestAdapter()!

const info: AdapterInfo = adapter.getInfo()
const limits: AdapterLimits = adapter.getLimits()
const features: string[] = adapter.getFeatures()
```

## See Also

- [GPU](/api/gpu)
- [Device](/api/device)
- [GPU & Adapters Guide](/guide/gpu-adapters)
