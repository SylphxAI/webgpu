# 001. Python Performance Backend Contract

**Status:** Accepted
**Date:** 2026-07-03

## Context

`@sylphx/webgpu` is the native Rust/wgpu WebGPU package used by TypeScript
consumers that need GPU compute in Node.js and Bun. SylphxAI's numerical and ML
libraries are moving toward Python-compatible syntax, behavior, and performance.
This package is part of that strategy, but it is not itself a NumPy or PyTorch
API layer.

The repository must stay consumer-neutral. It should provide standard WebGPU
behavior and reproducible native package evidence without encoding one numerical
library's benchmark story or hidden workload assumptions.

## Decision

`@sylphx/webgpu` is the backend substrate for Python-class GPU performance in
TypeScript runtimes.

- This package owns standards-aligned WebGPU behavior, native Rust/wgpu bindings,
  platform packages, TypeScript declarations, docs, tests, and release evidence.
- This package does not own NumPy/PyTorch syntax, tensor semantics, autograd
  semantics, or numerical-library admission gates.
- Numerical libraries such as tsnum and Axon may use this package as a GPU
  backend, but their Python API and performance parity claims must be proven in
  those repositories.
- This package may expose standard WebGPU async boundaries such as adapter
  acquisition, device acquisition, buffer mapping, and command completion.
  Numerical consumers should not leak those boundaries into every tensor
  operation; they should expose synchronous handles and explicit `sync` or
  readback awaits.
- `@sylphx/webgpu` performance claims must be stated as WebGPU/backend claims:
  dispatch overhead, buffer transfer behavior, shader execution capability,
  supported platforms, and native artifact compatibility.
- Consumer benchmarks may cite `@sylphx/webgpu` backend identity only when the
  benchmark records package version, platform package, adapter/backend info, and
  workload shape.

## Rationale

- Python-class numerical performance requires a native/GPU substrate, and this
  package provides that substrate for Node.js and Bun.
- API parity belongs to the numerical/ML library, not the WebGPU package.
- Standard WebGPU async semantics are correct at this layer, while Python-native
  numerical DX needs a higher-level queue abstraction.
- Keeping the backend consumer-neutral preserves reuse across rendering, compute,
  ML, and non-ML workloads.
- Recording backend identity prevents vague "GPU is fast" claims that cannot be
  reproduced.

## Consequences

**Positive:**

- tsnum and Axon can depend on a clear GPU backend boundary.
- The WebGPU package avoids coupling to one product workload.
- Benchmark evidence can separate backend capability from higher-level tensor
  library semantics.

**Negative:**

- `@sylphx/webgpu` alone cannot prove Python parity for a numerical library.
- Consumers must build their own workload-specific benchmark gates.
- Backend performance docs need adapter/platform metadata to be admissible.

## Validation

Backend validation remains the package's existing native/test/release path:

```bash
bun test
bun run build
```

For parity claims in numerical consumers, run the consumer repository's Python
parity benchmark instead.

## References

- Project boundary: `PROJECT.md`
- Project manifest: `.doctrine/project.json`
- Performance guide: `docs/guide/performance.md`
- Consumer parity contracts: `SylphxAI/tsnum` and `SylphxAI/axon`
