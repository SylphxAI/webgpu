# @sylphx/webgpu

`webgpu` is a production foundation repository for the `@sylphx/webgpu` native
WebGPU package. It owns the Rust/wgpu native implementation, JavaScript and
TypeScript package surface, platform-specific native packages, tests,
examples, documentation site, CI matrix, and release workflow for Node.js and
Bun consumers.

## Lifecycle And Layer

- Lifecycle: `production`
- Layer: `foundation`

## Goals

- Provide a standards-aligned WebGPU implementation for Node.js and Bun through
  documented package exports.
- Keep Rust native bindings, JavaScript wrappers, TypeScript declarations,
  platform packages, examples, tests, docs, and release artifacts coherent.
- Publish consumer-neutral WebGPU behavior and reproducible compatibility
  claims.

## Non-Goals

- Own one consumer application's rendering policy, shader library, asset
  pipeline, GPU scheduling policy, or benchmark narrative.
- Own the WebGPU standard, wgpu upstream behavior, browser behavior, or all GPU
  platform driver issues.
- Publish enterprise doctrine, org rulesets, rollout issue reconciliation, or
  shared CI policy.

## Boundaries

This repository owns the `@sylphx/webgpu` package family and native build
artifacts. Consumers must depend on documented package exports and examples,
not internal Rust modules, unpublished native files, or private CI artifacts.

## Public Surfaces

- `README.md`, `docs/`, `CONTRIBUTING.md`, `ROADMAP.md`, and `CHANGELOG.md`
  document the package, API, examples, and release history.
- `docs/adr/` records durable package contracts, including the backend boundary
  for Python-class numerical performance consumers.
- `package.json`, `index.js`, `webgpu.js`, `index.d.ts`, and `webgpu.d.ts`
  define the package surface.
- `src/` defines native Rust implementation exposed through napi.
- `npm/*/package.json` define platform package surfaces.
- `examples/` and `test/` provide executable compatibility evidence.
- `.github/workflows/ci.yml`, `test.yml`, and `release.yml` define build,
  test, and release paths.
- `.doctrine/project.json` is the machine-readable project manifest.

## Delivery

Pull requests run legacy native build and test workflows across supported
platforms. Main pushes run the release workflow, including native artifact
builds and central reusable release handling. Published package and native
artifact changes require CI, artifact readback, package readback, and consumer
smoke evidence because source revert alone does not unpublish packages.

The authoritative control-plane record is `.doctrine/project.json`.
