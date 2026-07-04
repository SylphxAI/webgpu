# Project-Control Gate Spec

## Goal

Validate WebGPU's project-control and GroundAtlas adoption surfaces without
changing native WebGPU behavior, package exports, platform native artifacts,
runtime tests, release artifacts, or compatibility claims.

## Scope

The gate validates repository control-plane facts only:

- neutral identity and truth homes live in `project.manifest.json`;
- Sylphx-specific governance facts remain in `.doctrine/project.json`;
- generated `.groundatlas*` outputs are evidence/navigation only;
- native package release proof remains CI, native artifact evidence, Release
  workflow completion, `release:readback`, and npm registry readback;
- GPU runtime and compatibility claims still require native tests or documented
  compatibility evidence before behavior/claim changes.

It does not own consumer rendering policy, shader assets, GPU scheduling policy,
upstream wgpu behavior, browser WebGPU behavior, platform driver bugs, or shared
organization rulesets.

## CI Contract

`.github/workflows/ci.yml` must:

1. run a single `groundatlas` job before the native build matrix;
2. run `node --test test/project-control.node-test.mjs`;
3. run `SylphxAI/groundatlas@v0.1.2` with `package-spec:
   groundatlas@0.1.2`, `require-atlas: "true"`, and `strict: "true"`;
4. assert that GroundAtlas selects `project.manifest.json` and treats
   `.doctrine/project.json` only as an adapter;
5. upload the manifest and fleet reports as `groundatlas-package-dogfood`;
6. keep the native build matrix gated through `needs: groundatlas` and an explicit failure step when the GroundAtlas job is not successful.

## Acceptance

- `ga audit` passes after `ga update`.
- `ga manifest --json` selects `project.manifest.json`.
- `ga fleet --require-atlas --strict --json` reports one adopted project with
  zero warnings and zero blockers.
- Native build matrix jobs depend on the GroundAtlas gate.
- Existing CI, Tests, Release, and `release:readback` package boundaries remain
  the release proof for changed native packages.
