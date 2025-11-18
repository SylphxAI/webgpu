# Rendering

WebGPU rendering pipelines transform vertex data into pixels on screen (or texture).

## Basic Triangle

The "Hello World" of graphics:

```javascript
const { Gpu, BufferUsage, TextureUsage } = require('@sylphx/webgpu')

// Setup
const gpu = Gpu.create()
const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
const device = adapter.requestDevice()

// Shader
const shader = device.createShaderModule(`
  @vertex
  fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4(position, 0.0, 1.0);
  }

  @fragment
  fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);  // Red
  }
`)

// Vertex data
const vertices = new Float32Array([
   0.0,  0.5,  // Top
  -0.5, -0.5,  // Bottom left
   0.5, -0.5   // Bottom right
])

const vertexBuffer = device.createBuffer(
  vertices.byteLength,
  BufferUsage.VERTEX | BufferUsage.COPY_DST,
  false
)
device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

// Pipeline
const pipeline = device.createRenderPipeline({
  vertex: {
    module: shader,
    entryPoint: 'vs_main',
    buffers: [{
      arrayStride: 8,  // 2 floats * 4 bytes
      attributes: [{
        shaderLocation: 0,
        offset: 0,
        format: 'float32x2'
      }]
    }]
  },
  fragment: {
    module: shader,
    entryPoint: 'fs_main',
    targets: [{ format: 'rgba8unorm' }]
  },
  primitive: {
    topology: 'triangle-list'
  },
  multisample: {
    count: 1
  }
})

// Render target
const texture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})

const textureView = texture.createView()

// Render
const encoder = device.createCommandEncoder()
const pass = encoder.beginRenderPass({
  colorAttachments: [{
    view: textureView,
    loadOp: 'clear',
    storeOp: 'store',
    clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
  }]
})

pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.draw(3, 1, 0, 0)
pass.end()

device.queueSubmit(encoder.finish())
device.poll(true)

console.log('✅ Triangle rendered!')
```

## Vertex Data Layouts

### Position Only

```javascript
const vertices = new Float32Array([
  x1, y1, z1,
  x2, y2, z2,
  x3, y3, z3
])

// Pipeline config
buffers: [{
  arrayStride: 12,  // 3 floats * 4 bytes
  attributes: [{
    shaderLocation: 0,
    offset: 0,
    format: 'float32x3'
  }]
}]
```

### Position + Color (Interleaved)

```javascript
const vertices = new Float32Array([
  // x,    y,    z,    r,    g,    b
  -0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  // Vertex 1: Red
   0.5, -0.5,  0.0,  0.0,  1.0,  0.0,  // Vertex 2: Green
   0.0,  0.5,  0.0,  0.0,  0.0,  1.0   // Vertex 3: Blue
])

buffers: [{
  arrayStride: 24,  // 6 floats * 4 bytes
  attributes: [
    {
      shaderLocation: 0,  // Position
      offset: 0,
      format: 'float32x3'
    },
    {
      shaderLocation: 1,  // Color
      offset: 12,
      format: 'float32x3'
    }
  ]
}]
```

```wgsl
struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) color: vec3<f32>
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec3<f32>
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
  var output: VertexOutput;
  output.position = vec4(input.position, 1.0);
  output.color = input.color;
  return output;
}

@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
  return vec4(color, 1.0);
}
```

### Position + UV (Texture Coordinates)

```javascript
const vertices = new Float32Array([
  // Position      UV
  -1.0, -1.0,     0.0, 0.0,
   1.0, -1.0,     1.0, 0.0,
   1.0,  1.0,     1.0, 1.0,
  -1.0,  1.0,     0.0, 1.0
])

buffers: [{
  arrayStride: 16,  // 4 floats * 4 bytes
  attributes: [
    { shaderLocation: 0, offset: 0, format: 'float32x2' },   // Position
    { shaderLocation: 1, offset: 8, format: 'float32x2' }    // UV
  ]
}]
```

