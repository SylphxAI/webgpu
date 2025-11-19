# Bind Group Creation Bug in v0.8.0

## Root Cause

The `createBindGroup()` implementation has a **fundamental design flaw**. It tries to sequentially consume resources from flat arrays without knowing which binding needs which resource type.

### Current Broken Code

```rust
pub fn create_bind_group(
    &self,
    descriptor: crate::BindGroupDescriptor,
    layout: &crate::GpuBindGroupLayout,
    entries: Vec<crate::BindGroupEntry>,  // Only has: binding, offset, size
    buffers: Option<Vec<&crate::GpuBuffer>>,
    textures: Option<Vec<&crate::GpuTextureView>>,
    samplers: Option<Vec<&crate::GpuSampler>>,
) -> Result<crate::GpuBindGroup>
```

**Problem**: `BindGroupEntry` doesn't specify **which type** of resource each binding expects!

```rust
pub struct BindGroupEntry {
    pub binding: u32,
    pub offset: Option<i64>,      // For buffer bindings
    pub size: Option<i64>,        // For buffer bindings
    // ❌ MISSING: resource_type indicator!
}
```

The code tries to guess by sequentially consuming from arrays:
1. First entry → pull from buffers[0]
2. Second entry → pull from buffers[1]
3. When buffers run out → switch to textures

This breaks when bind group has **mixed resource types** in non-sequential order:
```javascript
// Real-world example:
// Binding 0: buffer
// Binding 1: texture  <- ❌ Code tries buffers[1] instead!
// Binding 2: buffer
```

## The Fix

### Option 1: Add Resource Type to BindGroupEntry (RECOMMENDED)

```rust
#[napi(object)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource_type: String,  // "buffer" | "texture" | "sampler"
    // For buffers:
    pub offset: Option<i64>,
    pub size: Option<i64>,
}
```

Then map each binding to the correct resource:
```rust
let resource = match entry.resource_type.as_str() {
    "buffer" => {
        let buf = buffers.as_ref()
            .and_then(|b| b.get(buffer_idx))
            .ok_or_else(|| Error::from_reason("Not enough buffers"))?;
        buffer_idx += 1;
        wgpu::BindingResource::Buffer(...)
    }
    "texture" => {
        let tex = textures.as_ref()
            .and_then(|t| t.get(texture_idx))
            .ok_or_else(|| Error::from_reason("Not enough textures"))?;
        texture_idx += 1;
        wgpu::BindingResource::TextureView(&tex.view)
    }
    "sampler" => {
        let samp = samplers.as_ref()
            .and_then(|s| s.get(sampler_idx))
            .ok_or_else(|| Error::from_reason("Not enough samplers"))?;
        sampler_idx += 1;
        wgpu::BindingResource::Sampler(&samp.sampler)
    }
    _ => return Err(Error::from_reason("Invalid resource type"))
};
```

### Option 2: Use Binding Numbers as Indices

Match binding numbers to resource arrays by index:

```rust
#[napi(object)]
pub struct BindGroupResource {
    pub binding: u32,
    pub buffer: Option<&GpuBuffer>,
    pub texture: Option<&GpuTextureView>,
    pub sampler: Option<&GpuSampler>,
}
```

But this STILL has the ExternalObject extraction problem!

### Option 3: Separate Methods (NOT RECOMMENDED)

Go back to three separate methods:
- `createBindGroupBuffers()`
- `createBindGroupTextures()`
- `createBindGroupSamplers()`

This only works for **homogeneous** bind groups (all same type).

## Recommended Solution

**Use Option 1** with `resource_type` field. This:
- ✅ Avoids ExternalObject serialization issues
- ✅ Supports mixed resource types
- ✅ Matches WebGPU semantics
- ✅ Clear and explicit

## Reference: WebGPU Standard

```typescript
interface GPUBindGroupEntry {
  binding: GPUIndex32;
  resource: GPUBindingResource;  // Union type
}

type GPUBindingResource =
  | GPUSampler
  | GPUTextureView
  | GPUBufferBinding;
```

We can't use union types in napi-rs, so we:
1. Use `resource_type` string to indicate which type
2. Pass actual resources as flat arrays (to avoid ExternalObject issues)
3. Map each binding to the correct resource array based on `resource_type`

## Impact

**v0.8.0 Status:**
- ❌ `createBindGroup()` fundamentally broken (fatal bug in resource mapping logic)
- ✅ `createPipelineLayout()` works
- ✅ `createComputePipeline()` works
- ✅ `beginRenderPass()` works
- ⚠️  `getBindGroupLayout()` added to Rust but NOT in TypeScript definitions

**TypeScript Definitions Issue:**
```bash
$ grep "getBindGroupLayout" src/pipeline.rs
30:    #[napi(js_name = "getBindGroupLayout")]  # ✅ EXISTS
58:    #[napi(js_name = "getBindGroupLayout")]  # ✅ EXISTS

$ grep "getBindGroupLayout" index.d.ts
# ❌ NOT FOUND - TypeScript definitions weren't regenerated!
```

**Root Cause**: Build workflow didn't regenerate TypeScript definitions from Rust annotations.

**Result**: **Cannot create bind groups** = cannot run ANY GPU operations

## Next Steps

1. **Fix createBindGroup() bug** (add `resource_type` field to BindGroupEntry)
2. **Regenerate TypeScript definitions** (rebuild native module with `napi build`)
3. **Test with real bind group** (mixed buffer + texture resources)
4. **Release v0.9.0** (patch release to fix critical bugs)
