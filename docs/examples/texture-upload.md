# Texture Upload Example

Uploading image data to GPU textures.

## Code

```javascript
const { Gpu, TextureUsage, BufferUsage } = require('@sylphx/webgpu')

async function textureUpload() {
  // Setup GPU
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
  const device = adapter.requestDevice()

  console.log('Using GPU:', adapter.getInfo().name)

  // Create texture
  const width = 256
  const height = 256

  const texture = device.createTexture({
    label: 'Gradient Texture',
    size: { width, height, depthOrArrayLayers: 1 },
    format: 'rgba8unorm',
    usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST | TextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1
  })

  // Generate gradient image
  const data = new Uint8Array(width * height * 4)

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 4

      data[i] = (x / width) * 255        // R: horizontal gradient
      data[i + 1] = (y / height) * 255   // G: vertical gradient
      data[i + 2] = 128                  // B: constant
      data[i + 3] = 255                  // A: opaque
    }
  }

  console.log('Generated gradient image:')
  console.log(`  Width: ${width}`)
  console.log(`  Height: ${height}`)
  console.log(`  Size: ${data.byteLength} bytes`)

  // Upload to GPU
  const bytesPerRow = width * 4

  device.queueWriteTexture(
    {
      texture: texture,
      mipLevel: 0,
      origin: { x: 0, y: 0, z: 0 }
    },
    Buffer.from(data.buffer),
    {
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

  console.log('✅ Texture uploaded to GPU')

  // Read back to verify
  const stagingBufferSize = 256 * Math.ceil((width * 4) / 256)  // Align to 256 bytes
  const stagingBuffer = device.createBuffer(
    stagingBufferSize * height,
    BufferUsage.COPY_DST | BufferUsage.MAP_READ,
    false
  )

  const encoder = device.createCommandEncoder()
  encoder.copyTextureToBuffer(
    {
      texture: texture,
      mipLevel: 0,
      origin: { x: 0, y: 0, z: 0 }
    },
    {
      buffer: stagingBuffer,
      offset: 0,
      bytesPerRow: stagingBufferSize,
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

  // Map and read
  const mapped = await stagingBuffer.mapRead()
  const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset)

  // Verify a few pixels
  console.log('\nVerifying pixels:')

  // Top-left (0, 0)
  const tl = 0
  console.log(`  Top-left (0, 0): RGBA(${pixels[tl]}, ${pixels[tl+1]}, ${pixels[tl+2]}, ${pixels[tl+3]})`)

  // Top-right (255, 0)
  const tr = 255 * 4
  console.log(`  Top-right (255, 0): RGBA(${pixels[tr]}, ${pixels[tr+1]}, ${pixels[tr+2]}, ${pixels[tr+3]})`)

  // Bottom-left (0, 255)
  const bl = 255 * stagingBufferSize
  console.log(`  Bottom-left (0, 255): RGBA(${pixels[bl]}, ${pixels[bl+1]}, ${pixels[bl+2]}, ${pixels[bl+3]})`)

  // Center (128, 128)
  const center = 128 * stagingBufferSize + 128 * 4
  console.log(`  Center (128, 128): RGBA(${pixels[center]}, ${pixels[center+1]}, ${pixels[center+2]}, ${pixels[center+3]})`)

  // Cleanup
  texture.destroy()
  stagingBuffer.destroy()
  device.destroy()
}

textureUpload().catch(console.error)
```

## Running

```bash
node examples/texture-upload.js
```

## Expected Output

```
Using GPU: Apple M1 Pro
Generated gradient image:
  Width: 256
  Height: 256
  Size: 262144 bytes
✅ Texture uploaded to GPU

Verifying pixels:
  Top-left (0, 0): RGBA(0, 0, 128, 255)
  Top-right (255, 0): RGBA(255, 0, 128, 255)
  Bottom-left (0, 255): RGBA(0, 255, 128, 255)
  Center (128, 128): RGBA(127, 127, 128, 255)
```

## How It Works

### 1. Create Texture

```javascript
const texture = device.createTexture({
  size: { width: 256, height: 256, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST | TextureUsage.COPY_SRC,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})
```

- `format: 'rgba8unorm'`: 8-bit RGBA, normalized (0-1)
- `COPY_DST`: Can be written to
- `COPY_SRC`: Can be read from
- `TEXTURE_BINDING`: Can be sampled in shaders

### 2. Generate Image Data

```javascript
const data = new Uint8Array(width * height * 4)

for (let y = 0; y < height; y++) {
  for (let x = 0; x < width; x++) {
    const i = (y * width + x) * 4
    data[i] = (x / width) * 255        // R gradient
    data[i + 1] = (y / height) * 255   // G gradient
    data[i + 2] = 128                  // B constant
    data[i + 3] = 255                  // A opaque
  }
}
```

