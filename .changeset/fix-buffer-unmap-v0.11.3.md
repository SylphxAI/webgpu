---
"@sylphx/webgpu": patch
---

fix: critical buffer.unmap() bug - ensure getMappedRange() modifications are flushed to GPU

Previous versions (0.9.1-0.11.2) published with old binaries due to cargo cache. This release includes:
- Properly rebuilt native binaries with buffer.rs fix
- Rust 1.81 compatibility (downgraded dependencies)
- Fixed Linux ARM64 cross-compilation target installation
