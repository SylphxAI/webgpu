# ADR-PROPOSED — Fleet WebGPU Rust North Star architecture

- **Status:** Proposed
- **Date:** 2026-07-10
- **Relates to:** ADR-167 (SylphxAI/doctrine), `@sylphx/numpy` / `@sylphx/torch` GPU substrate
- **Change class:** `required-future` for WebGPU native foundation; `advisory` for fleet

## Context

`@sylphx/webgpu` is a production-ready WebGPU implementation for Node.js and Bun:
standards-aligned GPU device/queue/buffer/pipeline semantics via Rust/wgpu native
bindings (napi-rs). JavaScript (`webgpu.js`, `index.js`) is an auto-generated loader
and standards wrapper — not backend authority. The crate powers the SylphxAI
Python-to-TypeScript ML stack as consumer-neutral GPU substrate.

Unlike application repos, WebGPU is a **published native foundation** — Rust has
always been runtime authority for GPU hot paths. The migration ledger tracks **8
capabilities** across device/queue, buffer mapping, render pipeline, compute,
constants, and npm publish surfaces. rej-010 re-audit downgraded prior `ts_deleted`
claims pending SHA-bound differential proof. Central doctrine
[ADR-167](https://github.com/SylphxAI/doctrine/blob/main/docs/adr/ADR-167-boundary-contract-stack-and-platform-pillars.md)
classifies native GPU bindings as Rust-first foundation.

Cutover posture here is **proof hardening**, not greenfield rewrite: preserve
WebGPU-standard API contracts, npm publish identity, and bun test gate parity.

## Decision

### 1. North Star production stack (WebGPU repo)

| Layer | North Star | Transitional (until sunset slice) |
| --- | --- | --- |
| Cross-boundary contract | Rust napi exports + `webgpu.d.ts` generated types | None (npm API is the contract) |
| GPU device/queue/adapter | Rust `src/gpu.rs`, `device.rs`, `queue.rs`, `adapter.rs` | None — always Rust authority |
| Buffer mapping semantics | Rust `src/buffer.rs` | `webgpu.js` unmap flush wrapper only |
| Render/compute pipelines | Rust `src/pipeline.rs`, `render_pass.rs`, `compute_pass.rs` | None |
| Constants / limits | Rust `src/constants.rs` | None |
| JS standard wrapper | `webgpu.js` (thin projection) | unchanged consumer surface |
| Proto/admin surfaces | N/A (no cross-repo proto today) | Add Buf only if ops RPC needed |
| Publish artifacts | npm `@sylphx/webgpu` + NAPI prebuilds | unchanged release path |

### 2. Ownership matrix

| Concern | Owner | WebGPU may | WebGPU must not |
| --- | --- | --- | --- |
| Native WebGPU bindings, wgpu integration | **SylphxAI/webgpu** | Own Rust crate, npm package, CI gates | Embed product tensor semantics |
| NumPy/Torch GPU consumers | **SylphxAI/** sibling repos | Consume `@sylphx/webgpu` as substrate | Fork GPU core into product repos |
| Engineering doctrine, fleet audits | **SylphxAI/doctrine** | Run conformance audits | Fork standards into repo prose |

### 3. Strangler-fig cutover posture

Foundation repo — slices are **differential-proof** not runtime migration:

- **S0:** `cargo test`/`clippy` + `npm test` bun gate green.
- **S1:** Constants differential — native SSOT vs `webgpu.js` wrapper slice.
- **S2:** GPU device/queue/buffer mapping differential corpus.
- **S3:** Render/compute pipeline differential vs standards test suite.
- **S4:** Re-claim `ts_deleted` only after `differential_green` + npm publish readback.
- Each slice requires N4 (`cargo test` + `cargo clippy -D warnings`) per fleet cutover registry.

### 4. Contract stack (ADR-167 alignment)

- **Rust/wgpu** is SSOT for all GPU authority; JS is generated loader + standard wrapper.
- **Connect/gRPC** not required unless future cross-repo ops surfaces land.
- **TypeScript `.d.ts`** are type projections — not backend authority.
- Hand-written duplicate GPU implementations in TS are rejected.

## Alternatives considered

| Alternative | Why rejected |
| --- | --- |
| TypeScript GPU backend revival | Violates foundation design; no TS GPU core exists |
| Skip differential proof | rej-010 promotion freeze requires SHA-bound harness |
| Product tensor logic in `src/gpu.rs` | Violates substrate boundary; consumers own semantics |

## Consequences

- All GPU authority remains in `src/*.rs`; JS is wrapper and types only.
- `docs/specs/migration-ledger.json` tracks proof status, not greenfield migration percent.
- Prior `ts_deleted` claims stay reverted until differential_green at bound SHAs.
- npm published versions require standards test suite reproduction in CI.

## Validation

- `docs/specs/migration-ledger.json` — capability `state`, `proof.status`, `promotionHold`
- Differential harness: `scripts/run-webgpu-differential.sh` + `tests/webgpu_differential.rs`
- `test/webgpu-standard.test.ts`, `test/mapped-buffer.test.js` bun gates
- npm publish readback: `@sylphx/webgpu` digest matches release workflow
- `python3 $DOCTRINE/scripts/project-control-plane-audit.py --local . --fail-on-drift`
- `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` (N4 gate)