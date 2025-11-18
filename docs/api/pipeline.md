# Pipeline

Pipelines define how shaders execute and how data flows through the GPU.

## Compute Pipeline

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

### Compute Pipeline Descriptor

**Properties:**
- `label` (String, optional): Debug label
- `compute` (Object):
  - `module` (ShaderModule): Compiled shader
  - `entryPoint` (String): Entry function name

### Example

```javascript
const shader = device.createShaderModule(`
  @group(0) @binding(0) var<storage, read> input: array<f32>;
  @group(0) @binding(1) var<storage, read_write> output: array<f32>;

  @compute @workgroup_size(64)
  fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;
    if (index >= arrayLength(&output)) {
      return;
    }
    output[index] = input[index] * 2.0;
  }
`)

const pipeline = device.createComputePipeline({
  label: 'Multiply by 2',
  compute: {
    module: shader,
    entryPoint: 'main'
  }
})
```

## Render Pipeline

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

### Render Pipeline Descriptor

**vertex** (Object):
- `module` (ShaderModule): Vertex shader
- `entryPoint` (String): Entry function name
- `buffers` (Array): Vertex buffer layouts

**fragment** (Object, optional):
- `module` (ShaderModule): Fragment shader
- `entryPoint` (String): Entry function name
- `targets` (Array): Render target configurations

**primitive** (Object):
- `topology` (String): Primitive topology
- `frontFace` (String): `'ccw'` or `'cw'`
- `cullMode` (String): `'none'`, `'front'`, or `'back'`

**depthStencil** (Object, optional):
- `format` (String): Depth/stencil format
- `depthWriteEnabled` (Boolean): Enable depth writes
- `depthCompare` (String): Depth comparison function

**multisample** (Object):
- `count` (Number): Sample count (1, 4, etc.)

## Vertex Buffer Layout

### Single Attribute

```javascript
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
```

### Multiple Attributes (Interleaved)

```javascript
buffers: [
  {
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
  }
]
```

### Vertex Formats

| Format | Type | Components |
|--------|------|------------|
| `'float32'` | `f32` | 1 |
| `'float32x2'` | `vec2<f32>` | 2 |
| `'float32x3'` | `vec3<f32>` | 3 |
| `'float32x4'` | `vec4<f32>` | 4 |
| `'uint32'` | `u32` | 1 |
| `'uint32x2'` | `vec2<u32>` | 2 |
| `'uint32x3'` | `vec3<u32>` | 3 |
| `'uint32x4'` | `vec4<u32>` | 4 |
| `'sint32'` | `i32` | 1 |
| `'sint32x2'` | `vec2<i32>` | 2 |
| `'sint32x3'` | `vec3<i32>` | 3 |
| `'sint32x4'` | `vec4<i32>` | 4 |

## Primitive Topology

- `'point-list'` - Each vertex is a point
- `'line-list'` - Each pair is a line
- `'line-strip'` - Connected lines
- `'triangle-list'` - Each triplet is a triangle
- `'triangle-strip'` - Connected triangles

## Blend Modes

### Alpha Blending (Transparent)

```javascript
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
```

### Additive Blending (Lights)

```javascript
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
```

### Blend Factors

- `'zero'` - 0
- `'one'` - 1
- `'src'` - Source color
- `'one-minus-src'` - 1 - source
- `'src-alpha'` - Source alpha
- `'one-minus-src-alpha'` - 1 - source alpha
- `'dst'` - Destination color
- `'one-minus-dst'` - 1 - destination
- `'dst-alpha'` - Destination alpha
- `'one-minus-dst-alpha'` - 1 - destination alpha

### Blend Operations

- `'add'` - src + dst
- `'subtract'` - src - dst
- `'reverse-subtract'` - dst - src
- `'min'` - min(src, dst)
- `'max'` - max(src, dst)

## Depth/Stencil

```javascript
depthStencil: {
  format: 'depth24plus',
  depthWriteEnabled: true,
  depthCompare: 'less',
  stencilFront: {
    compare: 'always',
    failOp: 'keep',
    depthFailOp: 'keep',
    passOp: 'keep'
  },
  stencilBack: {
    compare: 'always',
    failOp: 'keep',
    depthFailOp: 'keep',
    passOp: 'keep'
  }
}
```

