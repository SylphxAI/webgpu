# GPU

The GPU instance is the entry point to WebGPU. It provides methods to discover and request GPU adapters.

## Creating GPU Instance

```javascript
const { Gpu } = require('@sylphx/webgpu')
const gpu = Gpu.create()
```

## Methods

### `Gpu.create()`

Creates a GPU instance.

**Returns:** `Gpu`

**Example:**
```javascript
const gpu = Gpu.create()
```

### `gpu.requestAdapter(options)`

Requests a GPU adapter with specified options.

**Parameters:**
- `options` (Object, optional)
  - `powerPreference` (String, optional): `'high-performance'` or `'low-power'`

**Returns:** `Adapter` or `null` if no adapter available

**Example:**
```javascript
const adapter = gpu.requestAdapter({
  powerPreference: 'high-performance'
})

if (!adapter) {
  throw new Error('No GPU adapter found')
}
```

### `gpu.enumerateAdapters()`

Returns all available GPU adapters.

**Returns:** `Array<Adapter>`

**Example:**
```javascript
const adapters = gpu.enumerateAdapters()

console.log(`Found ${adapters.length} GPU(s)`)

adapters.forEach((adapter, i) => {
  const info = adapter.getInfo()
  console.log(`GPU ${i}: ${info.name} (${info.backend})`)
})
```

## Power Preference

### `'high-performance'`
Prefer discrete GPU with maximum performance. Higher power consumption.

**Use when:**
- Running compute-intensive workloads
- Maximum performance required
- Power consumption not a concern

### `'low-power'`
Prefer integrated GPU with lower power consumption. Adequate performance for most tasks.

**Use when:**
- Battery life is important
- Workload is not compute-intensive
- Running on laptop or mobile device

### `undefined` (default)
Let the system decide based on system policy.

## Example: Selecting GPU

```javascript
const { Gpu } = require('@sylphx/webgpu')

function selectBestGPU() {
  const gpu = Gpu.create()
  const adapters = gpu.enumerateAdapters()

  if (adapters.length === 0) {
    throw new Error('No GPU found')
  }

  // Log all available GPUs
  console.log(`Available GPUs:`)
  adapters.forEach((adapter, i) => {
    const info = adapter.getInfo()
    console.log(`  ${i}: ${info.name} (${info.backend})`)
  })

  // Prefer high-performance adapter
  const adapter = gpu.requestAdapter({
    powerPreference: 'high-performance'
  })

  if (!adapter) {
    // Fall back to first available
    return adapters[0]
  }

  return adapter
}

const adapter = selectBestGPU()
const device = adapter.requestDevice()
```

## Platform-Specific Behavior

### macOS
- Usually 1 adapter (Metal)
- Integrated + discrete GPUs share same Metal adapter
- System automatically chooses GPU based on workload

### Linux
- May have multiple adapters (Vulkan)
- Separate adapters for integrated and discrete GPUs
- Manual selection may be required

### Windows
- May have multiple adapters (DirectX 12 or Vulkan)
- Separate adapters for integrated and discrete GPUs
- `powerPreference` helps select appropriate GPU

## Error Handling

```javascript
try {
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter()

  if (!adapter) {
    throw new Error('No GPU adapter available')
  }

  const device = adapter.requestDevice()
} catch (err) {
  console.error('GPU initialization failed:', err)
  // Fall back to CPU or show error
}
```

## TypeScript

```typescript
import { Gpu, Adapter } from '@sylphx/webgpu'

const gpu: Gpu = Gpu.create()
const adapter: Adapter | null = gpu.requestAdapter({
  powerPreference: 'high-performance'
})
```

## See Also

- [Adapter](/api/adapter)
- [Device](/api/device)
- [GPU & Adapters Guide](/guide/gpu-adapters)
