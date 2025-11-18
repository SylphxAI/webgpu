# Texture

Textures are multi-dimensional arrays of image data.

## Creating Textures

```javascript
const { TextureUsage } = require('@sylphx/webgpu')

const texture = device.createTexture({
  label: 'My Texture',
  size: { width: 512, height: 512, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})
```

## Texture Formats

### Color Formats

| Format | Description | Bytes/Pixel |
|--------|-------------|-------------|
| `'r8unorm'` | 8-bit red | 1 |
| `'rg8unorm'` | 8-bit RG | 2 |
| `'rgba8unorm'` | 8-bit RGBA | 4 |
| `'rgba8unorm-srgb'` | 8-bit RGBA sRGB | 4 |
| `'bgra8unorm'` | 8-bit BGRA | 4 |
| `'bgra8unorm-srgb'` | 8-bit BGRA sRGB | 4 |
| `'r16float'` | 16-bit float red | 2 |
| `'rg16float'` | 16-bit float RG | 4 |
| `'rgba16float'` | 16-bit float RGBA | 8 |
| `'r32float'` | 32-bit float red | 4 |
| `'rg32float'` | 32-bit float RG | 8 |
| `'rgba32float'` | 32-bit float RGBA | 16 |

### Depth/Stencil Formats

| Format | Description |
|--------|-------------|
| `'depth16unorm'` | 16-bit depth |
| `'depth24plus'` | 24-bit depth (or more) |
| `'depth24plus-stencil8'` | 24-bit depth + 8-bit stencil |
| `'depth32float'` | 32-bit float depth |
| `'depth32float-stencil8'` | 32-bit float depth + 8-bit stencil |
| `'stencil8'` | 8-bit stencil |

## Texture Usage Flags

Combine with bitwise OR (`|`):

```javascript
const usage = TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST
```

### Available Flags

| Flag | Value | Description |
|------|-------|-------------|
| `COPY_SRC` | 0x01 | Can be copied from |
| `COPY_DST` | 0x02 | Can be copied to |
| `TEXTURE_BINDING` | 0x04 | Can be sampled in shaders |
| `STORAGE_BINDING` | 0x08 | Can be used as storage texture |
| `RENDER_ATTACHMENT` | 0x10 | Can be rendered to |

## Writing to Textures

### Queue Write

```javascript
const width = 256
const height = 256
const bytesPerPixel = 4

// Create RGBA data
const data = new Uint8Array(width * height * bytesPerPixel)
for (let i = 0; i < data.length; i += 4) {
  data[i] = 255      // R
  data[i + 1] = 0    // G
  data[i + 2] = 0    // B
  data[i + 3] = 255  // A
}

device.queueWriteTexture(
  {
    texture: texture,
    mipLevel: 0,
    origin: { x: 0, y: 0, z: 0 }
  },
  Buffer.from(data.buffer),
  {
    offset: 0,
    bytesPerRow: width * bytesPerPixel,
    rowsPerImage: height
  },
  {
    width: width,
    height: height,
    depthOrArrayLayers: 1
  }
)
```

### Copy from Buffer

```javascript
const encoder = device.createCommandEncoder()

encoder.copyBufferToTexture(
  {
    buffer: sourceBuffer,
    offset: 0,
    bytesPerRow: width * 4,
    rowsPerImage: height
  },
  {
    texture: texture,
    mipLevel: 0,
    origin: { x: 0, y: 0, z: 0 }
  },
  {
    width: width,
    height: height,
    depthOrArrayLayers: 1
  }
)

device.queueSubmit(encoder.finish())
```

## Reading from Textures

```javascript
// Create staging buffer
const bytesPerRow = 256 * Math.ceil((width * 4) / 256)
const buffer = device.createBuffer(
  bytesPerRow * height,
  BufferUsage.COPY_DST | BufferUsage.MAP_READ,
  false
)

// Copy texture to buffer
const encoder = device.createCommandEncoder()
encoder.copyTextureToBuffer(
  {
    texture: texture,
    mipLevel: 0,
    origin: { x: 0, y: 0, z: 0 }
  },
  {
    buffer: buffer,
    offset: 0,
    bytesPerRow: bytesPerRow,
    rowsPerImage: height
  },
  {
    width: width,
    height: height,
    depthOrArrayLayers: 1
  }
)

device.queueSubmit(encoder.finish())
device.poll(true)

// Read pixels
const mapped = await buffer.mapRead()
const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset, width * height * 4)

console.log('First pixel RGBA:', pixels[0], pixels[1], pixels[2], pixels[3])
```