Creates horizontal red gradient and vertical green gradient.

### 3. Upload to GPU

```javascript
device.queueWriteTexture(
  {
    texture: texture,
    mipLevel: 0,
    origin: { x: 0, y: 0, z: 0 }
  },
  Buffer.from(data.buffer),
  {
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
```

### 4. Read Back (Verification)

Copy texture to buffer:

```javascript
encoder.copyTextureToBuffer(
  { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  { buffer: stagingBuffer, offset: 0, bytesPerRow: alignedBytesPerRow, rowsPerImage: height },
  { width, height, depthOrArrayLayers: 1 }
)
```

Map and read:

```javascript
const mapped = await stagingBuffer.mapRead()
const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset)
```

## Variations

### Loading from PNG File

```javascript
const fs = require('fs')
const { PNG } = require('pngjs')

function loadPNG(path) {
  return new Promise((resolve, reject) => {
    fs.createReadStream(path)
      .pipe(new PNG())
      .on('parsed', function() {
        resolve({
          width: this.width,
          height: this.height,
          data: new Uint8Array(this.data)
        })
      })
      .on('error', reject)
  })
}

async function uploadPNG(device, path) {
  const image = await loadPNG(path)

  const texture = device.createTexture({
    size: { width: image.width, height: image.height, depthOrArrayLayers: 1 },
    format: 'rgba8unorm',
    usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1
  })

  device.queueWriteTexture(
    { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
    Buffer.from(image.data.buffer),
    { offset: 0, bytesPerRow: image.width * 4, rowsPerImage: image.height },
    { width: image.width, height: image.height, depthOrArrayLayers: 1 }
  )

  return texture
}
```

### Using in Shader

```wgsl
@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(myTexture, mySampler, uv);
}
```

Bind texture and sampler:

```javascript
const sampler = device.createSampler({
  addressModeU: 'repeat',
  addressModeV: 'repeat',
  magFilter: 'linear',
  minFilter: 'linear'
})

const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: texture.createView() },
    { binding: 1, resource: sampler }
  ]
})
```

### Mipmaps

Generate mipmaps for better quality at different distances:

```javascript
const mipLevelCount = Math.floor(Math.log2(Math.max(width, height))) + 1

const texture = device.createTexture({
  size: { width, height, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST | TextureUsage.RENDER_ATTACHMENT,
  dimension: '2d',
  mipLevelCount: mipLevelCount,
  sampleCount: 1
})

// Upload level 0
device.queueWriteTexture(
  { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  data,
  { offset: 0, bytesPerRow: width * 4, rowsPerImage: height },
  { width, height, depthOrArrayLayers: 1 }
)

// Generate remaining mip levels using compute shader
// (Implementation requires compute shader to downsample)
```

### Cube Map

For environment mapping:

```javascript
const cubeTexture = device.createTexture({
  size: { width: 512, height: 512, depthOrArrayLayers: 6 },  // 6 faces
  format: 'rgba8unorm',
  usage: TextureUsage.TEXTURE_BINDING | TextureUsage.COPY_DST,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})

// Upload each face
const faces = ['right', 'left', 'top', 'bottom', 'front', 'back']

for (let i = 0; i < 6; i++) {
  const faceData = loadCubeFace(faces[i])

  device.queueWriteTexture(
    {
      texture: cubeTexture,
      mipLevel: 0,
      origin: { x: 0, y: 0, z: i }  // z = face index
    },
    faceData,
    { offset: 0, bytesPerRow: 512 * 4, rowsPerImage: 512 },
    { width: 512, height: 512, depthOrArrayLayers: 1 }
  )
}

// View as cube
const cubeView = cubeTexture.createView({
  dimension: 'cube'
})
```

## Bytes Per Row Alignment

WebGPU requires `bytesPerRow` to be a multiple of 256 bytes:

```javascript
const width = 100
const bytesPerPixel = 4
const unalignedBytesPerRow = width * bytesPerPixel  // 400

// Align to 256
const bytesPerRow = Math.ceil(unalignedBytesPerRow / 256) * 256  // 512

// Allocate aligned buffer
const buffer = device.createBuffer(
  bytesPerRow * height,
  BufferUsage.COPY_DST | BufferUsage.MAP_READ,
  false
)
```

## See Also

- [Textures Guide](/guide/textures)
- [Texture API](/api/texture)
- [Rendering Guide](/guide/rendering)
- [More Examples](/examples/)
