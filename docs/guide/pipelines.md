# Pipelines

Pipelines define how shaders are executed and how data flows through the GPU.

## Compute Pipelines

Compute pipelines run compute shaders for parallel computation.

### Creating Compute Pipeline

```javascript
const pipeline = device.createComputePipeline({
  label: 'Vector Add Pipeline',
  compute: {
    module: shaderModule,
    entryPoint: 'main'
  }
})
```

### Bind Group Layout

```javascript
const bindGroupLayout = pipeline.getBindGroupLayout(0)

const bindGroup = device.createBindGroup({
  layout: bindGroupLayout,
  entries: [
    { binding: 0, resource: { buffer: inputBuffer } },
    { binding: 1, resource: { buffer: outputBuffer } }
  ]
})
```

### Dispatching Compute

```javascript
const encoder = device.createCommandEncoder()
const pass = encoder.beginComputePass()

pass.setPipeline(pipeline)
pass.setBindGroup(0, bindGroup)
pass.dispatchWorkgroups(
  Math.ceil(arrayLength / 64),  // x
  1,                            // y
  1                             // z
)

pass.end()
device.queueSubmit(encoder.finish())
```

## Render Pipelines

Render pipelines define vertex and fragment shader execution for rendering.

### Creating Render Pipeline

```javascript
const pipeline = device.createRenderPipeline({
  label: 'Triangle Pipeline',

  vertex: {
    module: shaderModule,
    entryPoint: 'vs_main',
    buffers: [
      {
        arrayStride: 12,  // 3 floats * 4 bytes
        attributes: [
          {
            shaderLocation: 0,
            offset: 0,
            format: 'float32x3'
          }
        ]
      }
    ]
  },

  fragment: {
    module: shaderModule,
    entryPoint: 'fs_main',
    targets: [
      {
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
      }
    ]
  },

  primitive: {
    topology: 'triangle-list',
    frontFace: 'ccw',
    cullMode: 'none'
  },

  depthStencil: undefined,
  multisample: {
    count: 1
  }
})
```

### Vertex Buffer Layouts

Define how vertex data is laid out:

```javascript
// Single attribute (position)
{
  arrayStride: 12,  // 3 * 4 bytes
  attributes: [
    {
      shaderLocation: 0,
      offset: 0,
      format: 'float32x3'
    }
  ]
}

// Multiple attributes (position + color)
{
  arrayStride: 24,  // 6 * 4 bytes
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
}
```

### Primitive Topology

- `'point-list'` - Each vertex is a point
- `'line-list'` - Each pair of vertices is a line
- `'line-strip'` - Connected lines
- `'triangle-list'` - Each 3 vertices form a triangle
- `'triangle-strip'` - Connected triangles

### Blend Modes

```javascript
// Alpha blending (transparent)
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

// Additive blending (lights)
blend: {
  color: {
    srcFactor: 'one',
    dstFactor: 'one',
    operation: 'add'
  },
  alpha: {
    srcFactor: 'one',
    dstFactor: 'one',
    operation: 'add'
  }
}

// No blending (opaque)
blend: undefined
```

## Render Pass

```javascript
const encoder = device.createCommandEncoder()

const pass = encoder.beginRenderPass({
  colorAttachments: [
    {
      view: textureView,
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
  ],
  depthStencilAttachment: undefined
})

pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.draw(3, 1, 0, 0)  // 3 vertices, 1 instance

pass.end()
device.queueSubmit(encoder.finish())
```

### Load/Store Operations

**Load Op** (what to do with existing content):
- `'clear'` - Clear to clearValue
- `'load'` - Keep existing content

**Store Op** (what to do after rendering):
- `'store'` - Save to texture
- `'discard'` - Don't save (faster if not needed)

## Depth Testing

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

### Depth Compare Functions

- `'less'` - Pass if new < existing (default)
- `'less-equal'` - Pass if new ≤ existing
- `'greater'` - Pass if new > existing
- `'greater-equal'` - Pass if new ≥ existing
- `'equal'` - Pass if new == existing
- `'not-equal'` - Pass if new != existing
- `'always'` - Always pass
- `'never'` - Never pass

## Multisampling (MSAA)

Anti-aliasing by rendering to multisampled texture:

```javascript
const msaaTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 4  // 4x MSAA
})

const resolveTexture = device.createTexture({
  size: { width: 800, height: 600, depthOrArrayLayers: 1 },
  format: 'rgba8unorm',
  usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
  dimension: '2d',
  mipLevelCount: 1,
  sampleCount: 1
})

const pipeline = device.createRenderPipeline({
  // ...
  multisample: {
    count: 4  // Must match texture
  }
})

const pass = encoder.beginRenderPass({
  colorAttachments: [
    {
      view: msaaTexture.createView(),
      resolveTarget: resolveTexture.createView(),  // Resolve to this
      loadOp: 'clear',
      storeOp: 'store',
      clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
  ]
})
```

