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
python3 /Users/kyle/.doctrine/scripts/project-control-plane-audit.py --local . --fail-on-drift --json
git diff --check
```

For native package changes, also run the relevant Rust, Bun, napi, test,
platform build, docs, package readback, and consumer smoke checks.
