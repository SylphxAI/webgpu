# Triangle Example

Rendering a colored triangle using the render pipeline.

## Code

```javascript
const { Gpu, BufferUsage, TextureUsage } = require('@sylphx/webgpu')

async function renderTriangle() {
  // Setup GPU
  const gpu = Gpu.create()
  const adapter = gpu.requestAdapter({ powerPreference: 'high-performance' })
  const device = adapter.requestDevice()

  console.log('Using GPU:', adapter.getInfo().name)

  // Create shader
  const shader = device.createShaderModule(`
    struct VertexInput {
      @location(0) position: vec2<f32>,
      @location(1) color: vec3<f32>
    }

    struct VertexOutput {
      @builtin(position) position: vec4<f32>,
      @location(0) color: vec3<f32>
    }

    @vertex
    fn vs_main(input: VertexInput) -> VertexOutput {
      var output: VertexOutput;
      output.position = vec4(input.position, 0.0, 1.0);
      output.color = input.color;
      return output;
    }

    @fragment
    fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
      return vec4(color, 1.0);
    }
  `, 'Triangle Shader')

  // Vertex data (position + color, interleaved)
  const vertices = new Float32Array([
    // x     y      r     g     b
     0.0,  0.5,   1.0,  0.0,  0.0,  // Top (red)
    -0.5, -0.5,   0.0,  1.0,  0.0,  // Bottom left (green)
     0.5, -0.5,   0.0,  0.0,  1.0   // Bottom right (blue)
  ])

  const vertexBuffer = device.createBuffer(
    vertices.byteLength,
    BufferUsage.VERTEX | BufferUsage.COPY_DST,
    false
  )

  device.queueWriteBuffer(vertexBuffer, 0, Buffer.from(vertices.buffer))

  // Create pipeline
  const pipeline = device.createRenderPipeline({
    label: 'Triangle Pipeline',

    vertex: {
      module: shader,
      entryPoint: 'vs_main',
      buffers: [
        {
          arrayStride: 20,  // 5 floats * 4 bytes
          attributes: [
            {
              shaderLocation: 0,  // position
              offset: 0,
              format: 'float32x2'
            },
            {
              shaderLocation: 1,  // color
              offset: 8,
              format: 'float32x3'
            }
          ]
        }
      ]
    },

    fragment: {
      module: shader,
      entryPoint: 'fs_main',
      targets: [
        {
          format: 'rgba8unorm'
        }
      ]
    },

    primitive: {
      topology: 'triangle-list',
      frontFace: 'ccw',
      cullMode: 'none'
    },

    multisample: {
      count: 1
    }
  })

  // Create render target
  const width = 800
  const height = 600

  const texture = device.createTexture({
    label: 'Render Target',
    size: { width, height, depthOrArrayLayers: 1 },
    format: 'rgba8unorm',
    usage: TextureUsage.RENDER_ATTACHMENT | TextureUsage.COPY_SRC,
    dimension: '2d',
    mipLevelCount: 1,
    sampleCount: 1
  })

  const textureView = texture.createView()

  // Render
  const encoder = device.createCommandEncoder('Render Triangle')

  const pass = encoder.beginRenderPass({
    label: 'Triangle Render Pass',
    colorAttachments: [
      {
        view: textureView,
        loadOp: 'clear',
        storeOp: 'store',
        clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }  // Dark gray background
      }
    ]
  })

  pass.setPipeline(pipeline)
  pass.setVertexBuffer(0, vertexBuffer)
  pass.draw(3, 1, 0, 0)  // 3 vertices, 1 instance
  pass.end()

  device.queueSubmit(encoder.finish())
  device.poll(true)

  console.log('✅ Triangle rendered!')
  console.log(`   Resolution: ${width}x${height}`)
  console.log('   Colors: Red (top), Green (bottom-left), Blue (bottom-right)')

  // Cleanup
  vertexBuffer.destroy()
  texture.destroy()
  device.destroy()
}

renderTriangle().catch(console.error)
```

## Running

```bash
node examples/triangle.js
```

