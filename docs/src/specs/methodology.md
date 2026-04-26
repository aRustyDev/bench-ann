# SPEC: Research Methodology — Vector DS&A

> **Bead**: `research-cfj.2.13.3`
> **Status**: draft
> **Author**: aRustyDev + Claude
> **Date**: 2026-04-24

## Purpose

Defines how the Vector DS&A research is conducted: what each phase produces, what templates to use, how to determine convergence, and what quality bars apply.

## Scope

This methodology applies to the Vector DS&A initiative (`research-cfj.2`) only. Other initiatives should define their own methodology specs, using this as a reference. Assumes `research-questions.md` and `scope.md` are approved.

## Success Criteria

This methodology is successful when:
- [ ] An agent can execute all phases end-to-end without requesting clarification on process
- [ ] Every research question (Q1-Q6) has a mapped phase that produces its deliverable
- [ ] All output artifacts have a defined template, location, and quality bar
- [ ] Phase transitions are unambiguous (clear entry/exit criteria)

## Phases & Outputs

### Phase 0: Research Planning
| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Research questions | `spec.md` | `specs/research-questions.md` | Reviewed + approved |
| Scope definition | `spec.md` | `specs/scope.md` | Reviewed + approved |
| This methodology | `spec.md` | `specs/methodology.md` | Reviewed + approved |
| Resource plan | `plan.md` | `specs/resource-plan.md` | Reviewed + approved |

### Phase 1: Research & Exploration (iterative runs)
| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Search matrix per survey task | `search-matrix.md` | `runs/run-N/search-matrix-*.md` | Built before searching |
| Run report per iteration | `run-report.md` | `runs/run-N/` (multiple files) | `[inferred]` OK |
| Glossary (incremental) | `glossary.yaml` | `knowledge/glossary.yaml` | `[cited]`+ for final |
| Taxonomy (incremental) | `taxonomy.yaml` | `knowledge/taxonomy.yaml` | Convergence-assessed |
| References (incremental) | `references.yaml` | `knowledge/references.yaml` | N/A (source list) |

### Phase 2: Targeted Research (cohort-based)
| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Cohort definitions | `spec.md` | `specs/cohort-definitions.md` | Reviewed + approved |
| Crate evaluation per crate | `crate-evaluation.md` | `targeted/cohort-*/crate-name.md` | `[cited]`+ required |
| Cohort synthesis per cohort | `cohort-synthesis.md` | `targeted/cohort-*/synthesis.md` | `[cited]`+ required |
| Cross-cohort synthesis | `cohort-synthesis.md` | `findings/cross-cohort-synthesis.md` | `[cited]`+ required |

### Phase 3: Planning & Scoping (codebase analysis)
| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Analysis plan | `plan.md` | `analysis/plans/codebase-analysis-plan.md` | Reviewed + approved |

### Phase 4: Codebase Analysis
| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Analysis run reports | `codebase-analysis.md` | `analysis/codebase/run-N-*.md` | `[observed]` required |
| Synthesis | (free-form) | `analysis/codebase/synthesis.md` | `[observed]`+ |
| Final report | (free-form) | `findings/codebase-analysis-report.md` | `[observed]`+ |

## R&E Run Protocol

### What each run produces

Every R&E run covers the same dimensions at increasing depth. A run is a single focused session (or set of sessions) that produces a run report.

| Dimension | Run 1 (broad) | Run 2 (refine) | Run N (converge) |
|-----------|---------------|----------------|------------------|
| Ecosystem Survey | Cast wide net, discover candidates | Fill gaps, chase leads from Run 1 | Verify completeness |
| Resources & References | Dump everything found | Curate, rank, discard noise | Final reference set |
| Keywords & Concepts | List terms encountered | Write precise definitions | Stable glossary |
| Metrics & KPIs | Identify what to measure | Operationalize (specific thresholds) | Validated metrics |
| Taxonomy | Draft categories | Refine boundaries | Stable taxonomy |
| Scoping | Broad requirements | Narrow to what matters | Locked scope |
| Knowledge Synthesis | Capture findings + open questions | Assess convergence | Confirm convergence |

### Search matrix requirement

Before executing any survey or gap-fill search:
1. Build a search-term matrix using the `search-term-matrices` skill (if unavailable, create manually following the `search-matrix.md` template)
2. Matrix specifies: engines, queries, operators, acceptance/success criteria per tier
3. Execute tier by tier, logging results in the matrix execution log
4. Attach the matrix as an artifact alongside the run report

### Convergence criteria

