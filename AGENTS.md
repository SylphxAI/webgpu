# webgpu — local agent notes only

Static engineering and delivery standards load from the active Skills runtime
([SylphxAI/skills](https://github.com/SylphxAI/skills) is binding instruction
SSOT). Doctrine and Mission Control are retired historical lineage and must not
be loaded as current instruction authority.

Local truth: `PROJECT.md`, `` when present.

## Boundary hazards

- Never commit secrets, tokens, `.env` files, or credentials.

## Local commands

```bash
python3 /Users/kyle/.doctrine/scripts/project-control-plane-audit.py --local . --fail-on-drift --json
git diff --check
```

## Validation notes

- Prefer the **narrowest** affected check before full workspace runs.
- Report layers honestly: local diff · trunk FF · deploy · prod proof (do not collapse).

## Backend false-authority fence

Work: wi_01KYFN6993PMG8WD00Q51AE231

If this repository has completed a **Rust backend** cutover:

1. Production backend behavior authority is the Rust crate/binary/service path declared in deploy manifests / package native bin / Docker ENTRYPOINT / `sylphx.toml`.
2. Residual TypeScript service trees are **not** product authority unless explicitly proven still on the live path.
3. Do not "fix production" by editing residual TypeScript and assuming runtime will pick it up.
4. Prefer deleting residual TS backend trees after Rust sole proof; keep history in Git.
5. Intentional TypeScript frontends, npm packaging wrappers, and native-binding surfaces may remain.
