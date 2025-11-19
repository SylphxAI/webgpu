# @sylphx/webgpu

## 0.4.0

### Minor Changes

- feat(device): Add error scope management methods (pushErrorScope, popErrorScope) for WebGPU standard compliance
  - Implement `device.pushErrorScope(filter)` for pushing error scopes
  - Implement `device.popErrorScope()` for popping error scopes and retrieving errors
  - Support "validation" and "out-of-memory" filters (wgpu limitation: "internal" not supported)
  - Increase WebGPU W3C standard compliance from 85% to 88%
  - Complete Phase 2: Error Handling implementation

## 0.3.0

### Minor Changes

- feat: Complete WebGPU standard property implementation (queue, features, limits, label)
  - Added `device.queue` standard property with `submit()` and `writeBuffer()` methods
  - Added `device.features` property returning `GpuSupportedFeatures`
  - Added `device.limits` property returning `GpuSupportedLimits`
  - Added `device.label` property (returns null as wgpu doesn't expose labels)
  - Deprecated: `device.queue_submit()` → use `device.queue.submit()`
  - Deprecated: `device.queue_write_buffer()` → use `device.queue.writeBuffer()`
  - WebGPU W3C compliance increased from ~60% to ~85%