## Indexed Rendering

Reuse vertices with index buffer:

```javascript
const vertices = new Float32Array([
  -1.0, -1.0,  // 0: Bottom left
   1.0, -1.0,  // 1: Bottom right
   1.0,  1.0,  // 2: Top right
  -1.0,  1.0   // 3: Top left
])

const indices = new Uint16Array([
  0, 1, 2,  // Triangle 1
  0, 2, 3   // Triangle 2
])

const indexBuffer = device.createBuffer(
  indices.byteLength,
  BufferUsage.INDEX | BufferUsage.COPY_DST,
  false
)
device.queueWriteBuffer(indexBuffer, 0, Buffer.from(indices.buffer))

// Render
pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.setIndexBuffer(indexBuffer, 'uint16')
pass.drawIndexed(6, 1, 0, 0, 0)  // 6 indices
```

## Textures in Rendering

### Textured Quad

```wgsl
@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(myTexture, mySampler, uv);
}
```

```javascript
const sampler = device.createSampler({
  magFilter: 'linear',
  minFilter: 'linear',
  addressModeU: 'repeat',
  addressModeV: 'repeat'
})

const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: texture.createView() },
    { binding: 1, resource: sampler }
  ]
})

pass.setBindGroup(0, bindGroup)
```

## Uniforms (Transformation Matrices)

```wgsl
struct Uniforms {
  modelViewProjection: mat4x4<f32>
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
  return uniforms.modelViewProjection * vec4(position, 1.0);
}
```

```javascript
// Create uniform buffer (64 bytes for mat4x4)
const uniformBuffer = device.createBuffer(
  64,
  BufferUsage.UNIFORM | BufferUsage.COPY_DST,
  false
)

// Update matrix
const matrix = new Float32Array([
  1, 0, 0, 0,
  0, 1, 0, 0,
  0, 0, 1, 0,
  0, 0, 0, 1
])

device.queueWriteBuffer(uniformBuffer, 0, Buffer.from(matrix.buffer))

const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [{ binding: 0, resource: { buffer: uniformBuffer } }]
})
```

## Depth Testing

Enable depth testing to handle 3D occlusion:

```javascript
const depthTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'depth24plus',
  usage: TextureUsage.RENDER_ATTACHMENT,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})

const pipeline = device.createRenderPipeline({
  // ... vertex, fragment ...
  depthStencil: {
    format: 'depth24plus',
    depthWriteEnabled: true,
    depthCompare: 'less'
  }
})

const pass = encoder.beginRenderPass({
  colorAttachments: [ /* ... */ ],
  depthStencilAttachment: {
    view: depthTexture.createView(),
    depthLoadOp: 'clear',
    depthStoreOp: 'store',
    depthClearValue: 1.0
  }
})
```

## Blending (Transparency)

```javascript
fragment: {
  module: shader,
  entryPoint: 'fs_main',
  targets: [{
    format: 'rgba8unorm',
    blend: {
      color: {
        srcFactor: 'src-alpha',
        dstFactor: 'one-minus-src-alpha',
        operation: 'add'
      },
      alpha: {
        srcFactor: 'one',
        dstFactor: 'one',
        operation: 'add'
      }
    }
  }]
}
```

## Multisampling (Anti-Aliasing)

```javascript
// Create MSAA texture
const msaaTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT,
  dimension: '2d',
  sampleCount: 4  // 4x MSAA
})

// Create resolve texture (final output)
const resolveTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
  dimension: '2d',
  sampleCount: 1
})

// Pipeline must match sample count
const pipeline = device.createRenderPipeline({
  // ...
  multisample: { count: 4 }
})

// Render pass resolves MSAA to regular texture
const pass = encoder.beginRenderPass({
  colorAttachments: [{
    view: msaaTexture.createView(),
    resolveTarget: resolveTexture.createView(),
    loadOp: 'clear',
    storeOp: 'store',
    clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
  }]
})
```

