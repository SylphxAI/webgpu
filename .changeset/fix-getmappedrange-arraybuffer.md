---
"@sylphx/webgpu": patch
---

fix(getMappedRange): return ArrayBuffer for WebGPU standard compliance

**Critical Bug Fixed**: getMappedRange() now correctly returns ArrayBuffer (WebGPU standard) instead of Buffer, fixing the data loss bug where modifications weren't flushed to GPU.

**Root Cause**: Creating `new Float32Array(buffer)` creates a COPY instead of a VIEW, so modifications were lost. Returning ArrayBuffer ensures `new Float32Array(arrayBuffer)` creates a view that shares memory.

**Migration**: Change `new Float32Array(range.buffer, range.byteOffset, length)` to `new Float32Array(range, 0, length)`