## Index Buffers

Rendering with indexed vertices:

```javascript
const vertices = new Float32Array([
  -1.0, -1.0,  // Vertex 0
   1.0, -1.0,  // Vertex 1
   1.0,  1.0,  // Vertex 2
  -1.0,  1.0   // Vertex 3
])

const indices = new Uint16Array([
  0, 1, 2,  // Triangle 1
  0, 2, 3   // Triangle 2
])

const vertexBuffer = device.createBuffer(
  vertices.byteLength,
  BufferUsage.VERTEX | BufferUsage.COPY_DST,
  false
)

const indexBuffer = device.createBuffer(
  indices.byteLength,
  BufferUsage.INDEX | BufferUsage.COPY_DST,
  false
)

device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))
device.queueWriteBuffer(indexBuffer, 0, Buffer.from(indices.buffer))

// Render pass
pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.setIndexBuffer(indexBuffer, 'uint16')
pass.drawIndexed(6, 1, 0, 0, 0)  // 6 indices
```

## Indirect Drawing

Draw with GPU-generated parameters:

```javascript
// Indirect buffer contains: [vertexCount, instanceCount, firstVertex, firstInstance]
const indirectBuffer = device.createBuffer(
  16,  // 4 * u32
  BufferUsage.INDIRECT | BufferUsage.COPY_DST,
  false
)

const params = new Uint32Array([3, 1, 0, 0])  // Draw 3 vertices, 1 instance
device.queueWriteBuffer(indirectBuffer, 0, Buffer.from(params.buffer))

pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.drawIndirect(indirectBuffer, 0)
```

## Pipeline Caching

Pipelines are expensive to create. Cache and reuse:

```javascript
const pipelineCache = new Map()

function getPipeline(key, createFn) {
  if (!pipelineCache.has(key)) {
    pipelineCache.set(key, createFn())
  }
  return pipelineCache.get(key)
}

const pipeline = getPipeline('my-pipeline', () =>
  device.createRenderPipeline({ /* ... */ })
)
```

## Best Practices

### 1. Minimize Pipeline Switches

```javascript
// ❌ Don't: Create new pipeline every frame
for (let i = 0; i < objects.length; i++) {
  const pipeline = device.createRenderPipeline({ /* ... */ })
  pass.setPipeline(pipeline)
  // draw
}

// ✅ Do: Create once, reuse
const pipeline = device.createRenderPipeline({ /* ... */ })
for (let i = 0; i < objects.length; i++) {
  pass.setPipeline(pipeline)
  // draw
}
```

### 2. Batch by Pipeline

```javascript
// ✅ Do: Group draw calls by pipeline
for (const pipeline of pipelines) {
  pass.setPipeline(pipeline)
  for (const obj of objectsUsingPipeline(pipeline)) {
    // draw obj
  }
}
```

### 3. Use Appropriate Topology

```javascript
// ✅ Do: Use triangle-strip when possible (less vertices)
// For quad: 4 vertices instead of 6

// ❌ Don't: Use triangle-list for everything
```

### 4. Validate Shader Compatibility

```javascript
// Ensure vertex shader output matches fragment shader input
// Ensure bind group layouts match shader bindings
```

## Complete Example

```javascript
const { Gpu, BufferUsage, TextureUsage } = require('@sylphx/webgpu')

// Setup
const gpu = Gpu.create()
const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
const device = adapter.requestDevice()

// Shader
const shader = device.createShaderModule(`
  @vertex
  fn vs_main(@location(0) pos: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4(pos, 1.0);
  }

  @fragment
  fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);  // Red
  }
`, 'Triangle Shader')

// Pipeline
const pipeline = device.createRenderPipeline({
  vertex: {
    module: shader,
    entryPoint: 'vs_main',
    buffers: [{
      arrayStride: 12,
      attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }]
    }]
  },
  fragment: {
    module: shader,
    entryPoint: 'fs_main',
    targets: [{ format: 'rgba8unorm' }]
  },
  primitive: { topology: 'triangle-list' },
  multisample: { count: 1 }
})

console.log('✅ Pipeline created')
```

## Next Steps

- Learn about [Compute Shaders](/guide/compute) →
- Explore [Rendering](/guide/rendering) →
- See [Performance Tips](/guide/performance) →