## Expected Output

```
Using GPU: Apple M1 Pro
✅ Triangle rendered!
   Resolution: 800x600
   Colors: Red (top), Green (bottom-left), Blue (bottom-right)
```

## How It Works

### 1. Vertex Shader

Transforms vertex positions and passes colors to fragment shader:

```wgsl
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
  var output: VertexOutput;
  output.position = vec4(input.position, 0.0, 1.0);
  output.color = input.color;
  return output;
}
```

- Input: 2D position + RGB color
- Output: 4D position (clip space) + RGB color

### 2. Fragment Shader

Colors each pixel:

```wgsl
@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
  return vec4(color, 1.0);
}
```

- Input: Interpolated color from vertex shader
- Output: RGBA color

### 3. Vertex Data Layout

Interleaved vertex data:

```javascript
const vertices = new Float32Array([
  // position   color
   0.0,  0.5,   1.0,  0.0,  0.0,  // Vertex 0
  -0.5, -0.5,   0.0,  1.0,  0.0,  // Vertex 1
   0.5, -0.5,   0.0,  0.0,  1.0   // Vertex 2
])
```

Layout configuration:
```javascript
{
  arrayStride: 20,  // 5 floats * 4 bytes per vertex
  attributes: [
    { shaderLocation: 0, offset: 0, format: 'float32x2' },   // position
    { shaderLocation: 1, offset: 8, format: 'float32x3' }    // color
  ]
}
```

### 4. Render Pass

Clear screen and render triangle:

```javascript
const pass = encoder.beginRenderPass({
  colorAttachments: [
    {
      view: textureView,
      loadOp: 'clear',               // Clear before rendering
      storeOp: 'store',              // Save result
      clearValue: { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }
    }
  ]
})

pass.setPipeline(pipeline)
pass.setVertexBuffer(0, vertexBuffer)
pass.draw(3, 1, 0, 0)  // Draw 3 vertices
pass.end()
```

## Variations

### Solid Color Triangle

```wgsl
@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4(1.0, 0.0, 0.0, 1.0);  // Red
}
```

Remove color attribute from vertex buffer.

### Textured Triangle

Add UV coordinates:

```javascript
const vertices = new Float32Array([
  // position   uv
   0.0,  0.5,   0.5, 0.0,
  -0.5, -0.5,   0.0, 1.0,
   0.5, -0.5,   1.0, 1.0
])
```

Sample texture in fragment shader:

```wgsl
@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(myTexture, mySampler, uv);
}
```

### Indexed Triangle

Use index buffer to share vertices:

```javascript
const vertices = new Float32Array([
  -0.5, -0.5,  // 0: Bottom left
   0.5, -0.5,  // 1: Bottom right
   0.5,  0.5,  // 2: Top right
  -0.5,  0.5   // 3: Top left
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
pass.setIndexBuffer(indexBuffer, 'uint16')
pass.drawIndexed(6, 1, 0, 0, 0)  // 6 indices
```

### Animated Triangle

Rotate using uniforms:

```wgsl
struct Uniforms {
  time: f32
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
  let angle = uniforms.time;
  let cos_a = cos(angle);
  let sin_a = sin(angle);

  let rotated = vec2(
    position.x * cos_a - position.y * sin_a,
    position.x * sin_a + position.y * cos_a
  );

  return vec4(rotated, 0.0, 1.0);
}
```

Update uniform each frame:

```javascript
for (let frame = 0; frame < 1000; frame++) {
  const time = frame * 0.016  // 60 FPS
  const uniformData = new Float32Array([time])

  device.queueWriteBuffer(uniformBuffer, 0, Buffer.from(uniformData.buffer))

  // Render frame...
}
```

## Saving to Image

To save the rendered triangle to a PNG file, see [Rendering Guide - Saving Rendered Image](/guide/rendering#saving-rendered-image).

## See Also

- [Rendering Guide](/guide/rendering)
- [Pipelines Guide](/guide/pipelines)
- [Shaders Guide](/guide/shaders)
- [More Examples](/examples/)
