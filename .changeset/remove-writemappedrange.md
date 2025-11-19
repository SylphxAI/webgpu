---
"@sylphx/webgpu": minor
---

**BREAKING CHANGE**: Remove non-standard `writeMappedRange()` API

The `writeMappedRange()` method has been removed as it was not part of the WebGPU standard. Use the standard `getMappedRange()` pattern instead.

**Migration Guide:**

Before:
```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.COPY_DST,
  mappedAtCreation: true
})
buffer.writeMappedRange(Buffer.from(data.buffer))
buffer.unmap()
```

After:
```javascript
const buffer = device.createBuffer({
  size: 16,
  usage: GPUBufferUsage.COPY_DST,
  mappedAtCreation: true
})
const range = buffer.getMappedRange()  // Returns ArrayBuffer
const view = new Float32Array(range)
view.set(data)
buffer.unmap()
```

**Benefits:**
- ✅ 100% WebGPU standard compliance
- ✅ Better alignment with browser WebGPU behavior
- ✅ Simpler API surface
- ✅ Comprehensive edge case tests added
