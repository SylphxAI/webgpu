# @sylphx/webgpu

`webgpu` is a production foundation repository for the `@sylphx/webgpu` native
WebGPU package. It owns the Rust/wgpu native implementation, JavaScript and
TypeScript package surface, platform-specific native packages, tests,
examples, documentation site, CI matrix, and release workflow for Node.js and
Bun consumers.

## Lifecycle And Layer

- Lifecycle: `production`
- Layer: `foundation`
- Vendor-neutral project manifest: `project.manifest.json`
- Doctrine adapter manifest: `.doctrine/project.json`

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
- `project.manifest.json` is the vendor-neutral project manifest for
  GroundAtlas and external agents.
- `.doctrine/project.json` is the Sylphx Doctrine adapter and local governance
  catalog.

## Delivery

Pull requests run a GroundAtlas project-control gate before the native build
matrix, then legacy native build and test workflows across supported platforms.
Main pushes run the release workflow, including native artifact builds and
central reusable release handling. Direct local publish is disabled; package
publication must go through `.github/workflows/release.yml`.

Published package and native artifact changes require CI, artifact readback,
`release:readback` registry verification for the main package and platform
optional dependencies, provenance/attestation evidence, changelog evidence, and
consumer smoke proof because source revert alone does not unpublish packages.

The `@sylphx/webgpu@1.0.4` release has npm registry readback evidence for the
main package and all six platform optional dependencies from Release run
`28689182140`. Future releases must preserve that readback boundary.

## Project Control

`project.manifest.json` is the vendor-neutral control file for GroundAtlas and
external agents. `.doctrine/project.json` remains the Sylphx Doctrine adapter and
local governance catalog. Generated `.groundatlas*` reports and JSON/Markdown
GroundAtlas reports are evidence and navigation read models only; they are not
source of truth.