## Instanced Rendering

Draw many copies efficiently:

```wgsl
struct InstanceInput {
  @location(1) position: vec3<f32>,
  @location(2) color: vec3<f32>
}

@vertex
fn vs_main(
  @location(0) vertex_position: vec3<f32>,
  @builtin(instance_index) instance_idx: u32,
  instance: InstanceInput
) -> VertexOutput {
  var output: VertexOutput;
  output.position = vec4(vertex_position + instance.position, 1.0);
  output.color = instance.color;
  return output;
}
```

```javascript
pass.draw(
  vertexCount,
  instanceCount,  // Draw multiple instances
  firstVertex,
  firstInstance
)
```

## Render Bundles

Pre-record draw commands for reuse:

```javascript
const bundleEncoder = device.createRenderBundleEncoder({
  colorFormats: ['rgba8unorm'],
  depthStencilFormat: undefined,
  sampleCount: 1
})

bundleEncoder.setPipeline(pipeline)
bundleEncoder.setVertexBuffer(0, vertexBuffer)
bundleEncoder.draw(3, 1, 0, 0)

const bundle = bundleEncoder.finish()

// Reuse in render pass
const pass = encoder.beginRenderPass({ /* ... */ })
pass.executeBundles([bundle])
pass.end()
```

## Performance Tips

### 1. Minimize State Changes

```javascript
// ✅ Do: Group by pipeline
for (const pipeline of pipelines) {
  pass.setPipeline(pipeline)
  for (const obj of objectsUsingPipeline(pipeline)) {
    pass.setVertexBuffer(0, obj.buffer)
    pass.draw(obj.count, 1, 0, 0)
  }
}
```

### 2. Use Instancing

```javascript
// ❌ Don't: Many draw calls
for (let i = 0; i < 1000; i++) {
  pass.draw(vertexCount, 1, 0, 0)
}

// ✅ Do: Single instanced draw
pass.draw(vertexCount, 1000, 0, 0)
```

### 3. Frustum Culling

Only render visible objects:

```javascript
const visibleObjects = objects.filter(obj =>
  isInFrustum(obj, camera)
)

for (const obj of visibleObjects) {
  // Render only visible
}
```

### 4. Level of Detail (LOD)

Use simpler models for distant objects:

```javascript
const distance = length(obj.position - camera.position)
const lod = distance < 10 ? highPoly :
            distance < 50 ? medPoly :
            lowPoly

pass.setVertexBuffer(0, lod.buffer)
```

## Saving Rendered Image

```javascript
// Create staging buffer
const bytesPerRow = 256 * Math.ceil((width * 4) / 256)
const bufferSize = bytesPerRow * height

const stagingBuffer = device.createBuffer(
  bufferSize,
  BufferUsage.COPY_DST | BufferUsage.MAP_READ,
  false
)

// Copy texture to buffer
const encoder = device.createCommandEncoder()
encoder.copyTextureToBuffer(
  { texture, mipLevel: 0, origin: { x: 0, y: 0, z: 0 } },
  { buffer: stagingBuffer, offset: 0, bytesPerRow, rowsPerImage: height },
  { width, height, depthOrArrayLayers: 1 }
)

device.queueSubmit(encoder.finish())
device.poll(true)

// Read pixels
const mapped = await stagingBuffer.mapRead()
const pixels = new Uint8Array(mapped.buffer, mapped.byteOffset, width * height * 4)

// Save to PNG using external library
const fs = require('fs')
const PNG = require('pngjs').PNG

const png = new PNG({ width, height })
png.data = Buffer.from(pixels)
png.pack().pipe(fs.createWriteStream('output.png'))
```

## Next Steps

- Learn about [Performance](/guide/performance) →
- See [Triangle Example](/examples/triangle) →
- Explore [Testing](/guide/testing) →
