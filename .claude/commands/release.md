---
name: release
description: Publish new version and monitor release process
agent: coder
---

# Release & Publish

Prepare, publish, and monitor package release.

## Pre-Release Checks

**Quality Gates:**
- [ ] All tests pass
- [ ] No lint errors
- [ ] Build successful
- [ ] No security vulnerabilities
- [ ] Dependencies up to date
- [ ] CHANGELOG updated
- [ ] README accurate
- [ ] Breaking changes documented

**Version Decision:**
- Breaking changes → `major`
- New features → `minor` (default)
- Bug fixes → `patch`

## Release Process

### For TypeScript/JavaScript Projects

1. **Create Changeset:**
   ```bash
   bunx changeset
   ```
   - Select package
   - Choose version bump type
   - Write clear summary

2. **Version Bump:**
   ```bash
   bunx changeset version
   ```
   - Updates package.json
   - Updates CHANGELOG.md
   - Consumes changeset

3. **Commit & Push:**
   ```bash
   git add -A
   git commit -m "chore(release): <package>@<version>"
   git push
   ```

4. **Monitor CI:**
   ```bash
   gh run list --workflow=release --limit 5
   gh run watch <run-id>
   ```

5. **Verify Publication:**
   ```bash
   npm view <package>@<version>
   ```

### For Other Projects

1. Update version in manifest (package.json, setup.py, etc.)
2. Update CHANGELOG
3. Create git tag
4. Push tag to trigger release
5. Monitor CI/CD

## Post-Release

- [ ] Verify package published
- [ ] Test installation: `npm install <package>@latest`
- [ ] Create GitHub release with notes
- [ ] Announce (if public package)
- [ ] Close related issues/PRs

## Troubleshooting

**CI fails on install:**
- Update lockfile locally: `bun install`
- Commit and push

**Tests fail in CI:**
- Run tests locally: `npm test`
- Fix issues, commit, push

**Build fails:**
- Check build locally: `npm run build`
- Fix errors, commit, push

## Exit Criteria

- [ ] Package published successfully
- [ ] CI workflow completed
- [ ] GitHub release created
- [ ] Version verified on registry
- [ ] Installation tested

Report: Version number, publish time, registry URL.
