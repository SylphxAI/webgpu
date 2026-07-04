# Repository Instructions

Start with `PROJECT.md` and `.doctrine/project.json` before changing this
repository. They define the project goal, lifecycle, boundaries, public
surfaces, delivery model, and adoption gaps.

Use `SylphxAI/doctrine` for enterprise standards. Keep `@sylphx/webgpu`
consumer-neutral: product-specific rendering policy, shader assets, benchmark
narratives, and GPU workload assumptions belong in consuming applications or
documented examples, not hidden package behavior.

For control-plane-only changes, validate with:

```bash
node --test test/project-control.node-test.mjs
npm exec --yes --package groundatlas@0.1.2 -- ga update --out .groundatlas-pilot
npm exec --yes --package groundatlas@0.1.2 -- ga audit --out .groundatlas-pilot
npm exec --yes --package groundatlas@0.1.2 -- ga manifest --out .groundatlas-pilot --json
npm exec --yes --package groundatlas@0.1.2 -- ga fleet . --out .groundatlas-pilot --require-atlas --strict --json
python3 /Users/kyle/.doctrine/scripts/project-control-plane-audit.py --local . --fail-on-drift --json
git diff --check
```

For native package changes, also run the relevant Rust, Bun, napi, test,
platform build, docs, package readback, and consumer smoke checks.

## GroundAtlas Boundary

`project.manifest.json` is the vendor-neutral GroundAtlas control file;
`.doctrine/project.json` is the Sylphx-specific adapter and generated
`.groundatlas*` reports are not SSOT. The CI native build matrix is gated by
`needs: groundatlas`; release proof remains native artifacts, Release workflow,
`release:readback`, package registry readback, and consumer smoke evidence for
changed packages.
