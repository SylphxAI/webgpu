---
"@sylphx/webgpu": major
---

Major v1.0 release with comprehensive API documentation

This release includes complete API documentation rewrites with 100% WebGPU standard compliance:

**Documentation Updates:**
- Complete buffer.md rewrite (699 lines) documenting all v1.0 features including overlapping range detection
- Complete device.md rewrite (883 lines) with corrected API patterns (device.queue.writeBuffer, descriptor-based createBuffer)
- Complete index.md rewrite (647 lines) with correct v1.0 examples and usage patterns
- Enhanced gpu.md (461 lines) with platform-specific behavior and comprehensive troubleshooting

**API Improvements:**
- Fixed documentation to show correct WebGPU standard API patterns
- All examples now use descriptor objects for resource creation
- Queue operations properly documented as device.queue.* methods
- Comprehensive troubleshooting sections added

**Standards Compliance:**
- 100% WebGPU specification compliance
- All non-standard APIs removed
- Overlapping range detection in getMappedRange()
- Complete state machine documentation

Total: 2,118 insertions, 544 deletions across API documentation