### Depth Compare Functions

- `'never'` - Never pass
- `'less'` - Pass if new < existing
- `'equal'` - Pass if new == existing
- `'less-equal'` - Pass if new ≤ existing
- `'greater'` - Pass if new > existing
- `'not-equal'` - Pass if new != existing
- `'greater-equal'` - Pass if new ≥ existing
- `'always'` - Always pass

## Methods

### `pipeline.getBindGroupLayout(index)`

Gets the bind group layout at the specified index.

**Parameters:**
- `index` (Number): Bind group index (0-3)

**Returns:** `BindGroupLayout`

**Example:**
```javascript
const layout = pipeline.getBindGroupLayout(0)

const bindGroup = device.createBindGroup({
  layout: layout,
  entries: [
    { binding: 0, resource: { buffer: bufferA } },
    { binding: 1, resource: { buffer: bufferB } }
  ]
})
```

## Complete Example

### Compute Pipeline

```javascript
const { Gpu, BufferUsage } = require('@sylphx/webgpu')

// Setup
const gpu = Gpu.create()
const adapter = gpu.requestAdapter()
const device = adapter.requestDevice()

// Shader
const shader = device.createShaderModule(`
  @group(0) @binding(0) var<storage, read> a: array<f32>;
  @group(0) @binding(1) var<storage, read> b: array<f32>;
  @group(0) @binding(2) var<storage, read_write> result: array<f32>;

  @compute @workgroup_size(64)
  fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;
    if (index >= arrayLength(&result)) {
      return;
    }
    result[index] = a[index] + b[index];
  }
`)

// Pipeline
const pipeline = device.createComputePipeline({
  label: 'Vector Add',
  compute: {
    module: shader,
    entryPoint: 'main'
  }
})

// Buffers
const bufferA = device.createBuffer(20, BufferUsage.STORAGE | BufferUsage.COPY_DST, false)
const bufferB = device.createBuffer(20, BufferUsage.STORAGE | BufferUsage.COPY_DST, false)
const bufferResult = device.createBuffer(20, BufferUsage.STORAGE, false)

// Write data
const a = new Float32Array([1, 2, 3, 4, 5])
const b = new Float32Array([10, 20, 30, 40, 50])
device.queueWriteBuffer(bufferA, 0, Buffer.from(a.buffer))
device.queueWriteBuffer(bufferB, 0, Buffer.from(b.buffer))

// Bind group
const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: { buffer: bufferA } },
    { binding: 1, resource: { buffer: bufferB } },
    { binding: 2, resource: { buffer: bufferResult } }
  ]
})

// Execute
const encoder = device.createCommandEncoder()
const pass = encoder.beginComputePass()
pass.setPipeline(pipeline)
pass.setBindGroup(0, bindGroup)
pass.dispatchWorkgroups(1, 1, 1)
pass.end()

device.queueSubmit(encoder.finish())
device.poll(true)

console.log('✅ Computation complete')
```

### Render Pipeline

```javascript
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

const pipeline = device.createRenderPipeline({
  label: 'Triangle',
  vertex: {
    module: shader,
    entryPoint: 'vs_main',
    buffers: [{
      arrayStride: 8,
      attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' }]
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
```

## TypeScript

```typescript
import { ComputePipeline, RenderPipeline } from '@sylphx/webgpu'

const computePipeline: ComputePipeline = device.createComputePipeline({
  compute: { module: shader, entryPoint: 'main' }
})

const renderPipeline: RenderPipeline = device.createRenderPipeline({
  vertex: { /* ... */ },
  fragment: { /* ... */ },
  primitive: { topology: 'triangle-list' },
  multisample: { count: 1 }
})
```

## See Also

- [Device](/api/device)
- [Command Encoder](/api/command-encoder)
- [Pipelines Guide](/guide/pipelines)
- [Compute Guide](/guide/compute)
- [Rendering Guide](/guide/rendering)
