---
name: improve
description: Proactively discover and implement improvements
agent: coder
---

# Proactive Improvement

Analyze project comprehensively. Discover improvement opportunities. Prioritize and execute.

## Discovery Areas

**Code Quality:**
- Code complexity hotspots
- Duplication patterns
- Test coverage gaps
- Error handling weaknesses
- Type safety improvements
- Naming inconsistencies

**Architecture:**
- Tight coupling
- Missing abstractions
- Scalability bottlenecks
- State management issues
- API design improvements
- Module organization

**Performance:**
- Slow algorithms (profiling data)
- Database query inefficiencies
- Bundle size optimization
- Memory usage patterns
- Network request optimization
- Caching opportunities

**Security:**
- Vulnerability scan (dependencies)
- Input validation gaps
- Authentication/authorization weaknesses
- Sensitive data exposure
- OWASP top 10 check
- Security best practices

**Developer Experience:**
- Development setup complexity
- Build time optimization
- Testing speed
- Debugging capabilities
- Error messages clarity
- Documentation gaps

**Maintenance:**
- Outdated dependencies
- Deprecated API usage
- Missing tests
- Incomplete documentation
- Configuration complexity
- Technical debt accumulation

**Features:**
- Missing functionality (user feedback)
- Integration opportunities
- Automation potential
- Monitoring/observability
- Error recovery
- User experience improvements

## Analysis Process

1. **Scan** codebase comprehensively
2. **Profile** performance bottlenecks
3. **Check** security vulnerabilities
4. **Review** dependencies for updates
5. **Analyze** test coverage
6. **Assess** documentation completeness
7. **Evaluate** architectural patterns
8. **Identify** missing features

## Prioritization Matrix

**Impact vs Effort:**
- **High Impact + Low Effort** → Do first
- **High Impact + High Effort** → Plan carefully
- **Low Impact + Low Effort** → Quick wins
- **Low Impact + High Effort** → Skip or defer

**Categories:**
- **Critical:** Security, data loss, crashes
- **High:** Performance degradation, poor UX
- **Medium:** Code quality, maintainability
- **Low:** Nice-to-haves, polish

## Execution Plan

For each improvement:
1. **Describe** what and why
2. **Estimate** effort (small/medium/large)
3. **Assess** impact (low/medium/high)
4. **Prioritize** using matrix
5. **Create** implementation plan

Then execute top priorities:
- Start with high-impact, low-effort
- Make atomic commits
- Test thoroughly
- Document changes

## Output Format

```markdown
## Improvement Opportunities

### Critical (Do Now)
1. [Security] Update vulnerable dependency X (CVE-XXXX)
   - Impact: High (prevents exploit)
   - Effort: Low (5 min)
   - Action: `npm update X`

### High Priority
1. [Performance] Optimize user query (N+1 problem)
   - Impact: High (10x speedup)
   - Effort: Medium (2 hrs)
   - Action: Batch queries with JOIN

### Quick Wins
1. [DX] Add pre-commit hooks
   - Impact: Medium (catch errors early)
   - Effort: Low (30 min)
   - Action: Setup husky + lint-staged

### Planned Improvements
1. [Feature] Add caching layer
   - Impact: High (reduce server load)
   - Effort: High (1 week)
   - Action: Design cache strategy, implement Redis

## Execution Plan
1. Fix security vulnerability (now)
2. Optimize query performance (today)
3. Add pre-commit hooks (today)
4. Plan caching implementation (next sprint)
```

## Exit Criteria

- [ ] Comprehensive analysis completed
- [ ] Opportunities prioritized
- [ ] High-priority improvements implemented
- [ ] Tests pass
- [ ] Impact measured
- [ ] Documentation updated

Report: Improvements discovered, implemented, impact metrics.
