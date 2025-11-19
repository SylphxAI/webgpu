---
"@sylphx/webgpu": major
---

**BREAKING CHANGE**: Fix ExternalObject serialization by accepting GPU objects as separate parameters

## Problem Fixed
Resolves critical napi-rs limitation where `External<T>` types cannot be deserialized when nested in `#[napi(object)]` descriptor structs. This caused all descriptor methods to fail with "Failed to get external value" errors.

## API Changes

All methods that previously accepted GPU objects (pipelines, layouts, modules) nested in descriptor objects now require them as separate parameters:

### createPipelineLayout
```javascript
// Before (v0.6.1 - broken):
device.createPipelineLayout({ bindGroupLayouts: [layout1, layout2] })

// After (v0.7.0 - working):
device.createPipelineLayout({ label: 'my-layout' }, [layout1, layout2])
```

### createComputePipeline
```javascript
// Before (v0.6.1 - broken):
device.createComputePipeline({
  layout: pipelineLayout,
  compute: { module: shaderModule, entryPoint: 'main' }
})

// After (v0.7.0 - working):
device.createComputePipeline(
  { label: 'my-pipeline', entryPoint: 'main' },
  pipelineLayout,  // separate parameter (or null for auto)
  shaderModule     // separate parameter
)
```

### createRenderPipeline
```javascript
// Before (v0.6.1 - broken):
device.createRenderPipeline({
  layout: pipelineLayout,
  vertex: { module: vertexModule, entryPoint: 'vs_main', ... },
  fragment: { module: fragmentModule, entryPoint: 'fs_main', ... }
})

// After (v0.7.0 - working):
device.createRenderPipeline(
  {
    label: 'my-render-pipeline',
    vertex: { entryPoint: 'vs_main', ... },
    fragment: { entryPoint: 'fs_main', ... }
  },
  pipelineLayout,    // separate parameter
  vertexModule,      // separate parameter
  fragmentModule     // separate parameter (or null if no fragment shader)
)
```

### createBindGroup (all variants)
```javascript
// Before (v0.6.1 - broken):
device.createBindGroup({ layout: bindGroupLayout }, entries)

// After (v0.7.0 - working):
device.createBindGroup({ label: 'my-bind-group' }, bindGroupLayout, entries)
```

## Migration Guide
Update all descriptor method calls to pass GPU objects as separate parameters instead of nesting them in the descriptor object. The descriptor object should now only contain primitive values (strings, numbers, booleans) and nested plain objects.

## Verification
All integration tests pass:
✅ Pipeline layout creation with separate bindGroupLayouts
✅ Compute pipeline with separate layout and module
✅ Compute pipeline with auto layout (null)
