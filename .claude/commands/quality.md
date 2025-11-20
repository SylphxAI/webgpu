---
name: quality
description: Improve test coverage, fix tests, optimize performance, reduce bundle size
agent: coder
---

# Quality Assurance

Maximize quality metrics: tests, coverage, performance, bundle size.

## Scope

**Testing:**
- Fix failing tests
- Add missing tests (.test.ts for all modules)
- Improve coverage (critical paths 100%, business logic 80%+)
- Test edge cases
- Test error paths
- Integration tests
- E2E tests (if applicable)

**Benchmarking:**
- Add benchmarks (.bench.ts for critical functions)
- Measure performance
- Identify regressions
- Optimize slow operations

**Performance:**
- Profile hot paths
- Optimize algorithms (reduce complexity)
- Fix memory leaks
- Reduce allocations
- Cache expensive operations
- Lazy load where possible

**Bundle Size:**
- Analyze bundle composition
- Remove unused dependencies
- Tree-shake effectively
- Code split strategically
- Compress assets
- Use dynamic imports

**Code Quality:**
- Fix lint errors/warnings
- Improve type safety (no `any`)
- Reduce complexity
- Improve maintainability
- Follow code standards

**Security:**
- Fix vulnerabilities
- Update dependencies
- Validate inputs
- Sanitize outputs
- Secure defaults

## Execution

### 1. Test Coverage

```bash
# Run tests with coverage
npm run test:coverage

# Identify gaps
# Add tests for uncovered code
# Focus on critical paths first
```

**Priority:**
- Critical business logic (100%)
- Error handling (100%)
- Edge cases (90%)
- Happy paths (100%)
- Integration points (80%)

### 2. Benchmarking

```bash
# Run benchmarks
npm run bench

# Compare against baseline
# Identify regressions
# Optimize bottlenecks
```

**Add benchmarks for:**
- Algorithm complexity (O(n) vs O(n²))
- Database queries
- API calls
- Data processing
- Rendering (if UI)

### 3. Performance Profiling

```bash
# Profile application
node --inspect ...
# Or use Clinic.js, 0x, etc.

# Identify bottlenecks
# Optimize hot paths
# Measure improvements
```

**Check:**
- CPU usage
- Memory usage
- I/O operations
- Network calls
- Rendering time

### 4. Bundle Analysis

```bash
# Analyze bundle
npm run build -- --analyze

# Check size
ls -lh dist/

# Identify large dependencies
# Remove unnecessary code
```

**Optimize:**
- Remove unused dependencies
- Use smaller alternatives
- Dynamic imports for large modules
- Tree-shake effectively
- Minify properly

### 5. Quality Metrics

**Track:**
- Test coverage: `npm run test:coverage`
- Bundle size: `ls -lh dist/`
- Performance: `npm run bench`
- Security: `npm audit`
- Type coverage: TypeScript strict mode
- Lint score: `npm run lint`

## Targets

**Tests:**
- Coverage: ≥80% (business logic ≥90%)
- All critical paths: 100%
- Build time: <30s
- Test time: <10s

**Performance:**
- Page load: <3s
- API response: <200ms
- Time to interactive: <5s
- Bundle size: <100KB (gzipped)

**Quality:**
- Zero lint errors
- Zero `any` types (unless justified)
- Zero security vulnerabilities (high/critical)
- Complexity score: <10 per function

## Commit Strategy

Atomic commits per improvement:
- `test(auth): add tests for token validation`
- `perf(db): optimize user query with batch loading`
- `build: reduce bundle size by 40% with code splitting`

## Exit Criteria

- [ ] All tests pass
- [ ] Coverage meets targets
- [ ] Benchmarks added for critical functions
- [ ] Performance optimized
- [ ] Bundle size reduced
- [ ] No security vulnerabilities
- [ ] Quality metrics improved

Report: Coverage %, bundle size reduction, performance gains, benchmarks added.
