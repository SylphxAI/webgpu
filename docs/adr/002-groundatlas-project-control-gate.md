# ADR 002: Add GroundAtlas Project-Control Gate

## Status

Accepted

## Context

WebGPU is a public production native package. It already has native build, test,
Release, OIDC, and npm registry readback boundaries, but it did not expose a
vendor-neutral project manifest or dogfood the released GroundAtlas
package/action.

The project-control surface must not make `.doctrine/project.json` a public
default and must not make generated `.groundatlas*` reports authoritative. This
change must not alter native WebGPU behavior, package exports, platform package
artifacts, runtime tests, release artifacts, or compatibility claims.

## Decision

Add:

- a vendor-neutral `project.manifest.json`;
- a single CI `groundatlas` job using `SylphxAI/groundatlas@v0.1.2` with
  `groundatlas@0.1.2`;
- assertions that GroundAtlas selects `project.manifest.json`, reports
  `.doctrine/project.json` only as an adapter, and has zero strict fleet
  warnings/blockers;
- a small Node project-control boundary test;
- `needs: groundatlas` on the native build matrix plus an explicit failure step so existing build contexts are
  gated without duplicating GroundAtlas across every platform target.

## Consequences

- GroundAtlas package/action dogfooding gates the existing native build matrix.
- `.doctrine/project.json` remains the Sylphx Doctrine adapter and local
  governance catalog.
- Release proof remains the existing Release workflow plus `release:readback` and
  npm registry readback for changed packages.
- Generated `.groundatlas*` reports remain evidence/navigation only.