R&E is "converged" when ALL of the following are true:
- [ ] Algorithm families are fully enumerated (no new families discovered in the last run)
- [ ] All relevant Rust crates are identified and categorized (no new crates discovered)
- [ ] Category boundaries are stable (taxonomy didn't change in the last run)
- [ ] Evaluation criteria are operationalized (specific metrics with thresholds)
- [ ] Candidate list per category is finalized
- [ ] Knowledge artifacts (glossary.yaml, taxonomy.yaml, references.yaml) are stable

The convergence assessment is part of each run's Knowledge Synthesis task. If convergence is not met, create another run.

## Targeted Research Protocol

### Cohort formation

After R&E converges, the taxonomy defines categories. Each category becomes a **cohort** — a group of related crates evaluated with the same methodology.

Cohort formation process:
1. Take the stable taxonomy categories from R&E
2. Assign each known crate to its category
3. Define per-cohort evaluation criteria (what matters for THIS category)
4. Create a cohort sub-epic with: plan → human review → iterative execution → synthesis

### Per-crate evaluation

Each crate in a cohort is evaluated using the `crate-evaluation.md` template. All evaluations within a cohort use the same evaluation criteria (defined in the cohort plan) so they're directly comparable.

### Cohort iteration

Each cohort follows a structured two-pass evaluation:

**Pass 1 — Initial Evaluation**:
1. Build search-term matrix (search-term-matrices skill) covering all crates in the cohort
2. Source code examination (analyze-codebase formula) for all crates
3. Initial benchmarks (execute-benchmark formula) — first parameter sweep using the shared harness
4. Draft per-crate evaluations using crate-evaluation.md template
5. Scope review (Rule #10): do findings reveal dimensions not in the evaluation framework?
6. Provenance check: did findings change prior understanding in knowledge artifacts?

**Pass 2 — Refinement**:
1. Build follow-up search-term matrix based on Pass 1 gaps
2. Targeted source code examination on specific questions from Pass 1
3. Refined benchmarks with tuned parameters — sweep the Pareto frontier
4. Update per-crate evaluations with new data
5. Scope review: any new dimensions discovered?
6. Convergence assessment

**Pass 3 — Resolution (only if needed)**:
Triggered when Pass 2 convergence fails. Focused on specific unresolved questions. Must have written justification.

**Convergence criteria for a cohort**:
- All acceptance criteria (per cohort-definitions.md) are met
- No unanswered questions from source code examination
- Benchmark results produce stable tier classifications across re-runs
- No scope expansions introduced without resolution
- All knowledge artifact changes have epistemic timeline entries (Rule #13)

### Cross-cohort synthesis

After cohorts complete (can begin as early cohorts finish): compare category winners across the taxonomy. This produces the Q6.b comparison matrix.

## Phase 2.5: Benchmarking

Benchmarking answers Q3 with `[measured]` data. It runs after targeted research identifies the top candidates but before final synthesis.

### Outputs

| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Benchmark plan | `plan.md` | `analysis/plans/benchmark-plan.md` | Reviewed + approved |
| Benchmark reports | `benchmark-report.md` | `analysis/benchmarks/` | `[measured]` required |
| Benchmark synthesis | (free-form) | `findings/benchmark-synthesis.md` | `[measured]` required |

### Benchmark protocol

1. **Design**: Define test corpus (standard datasets: SIFT1M, GloVe-100, or synthetic at target dimensionalities), metrics (recall@10, QPS, build time, memory/vector, index size), hardware specification, statistical method (warmup iterations, measurement iterations, confidence intervals)
2. **Setup**: Build harness that runs identical workloads across candidates. Validate reproducibility (same inputs → same metrics within variance).
3. **Execute**: Run each candidate across the configuration matrix (dimensionality × dataset size). Record raw criterion output.
4. **Synthesize**: Statistical comparison, produce ranking per metric, identify trade-off profiles.

### Hardware & dataset specification

- Document exact hardware (CPU model, cores, RAM, OS) in every benchmark report
- Primary datasets: at minimum one standard benchmark dataset (SIFT1M recommended) + one synthetic dataset at target dimensionality (384d or 768d)
- Published third-party benchmarks (ann-benchmarks.com) can be **cited** as supporting evidence but do NOT satisfy the `[measured]` bar — only first-party benchmarks count as measured

## Phase 2.75: Synthesis & Frameworks

Produces the decision framework (Q4) and deployment trade-off profiles (Q5). These are distinct from the cross-cohort synthesis — they are prescriptive frameworks, not implementation comparisons.

### Outputs

| Output | Template | Location | Quality bar |
|--------|----------|----------|-------------|
| Decision framework (Q4) | (free-form) | `findings/decision-framework.md` | `[measured]` or `[observed]` for key claims |
| Deployment trade-offs: in-process ↔ embedded ↔ external (Q5.a) | (free-form) | `findings/deployment-tradeoffs.md` | `[cited]`+ required |
| Deployment trade-offs: pure vector ↔ hybrid (Q5.b) | (free-form) | `findings/hybrid-tradeoffs.md` | `[cited]`+ required |

### Protocol

These deliverables synthesize across all prior phases:
- Q4 decision framework draws from: R&E taxonomy (Q1), benchmark data (Q3), crate evaluations (Q6.b)
- Q5.a deployment trade-offs draw from: crate evaluations + architectural observations from codebase analysis
- Q5.b hybrid trade-offs draw from: scope boundary analysis + cited evidence from hybrid DB research (research-anj.1 when available)

**Scope guard on Q5.b**: Document the architectural trade-off profile only. Specific hybrid database evaluation is deferred to research-anj.1.

## Phase 4: Codebase Analysis Protocol

### What a codebase analysis run covers

Each run focuses on a specific aspect across multiple codebases:
- **Run 1**: Index construction — how graphs/trees are built, data structures, build-time complexity
- **Run 2**: Query hot paths — distance computation, candidate selection, SIMD usage
- **Run 3**: Persistence — memory-mapping, serialization, index format
- **Run 4**: Safety & quality — unsafe usage, error handling, API design, dependency health

### Iteration & convergence

Codebase analysis iterates when:
- A run reveals an aspect worth deeper investigation (e.g., Run 2 finds unexpected SIMD patterns → Run 2b digs deeper)
- New candidates emerge from targeted research that weren't initially analyzed

Analysis is "done" when:
- All top candidates (from targeted research ranking) have been analyzed
- Key architectural patterns are documented with `[observed: file:line]` grounding
- Synthesis can answer: "which components are best-of-breed and why?"

## Question → Phase Mapping

| Question | Primary Phase | Supporting Phases |
|----------|--------------|-------------------|
| Q1 (algorithm taxonomy) | Phase 1 (R&E) | — |
| Q1.a-e (sub-questions) | Phase 1 (R&E) | — |
| Q2 (Rust crate matrix) | Phase 1 (R&E) + Phase 2 (Targeted) | — |
| Q3 (benchmarks) | Phase 2.5 (Benchmarking) | Phase 2 (candidate selection) |
| Q4 (decision framework) | Phase 2.75 (Synthesis) | Phase 1 + 2 + 2.5 |
| Q5.a (deployment trade-offs) | Phase 2.75 (Synthesis) | Phase 2 + 4 |
| Q5.b (hybrid trade-offs) | Phase 2.75 (Synthesis) | External (research-anj.1) |
| Q6.a (implementation survey) | Phase 1 (R&E) + Phase 2 (Targeted) | — |
| Q6.b (implementation comparison) | Phase 2 (Targeted) + Phase 4 (Codebase) | Phase 2.5 (data) |

## Phase Transitions

| Transition | Entry Criteria | Exit Criteria |
|-----------|----------------|---------------|
| Phase 0 → Phase 1 | All Phase 0 specs approved (human gates passed) | — |
| Phase 1 → Phase 2 | R&E convergence criteria met | Stable taxonomy, candidate list |
| Phase 2 → Phase 2.5 | At least one cohort synthesis complete | Top candidates identified |
| Phase 2.5 → Phase 2.75 | Benchmark data available for ≥3 candidates | Measured comparison data |
| Phase 2 → Phase 3 | All cohorts complete + benchmarks available | — |
| Phase 3 → Phase 4 | Codebase analysis plan approved (human gate) | — |
| Phase 4 → Done | All top candidates analyzed, synthesis complete | Findings docs produced |

## Review Gates

### Self-review before human gate

Before presenting any SPEC or Plan document for human review:
1. Dispatch `superpowers:code-reviewer` to review the document
2. Fix all Critical and Important issues
3. Then trigger the Human Review bead

### Human review triggers

The following documents require human approval before execution proceeds:
- Research questions (`research-cfj.2.13.5`)
- Scope definition (`research-cfj.2.13.6`)
- This methodology (`research-cfj.2.13.7`)
- Resource plan (`research-cfj.2.13.8`)
- Cohort definitions (`research-cfj.2.15.3`)
- Codebase analysis plan (`research-cfj.2.6.3`)

## Grounding Standards

Per `docs/src/methodology/grounding.md`:

| Phase | Minimum grounding |
|-------|------------------|
| R&E run reports | `[inferred]` OK — working notes |
| Crate evaluations | `[cited]` or `[observed]` required |
| Cohort synthesis | `[cited]` or `[observed]` required |
| Codebase analysis | `[observed: file:line]` required |
| Decision frameworks | `[measured]` or `[observed]` for key claims |
| Findings/recommendations | No `[inferred]` as primary evidence |

## Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| R&E runs don't converge (always finding new crates) | Cap at 4 runs; after that, accept the taxonomy and note known gaps |
| Cohort evaluation criteria too narrow/broad | Human review gate on cohort definitions catches this |
| Template rigidity slows execution | Templates are guides, not straitjackets. Deviate with a note explaining why. |

## Open Questions

1. Should R&E runs be time-boxed (e.g., max 1 session per run) or scope-boxed (e.g., must cover all dimensions)?
2. How many crates per cohort before a cohort is "too large" and should be split?

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-24 | Initial draft | aRustyDev + Claude |
| 2026-04-24 | Added Phase 2.5 (Benchmarking), Phase 2.75 (Synthesis & Frameworks), Phase 4 protocol, Q→Phase mapping, phase transitions, success criteria per code review | Claude |
| 2026-04-25 | Expanded Cohort iteration subsection with two-pass pattern, convergence criteria, shared harness prerequisite | aRustyDev + Claude |
