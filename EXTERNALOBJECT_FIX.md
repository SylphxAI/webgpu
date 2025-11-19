# ExternalObject Serialization Fix

## Problem

napi-rs **cannot deserialize `External<T>` types when nested in `#[napi(object)]` structs**.

When JavaScript passes an object like:
```javascript
{  layout: pipelineLayoutObject,
  compute: { module: shaderModuleObject }
}
```

napi-rs's automatic deserialization fails with:
```
"Failed to get external value on PipelineLayoutDescriptor.bindGroupLayouts"
"Failed to get external value on ComputePipelineDescriptor.layout"
```

## Root Cause

1. `#[napi(object)]` uses automatic deserial ization
2. `External<T>` expects special N-API external values (created via `napi_create_external`)
3. When class instances are passed as object properties, they're NOT external values
4. Deserialization fails

## Solution

**Remove `External<T>` from descriptor structs and accept class instances as direct reference parameters (`&T`).**

This is the pattern already proven to work for bind groups.

### Before (Broken):
```rust
#[napi(object)]
pub struct ComputePipelineDescriptor {
    pub layout: Option<External<GpuPipelineLayout>>,  // ❌ Fails
    pub compute: ComputeStage,
}

#[napi]
pub fn create_compute_pipeline(&self, descriptor: ComputePipelineDescriptor)
```

### After (Working):
```rust
#[napi(object)]
pub struct ComputePipelineDescriptor {
    pub label: Option<String>,
    // layout and module removed
}

#[napi]
pub fn create_compute_pipeline(
    &self,
    descriptor: ComputePipelineDescriptor,
    layout: Option<&GpuPipelineLayout>,  // ✅ Reference parameter
    module: &GpuShaderModule,            // ✅ Reference parameter
    entry_point: String,
)
```

## Implementation Plan

1. Remove External fields from:
   - `PipelineLayoutDescriptor.bindGroupLayouts`
   - `ComputePipelineDescriptor.layout` and `ComputeStage.module`
   - `BindGroupDescriptor.layout`
   - `RenderPipelineDescriptor.layout`, `VertexState.module`, `FragmentState.module`

2. Update methods to accept class instances as separate `&T` parameters

3. Update JavaScript wrapper to match new signature
