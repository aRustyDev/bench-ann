# Plan: Resource & Timeline — Vector DS&A

> **Bead**: `research-cfj.2.13.4`
> **Status**: draft
> **Author**: aRustyDev + Claude
> **Date**: 2026-04-24

## Objective

Estimate effort, identify external resources, and set milestones for the Vector DS&A research initiative.

## Prerequisites

- [x] Research questions approved (`research-cfj.2.13.1` + `research-cfj.2.13.5`)
- [x] Scope approved (`research-cfj.2.13.2` + `research-cfj.2.13.6`)
- [ ] Methodology approved (`research-cfj.2.13.3` + `research-cfj.2.13.7`)

## Phases & Effort Estimates

| Phase | Sessions | Notes |
|-------|----------|-------|
| Phase 0: Research Planning | 1 (this session) | Nearly complete |
| Phase 1: R&E Run 1 (broad) | 1-2 | Search matrices + broad survey |
| Phase 1: R&E Run 2 (refine) | 1-2 | Gap-fill, define metrics, refine taxonomy |
| Phase 1: R&E Run N (if needed) | 0-1 | Only if convergence not met |
| Phase 2: Cohort definition + human review | 1 | Taxonomy → cohorts → approval |
| Phase 2: Targeted research per cohort | 1-2 per cohort, est. 3-4 cohorts | 3-8 sessions total |
| Phase 2: Cross-cohort synthesis | 1 | Compare category winners |
| Phase 2.5: Benchmark design + execution | 2-3 | Harness setup + benchmark runs |
| Phase 2.75: Synthesis & frameworks | 1 | Decision framework, deployment trade-offs |
| Phase 3: Codebase analysis planning | 1 | Plan + human review |
| Phase 4: Codebase analysis execution | 2-4 | Multiple analysis runs |
| **Total** | **~14-24 sessions** | Spread across multiple weeks |

## External Resources Required

### Tools & Access
- **crates.io / lib.rs / docs.rs** — no auth needed
- **GitHub** — for cloning candidate repos (codebase analysis)
- **ann-benchmarks.com** — reference benchmark data
- **arXiv / Semantic Scholar** — academic papers (no auth needed)
- **search-term-matrices skill** — installed at `~/.claude/skills/search-term-matrices`

### Benchmark Infrastructure
- Local machine for benchmarking (document CPU, RAM, OS)
- Standard datasets: SIFT1M (~128d, 1M vectors), GloVe-100 (~100d), or synthetic at 384d/768d/1536d
- Rust toolchain with criterion for benchmarks
- Candidate crate repos cloned locally

### Knowledge Artifacts
- `docs/src/vector-dsa/knowledge/` — glossary, taxonomy, references (YAML, updated incrementally)
- Beads database (`beads_research`) for issue tracking

## Milestones

| Milestone | Trigger | Definition of Done |
|-----------|---------|-------------------|
| **M1: R&E Converged** | All convergence criteria met | Stable taxonomy, complete crate matrix, operationalized metrics |
| **M2: Cohorts Defined** | Taxonomy → cohort mapping approved | Each category has a cohort epic with assigned crates |
| **M3: Targeted Complete** | All cohort syntheses produced | Per-category rankings with grounded evidence |
| **M4: Benchmarks Complete** | ≥3 candidates benchmarked | Measured data across target configurations |
| **M5: Frameworks Produced** | Q4 + Q5 deliverables exist | Decision framework + deployment trade-off profiles |
| **M6: Codebase Analysis Complete** | Top candidates analyzed | Architecture patterns documented with file:line references |
| **M7: Research Complete** | All findings docs produced | All Q1-Q6 have grounded answers, knowledge artifacts stable |

## Risk Register

| Risk | Mitigation | Owner |
|------|-----------|-------|
| Session budget exceeds estimate | R&E capped at 4 runs; cohorts can be parallelized | Research lead |
| Benchmark hardware inconsistency | Document hardware in every report; use relative comparisons | Executor |
| Candidate crate disappears mid-research | Tag versions at research start; note if abandoned | Executor |

## Definition of Done

- [ ] All milestones M1-M7 achieved
- [ ] All knowledge artifacts (glossary, taxonomy, references) in stable state
- [ ] All findings docs produced with appropriate grounding
- [ ] Git commits capture all research artifacts

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-24 | Initial draft | aRustyDev + Claude |
