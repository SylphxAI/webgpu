# webgpu — local agent notes only

Doctrine and fleet delivery law live in the **host always-on constitution**
(`~/.grok/AGENTS.md` / Doctrine template). This file must **not** restate,
weaken, or fork that law (including PR-vs-direct-trunk delivery).

Local truth: `PROJECT.md`, `.doctrine/project.json` when present.

## Boundary hazards

- Never commit secrets, tokens, `.env` files, or credentials.

## Local commands

```bash
node --test test/project-control.node-test.mjs
npm exec --yes --package groundatlas@0.1.3 -- ga update --out .groundatlas-pilot
npm exec --yes --package groundatlas@0.1.3 -- ga audit --out .groundatlas-pilot
npm exec --yes --package groundatlas@0.1.3 -- ga manifest --out .groundatlas-pilot --json
npm exec --yes --package groundatlas@0.1.3 -- ga fleet . --out .groundatlas-pilot --require-atlas --strict --json
npm exec --yes --package groundatlas@0.1.3 -- ga fleet . --out .groundatlas-pilot --require-atlas --strict
python3 /Users/kyle/.doctrine/scripts/project-control-plane-audit.py --local . --fail-on-drift --json
git diff --check
```

## Validation notes

- Prefer the **narrowest** affected check before full workspace runs.
- Report layers honestly: local diff · trunk FF · deploy · prod proof (do not collapse).
