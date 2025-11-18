# Shaders

Shaders are programs that run on the GPU, written in WGSL (WebGPU Shading Language).

## WGSL Basics

WGSL is similar to Rust and HLSL:

```wgsl
// Compute shader
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;
  // Your code here
}
```

```wgsl
// Vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
  return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
```

```wgsl
// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red
}
```

## Creating Shader Modules

```javascript
const shaderModule = device.createShaderModule(`
  @compute @workgroup_size(64)
  fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Shader code
  }
`)
```

### With Label

```javascript
const shaderModule = device.createShaderModule(`
  // Shader code
`, 'My Shader')
```

## Data Types

### Scalars

```wgsl
var x: f32 = 1.5;      // 32-bit float
var y: i32 = -42;      // 32-bit signed int
var z: u32 = 100u;     // 32-bit unsigned int
var w: bool = true;    // Boolean
```

### Vectors

```wgsl
var v2: vec2<f32> = vec2(1.0, 2.0);
var v3: vec3<f32> = vec3(1.0, 2.0, 3.0);
var v4: vec4<f32> = vec4(1.0, 2.0, 3.0, 4.0);

// Swizzling
var xy: vec2<f32> = v4.xy;
var rgb: vec3<f32> = v4.rgb;
```

### Matrices

```wgsl
var m2x2: mat2x2<f32> = mat2x2(
  1.0, 0.0,
  0.0, 1.0
);

var m4x4: mat4x4<f32> = mat4x4<f32>(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 1.0, 0.0,
  0.0, 0.0, 0.0, 1.0
);
```

### Arrays

```wgsl
var arr: array<f32, 4> = array(1.0, 2.0, 3.0, 4.0);
var dynamic: array<f32>;  // Runtime-sized
```

### Structs

```wgsl
struct Particle {
  position: vec3<f32>,
  velocity: vec3<f32>,
  mass: f32
}

var p: Particle = Particle(
  vec3(0.0, 0.0, 0.0),
  vec3(1.0, 0.0, 0.0),
  1.0
);
```

## Bindings

### Storage Buffers

```wgsl
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;
```

```javascript
const bindGroup = device.createBindGroup({
  layout: pipeline.getBindGroupLayout(0),
  entries: [
    { binding: 0, resource: { buffer: inputBuffer } },
    { binding: 1, resource: { buffer: outputBuffer } }
  ]
})
```

### Uniform Buffers

```wgsl
struct Uniforms {
  time: f32,
  resolution: vec2<f32>
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
```

```javascript
const uniformBuffer = device.createBuffer(
  16,  // 4 + 8 + 4 (padding) = 16 bytes
  BufferUsage.UNIFORM | BufferUsage.COPY_DST,
  false
)
```

### Textures and Samplers

```wgsl
@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(myTexture, mySampler, uv);
}
```

## Built-in Functions

### Math

```wgsl
let x = abs(-5.0);           // 5.0
let y = sqrt(16.0);          // 4.0
let z = pow(2.0, 3.0);       // 8.0
let w = sin(3.14159);        // 0.0
let a = cos(0.0);            // 1.0
let b = min(1.0, 2.0);       // 1.0
let c = max(1.0, 2.0);       // 2.0
let d = clamp(5.0, 0.0, 1.0); // 1.0
```

### Vector Operations

```wgsl
let a = vec3(1.0, 2.0, 3.0);
let b = vec3(4.0, 5.0, 6.0);

let sum = a + b;              // vec3(5.0, 7.0, 9.0)
let dot_product = dot(a, b);  // 32.0
let cross_product = cross(a, b);
let len = length(a);          // 3.74...
let norm = normalize(a);      // Unit vector
```

## Control Flow

### If/Else

```wgsl
if (x > 0.0) {
  // Positive
} else if (x < 0.0) {
  // Negative
} else {
  // Zero
}
```

### For Loops

```wgsl
for (var i = 0u; i < 10u; i++) {
  // Loop body
}
```

### While Loops

```wgsl
var i = 0u;
while (i < 10u) {
  // Loop body
  i++;
}
```

## Compute Shader Example

Vector addition:

```wgsl
@group(0) @binding(0) var<storage, read> a: array<f32>;
@group(0) @binding(1) var<storage, read> b: array<f32>;
@group(0) @binding(2) var<storage, read_write> result: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;

  // Bounds check
  if (index >= arrayLength(&result)) {
    return;
  }

  result[index] = a[index] + b[index];
}
```

## Vertex Shader Example

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
```

## Fragment Shader Example

```wgsl
@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
  return vec4(color, 1.0);
}
```

## Memory Layout

### Alignment Rules

```wgsl
struct Particle {
  position: vec3<f32>,  // Offset 0, size 12
  // 4 bytes padding
  velocity: vec3<f32>,  // Offset 16, size 12
  // 4 bytes padding
  mass: f32,            // Offset 32, size 4
  // 12 bytes padding
}  // Total: 48 bytes (aligned to 16)
```

### Explicit Alignment

```wgsl
struct Uniforms {
  @align(16) time: f32,       // 16-byte aligned
  @align(16) resolution: vec2<f32>,
  @size(16) scale: f32        // Take 16 bytes
}
```

## Best Practices

### 1. Use Workgroup Size Wisely

```wgsl
// ✅ Do: Power of 2, typical values
@compute @workgroup_size(64)
@compute @workgroup_size(256)

// ⚠️ Avoid: Odd sizes
@compute @workgroup_size(37)
```

### 2. Bounds Checking

```wgsl
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;

  // ✅ Always check bounds
  if (index >= arrayLength(&result)) {
    return;
  }

  result[index] = compute(index);
}
```

### 3. Minimize Register Pressure

```wgsl
// ❌ Don't: Too many variables
var temp1 = a + b;
var temp2 = c + d;
var temp3 = temp1 + temp2;
var temp4 = temp3 * 2.0;
return temp4;

// ✅ Do: Reuse variables
var temp = a + b;
temp = temp + c + d;
return temp * 2.0;
```

### 4. Use Built-ins

```wgsl
// ❌ Don't: Manual implementation
fn length_manual(v: vec3<f32>) -> f32 {
  return sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

// ✅ Do: Use built-in
let len = length(v);
```

## Debugging Shaders

### Validation Errors

Shader compilation errors are reported when creating the module:

```javascript
try {
  const shader = device.createShaderModule(`
    @compute @workgroup_size(64)
    fn main() {
      let x: f32 = "hello";  // Type error
    }
  `)
} catch (err) {
  console.error('Shader compilation error:', err.message)
}
```

### Printf Debugging

Output to storage buffer:

```wgsl
@group(0) @binding(0) var<storage, read_write> debug: array<f32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let x = 42.0;
  debug[global_id.x] = x;  // Write debug value
}
```

## Next Steps

- Learn about [Pipelines](/guide/pipelines) →
- See [Compute Shader Examples](/guide/compute) →
- Explore [Rendering Examples](/guide/rendering) →
