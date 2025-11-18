# Textures

Textures are multi-dimensional arrays of image data used for rendering and compute operations.

## Creating Textures

```javascript
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

### Texture Formats

Common formats:

| Format | Description | Bytes per Pixel |
|--------|-------------|-----------------|
| `'rgba8unorm'` | 8-bit RGBA, normalized | 4 |
| `'rgba8unorm-srgb'` | 8-bit RGBA, sRGB | 4 |
| `'rgba16float'` | 16-bit float RGBA | 8 |
| `'rgba32float'` | 32-bit float RGBA | 16 |
| `'r8unorm'` | 8-bit red, normalized | 1 |
| `'rg8unorm'` | 8-bit RG, normalized | 2 |
| `'depth24plus'` | 24-bit depth | 4 |
| `'depth32float'` | 32-bit float depth | 4 |

### Usage Flags

```javascript
const { TextureUsage } = require('@sylphx/webgpu')

const usage = TextureUsage.TEXTURE_BINDING |
              TextureUsage.COPY_DST |
              TextureUsage.RENDER_ATTACHMENT
```

| Flag | Description |
|------|-------------|
| `COPY_SRC` | Can be copied from |
| `COPY_DST` | Can be copied to |
| `TEXTURE_BINDING` | Can be sampled in shaders |
| `STORAGE_BINDING` | Can be used as storage texture |
| `RENDER_ATTACHMENT` | Can be rendered to |

## Writing to Textures

### Queue Write

```javascript
const width = 256
const height = 256
const bytesPerPixel = 4 // RGBA

// Create RGBA data (red image)
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
const buffer = device.createBuffer(
  width * height * 4,
  BufferUsage.COPY_SRC,
  true
)

// Write to buffer
const mapped = buffer.getMappedRange(0, width * height * 4)
const view = new Uint8Array(mapped.buffer, mapped.byteOffset)
view.set(data)
buffer.unmap()

// Copy to texture
const encoder = device.createCommandEncoder()
encoder.copyBufferToTexture(
  {
    buffer: buffer,
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
// Create buffer to read into
const buffer = device.createBuffer(
  width * height * 4,
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
    bytesPerRow: width * 4,
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

// Read buffer
const mapped = await buffer.mapRead()
const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset, width * height * 4)

// First pixel
console.log('RGBA:', pixels[0], pixels[1], pixels[2], pixels[3])
```

## Texture Views

Texture views define how a texture is accessed in shaders:

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

- `'repeat'` - Wrap coordinates
- `'clamp-to-edge'` - Clamp to edge
- `'mirror-repeat'` - Mirror and wrap

### Filter Modes

- `'nearest'` - Nearest neighbor (pixelated)
- `'linear'` - Bilinear interpolation (smooth)

## Depth Textures

For depth testing in rendering:

```javascript
const depthTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'depth24plus',
  usage: TextureUsage.RENDER_ATTACHMENT,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})
```

## Multisampled Textures

For anti-aliasing:

```javascript
const msaaTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 4  // 4x MSAA
})
```

## 3D Textures

```javascript
const texture3D = device.createTexture({
  size: { width: 64, height: 64, depthOrArrayLayers: 64 },
  format: 'rgba8unorm',
  usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST,
  dimension: '3d',
  mipLevelCount: 1,
  sampleCount: 1
})
```

## Best Practices

### 1. Use Appropriate Formats

```javascript
// ✅ Do: Use efficient formats
const colorTexture = device.createTexture({
  format: 'rgba8unorm',  // 4 bytes per pixel
  // ...
})

// ❌ Don't: Waste memory
const colorTexture = device.createTexture({
  format: 'rgba32float',  // 16 bytes per pixel - overkill for color
  // ...
})
```

### 2. Power-of-Two Sizes

For mipmaps and better performance:

```javascript
// ✅ Do: Power of two
const texture = device.createTexture({
  size: { width: 512, height: 512, depthOrArrayLayers: 1 },
  // ...
})

// ⚠️ Okay but no mipmaps
const texture = device.createTexture({
  size: { width: 500, height: 500, depthOrArrayLayers: 1 },
  // ...
})
```

### 3. Proper Bytes Per Row

Must be multiple of 256:

```javascript
const width = 100
const bytesPerPixel = 4
const bytesPerRow = Math.ceil((width * bytesPerPixel) / 256) * 256

device.queueWriteTexture(
  { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  data,
  { offset: 0, bytesPerRow, rowsPerImage: height },
  { width, height, depthOrArrayLayers: 1 }
)
```

### 4. Cleanup

```javascript
texture.destroy()
sampler.destroy()
```

## Complete Example

```javascript
const { Gpu, TextureUsage } = require('@sylphx/webgpu')

async function textureExample() {
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
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

  // Create gradient image
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

  // Upload to GPU
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

## Next Steps

- Learn about [Shaders](/guide/shaders) →
- Explore [Rendering](/guide/rendering) →