## Texture Views

```javascript
const view = texture.createView({
  label: 'Texture View',
  format: 'rgba8unorm',
  dimension: '2d',
  baseMipLevel: 0,
  mipLevelCount: 1,
  baseArrayLayer: 0,
  arrayLayerCount: 1
})
```

### View Dimensions

- `'1d'` - 1D texture
- `'2d'` - 2D texture
- `'2d-array'` - Array of 2D textures
- `'cube'` - Cube map
- `'cube-array'` - Array of cube maps
- `'3d'` - 3D texture

## Methods

### `texture.createView(descriptor)`

Creates a view of the texture.

**Parameters:**
- `descriptor` (Object, optional)
  - `label` (String, optional)
  - `format` (String, optional): Texture format
  - `dimension` (String, optional): View dimension
  - `baseMipLevel` (Number, optional): First mip level
  - `mipLevelCount` (Number, optional): Number of mip levels
  - `baseArrayLayer` (Number, optional): First array layer
  - `arrayLayerCount` (Number, optional): Number of array layers

**Returns:** `TextureView`

**Example:**
```javascript
const view = texture.createView()
```

### `texture.destroy()`

Destroys the texture and releases GPU memory.

**Example:**
```javascript
texture.destroy()
```

## Samplers

Samplers control how textures are sampled in shaders:

```javascript
const sampler = device.createSampler({
  label: 'Linear Sampler',
  addressModeU: 'repeat',
  addressModeV: 'repeat',
  addressModeW: 'repeat',
  magFilter: 'linear',
  minFilter: 'linear',
  mipmapFilter: 'linear',
  lodMinClamp: 0,
  lodMaxClamp: 32,
  compare: undefined,
  maxAnisotropy: 1
})
```

### Address Modes

- `'repeat'` - Wrap texture coordinates
- `'clamp-to-edge'` - Clamp to edge
- `'mirror-repeat'` - Mirror and wrap

### Filter Modes

- `'nearest'` - Nearest neighbor (pixelated)
- `'linear'` - Bilinear interpolation (smooth)

## Usage in Shaders

```wgsl
@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(myTexture, mySampler, uv);
}
```

```javascript
const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: texture.createView() },
    { binding: 1, resource: sampler }
  ]
})
```

## Complete Example

```javascript
const { Gpu, TextureUsage, BufferUsage } = require('@sylphx/webgpu')

async function textureExample() {
  // Setup
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter()
  const device = adapter.requestDevice()

  // Create texture
  const width = 256
  const height = 256
  const texture = device.createTexture({
    size: { width, height, depthOrArrayLayers: 1 },
    format: 'rgba8unorm',
    usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST | TextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1
  })

  // Create gradient
  const data = new Uint8Array(width * height * 4)
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 4
      data[i] = (x / width) * 255      // R
      data[i + 1] = (y / height) * 255 // G
      data[i + 2] = 128                // B
      data[i + 3] = 255                // A
    }
  }

  // Upload
  device.queueWriteTexture(
    { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
    Buffer.from(data.buffer),
    { offset: 0, bytesPerRow: width * 4, rowsPerImage: height },
    { width, height, depthOrArrayLayers: 1 }
  )

  console.log('✅ Texture created and uploaded')

  // Cleanup
  texture.destroy()
  device.destroy()
}

textureExample()
```

## TypeScript

```typescript
import { Texture, TextureView, TextureUsage } from '@sylphx/webgpu'

const texture: Texture = device.createTexture({
  size: { width: 512, height: 512, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})

const view: TextureView = texture.createView()
```

## See Also

- [Device](/api/device)
- [Textures Guide](/guide/textures)
- [Rendering Guide](/guide/rendering)
