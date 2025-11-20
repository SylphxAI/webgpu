---
name: cleanup
description: Clean technical debt, refactor, optimize, and simplify codebase
agent: coder
---

# Cleanup & Refactor

Scan codebase for technical debt and code smells. Clean, refactor, optimize.

## Scope

**Code Smells:**
- Functions >20 lines → extract
- Duplication (3+ occurrences) → DRY
- Complexity (>3 nesting levels) → flatten
- Unused code/imports/variables → remove
- Commented code → delete
- Magic numbers → named constants
- Poor naming → clarify

**Technical Debt:**
- TODOs/FIXMEs → implement or delete
- Deprecated APIs → upgrade
- Outdated patterns → modernize
- Performance bottlenecks → optimize
- Memory leaks → fix
- Lint warnings → resolve

**Optimization:**
- Algorithm complexity → reduce
- N+1 queries → batch
- Unnecessary re-renders → memoize
- Large bundles → code split
- Unused dependencies → remove

## Execution

1. **Scan** entire codebase systematically
2. **Prioritize** by impact (critical → major → minor)
3. **Clean** incrementally with atomic commits
4. **Test** after every change
5. **Report** what was cleaned and impact

## Commit Strategy

One commit per logical cleanup:
- `refactor(auth): extract validateToken function`
- `perf(db): batch user queries to fix N+1`
- `chore: remove unused imports and commented code`

## Exit Criteria

- [ ] No code smells remain
- [ ] All tests pass
- [ ] Lint clean
- [ ] Performance improved (measure before/after)
- [ ] Technical debt reduced measurably

Report: Lines removed, complexity reduced, performance gains.
