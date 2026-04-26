# SPEC: Targeted Research Cohort Definitions — Vector DS&A

> **Bead**: `research-cfj.2.15.1`
> **Status**: draft
> **Author**: aRustyDev + Claude
> **Reviewer**: aRustyDev
> **Date**: 2026-04-25
> **Approved**: —

## Purpose

Define evaluation cohorts for the Targeted Research phase of Vector DS&A. Cohorts group crates by algorithm family so that within-family comparisons are meaningful (same metrics, same benchmarks, same baseline expectations). Each cohort gets its own evaluation plan because different algorithm families have different primary trade-offs.

This spec is the bridge between R&E (which identified and classified candidates) and execution (which evaluates them).

## Scope

### In Scope

- Cohort membership for all Tier 1 and Tier 2 crates from Run 2 scoping
- Per-cohort evaluation dimensions (which of the 10 apply and why)
- Per-cohort evaluation plan (what to examine, what to benchmark)
- Per-cohort acceptance criteria
- Cross-cutting utility evaluation (simsimd)
- Disposition of pre-existing per-crate beads (research-cfj.2.8 through research-cfj.2.12)

### Out of Scope

- Tier 3 crate evaluation (referenced for context only, not deep-dived)
- Actual evaluation execution (Phase 3)
- Cross-cohort synthesis (Phase 4, research-cfj.2.15.2)
- Benchmark harness implementation
- Project-specific recommendations

### Assumptions

- The taxonomy from Run 2 is stable — no further algorithm family reclassifications expected
- The 10 evaluation dimensions from scope.md are complete — no new dimensions anticipated
- The metrics from Run 2 metrics.md are operationalized and ready for use
- Tier 1 crates receive full evaluation; Tier 2 crates receive focused evaluation on their unique contribution
- Crates removed from consideration in R&E scoping stay removed
- A shared benchmark harness will be built before per-cohort execution begins (see Execution Prerequisites)

## Methodology

This section defines HOW cohorts are evaluated. It complements `specs/methodology.md` (which defines the overall research methodology) with cohort-specific execution patterns.

### Search-Term Matrices (Rule #7)

**All information-gathering searches during Targeted Research MUST be preceded by a search-term matrix.** This applies to:

- Crate documentation searches (API docs, README, examples)
- Algorithm reference searches (papers, blog posts, comparisons)
- Benchmark reference data searches (ann-benchmarks results, third-party evaluations)
- Bug/issue searches (known limitations, platform issues, correctness bugs)

Use the `search-term-matrices` skill to build the matrix. One matrix per cohort per pass is the minimum granularity — a cohort doing Pass 1 source code examination builds one matrix covering all crates in the cohort. A separate matrix is built for Pass 2 follow-up questions.

### Iteration Pattern

Each cohort follows a two-pass evaluation with an optional third pass:

**Pass 1 — Initial Evaluation**:
1. Build search-term matrix for the cohort
2. Source code examination (analyze-codebase formula) for all crates
3. Initial benchmarks (execute-benchmark formula) — first parameter sweep, establish baseline
4. Draft per-crate evaluations using `crate-evaluation.md` template
5. Scope review (Rule #10): do findings reveal evaluation dimensions not in our framework?
6. Provenance check: did any findings change prior understanding in knowledge artifacts?

**Pass 2 — Refinement**:
1. Build follow-up search-term matrix based on Pass 1 gaps
2. Follow-up source code examination on specific questions from Pass 1
3. Refined benchmarks with tuned parameters — sweep the Pareto frontier
4. Update per-crate evaluations with new data
5. Scope review: any new dimensions discovered in Pass 2?
6. Convergence assessment (see below)

**Pass 3 — Resolution (only if needed)**:
Triggered when Pass 2 convergence assessment fails. Focused on specific unresolved questions, not a broad re-evaluation. Must have a written justification for why a third pass is needed.

### Convergence Criteria

A cohort evaluation is "converged" when ALL of the following are true:

- [ ] All acceptance criteria for the cohort (defined per-cohort above) are met
- [ ] No unanswered questions from source code examination remain
- [ ] Benchmark results are stable — re-running with the same parameters produces the same tier classification
- [ ] Per-crate evaluations are complete for all Tier 1 crates and all Tier 2 unique-contribution aspects
- [ ] No scope expansions were introduced in the current pass without resolution
- [ ] All knowledge artifact changes during this cohort's evaluation have corresponding epistemic timeline entries (Rule #13)

If convergence is not met after Pass 2, document what's missing and create Pass 3 with specific targets (Rule #9 prior-run handoff applies).

### Scope Review During Evaluation (Rule #10)

At the end of each pass, review the cohort's evaluation dimensions against findings:

- **Did we find a capability not in our 10-dimension framework?** → Draft an amendment to scope.md. Expansions require human review.
- **Did a crate's actual behavior contradict our taxonomy classification?** → This is an epistemic event. Create a timeline entry, update taxonomy.yaml with provenance. If reclassification changes cohort membership, flag for human review.
- **Did benchmark results reveal a metric gap?** (e.g., a dimension of performance not captured by metrics.md) → Draft amendment to metrics.md. Refinements can be adopted directly.

### Prior-Run Handoff (Rule #9)

After completing Pass N of a cohort evaluation:
1. Identify specific gaps, questions, and refinement targets
2. Update Pass N+1's bead descriptions with these specific targets
3. Generic descriptions like "refine evaluation" become targeted like "Pass 1 found hnsw_rs filtered search degrades >50% at 1% selectivity — investigate whether this is fundamental or parameter-dependent"

### Provenance Tracking (Rules #11-13)

During cohort evaluation, provenance tracking triggers when:
- Source code examination reveals a crate's actual algorithm differs from taxonomy classification
- Benchmark results contradict published performance claims
- A crate's filtered ANN implementation uses an approach not documented in the taxonomy's cross-cutting concerns
- Any knowledge artifact (taxonomy.yaml, glossary.yaml, references.yaml) is updated based on evaluation findings

Use the `provenance-tracking` skill for all such changes. Create epistemic timeline entries during each cohort's synthesis step.

### Execution Prerequisites

Before any cohort begins Pass 1:
1. **Shared benchmark harness** must be built and validated (see research-ola)
2. **Cohort definitions** must be human-approved (research-cfj.2.15.3)
3. **Pre-existing beads** reassigned to their cohort sub-epics

## Specification

### Cohort Structure Overview

| Cohort | Algorithm Family | Tier 1 Crates | Tier 2 Crates | Primary Trade-off Axis |
|--------|-----------------|---------------|---------------|----------------------|
| A | NSW/HNSW (graph) | hnsw_rs, usearch | instant-distance | Pure Rust vs FFI; filtered ANN implementations |
| B | MRNG-derived / DiskANN (graph) | diskann | diskann-rs, rust-diskann | SSD-resident performance; memory per vector |
| C | Tree-based (RPT, KD, VP) | arroy, kiddo | kd-tree, vpsearch | Dimensionality degradation; domain specialization |
| D | Quantization + Partition (PQ, RaBitQ, TurboQuant, IVF) | faiss | rabitq-rs, turbo-quant, turbovec | Compression ratio vs recall; training cost |
| X | Distance computation (utility) | — | simsimd | Throughput across precisions; composability |

Cohort X is not a traditional evaluation cohort — simsimd is a utility library, not a search index. It is evaluated separately on distance computation speed and integration potential with index crates.

---

### Cohort A: Graph-based — NSW/HNSW

**Algorithm family**: Navigable Small World / Hierarchical Navigable Small World

**Why this cohort**: HNSW is the dominant ANN algorithm in both literature and production. The Rust ecosystem has multiple implementations spanning pure Rust and FFI, with varying feature sets. Direct comparison within this family answers: "which HNSW implementation should a Rust project use?"

#### Crates

| Crate | Tier | Lang | Downloads | Key Differentiator |
|-------|------|------|-----------|-------------------|
| hnsw_rs | 1 | Pure Rust | 332K | Most popular pure-Rust HNSW. Filterable trait for in-filter ANN. |
| usearch | 1 | FFI (C++) | 221K | Feature-complete: filtered search, multi-precision (f32/f16/i8), 192 versions. |
| instant-distance | 2 | Pure Rust | 126K | Clean API. Possibly "complete/stable" rather than abandoned. Focused evaluation. |

**Tier 3 references** (not deep-dived, but examined for architectural patterns):
hnswlib-rs, swarc, granne, hnsw, small-world-rs, ruvector (GNN-based, unclassified — new ecosystem, unclear algorithm)

#### Evaluation Dimensions

All 10 dimensions apply:

| Dimension | Relevance | Notes |
|-----------|-----------|-------|
| Performance | Primary | Recall@10 vs QPS Pareto frontier is the defining comparison |
| Correctness | Primary | Verify recall accuracy across parameter sweep |
| Scalability | Primary | Test 128d through 1536d; Hub Highway Hypothesis validation |
| Code quality | Primary | Unsafe usage in hnsw_rs vs usearch FFI boundary |
| Maturity | Primary | Release cadence, issue response, production usage |
| Composability | Secondary | Can PQ/SQ be plugged in? Does usearch's C++ core limit composability? |
| Filtered search | Primary | hnsw_rs (Filterable trait) vs usearch (predicate fn) — API and performance |
| Incremental updates | Secondary | Insert/delete without rebuild |
| Persistence | Secondary | instant-distance (serialization), usearch (save/load) |
| Platform portability | Secondary | All three should work on ARM64 and x86_64 |

#### Evaluation Plan

**Source code examination** (analyze-codebase formula):
1. Graph construction algorithm: M, ef_construction implementation, neighbor selection heuristic
2. Search algorithm: ef_search, visited-set management, SIMD usage
3. Filtered search implementation: how predicates integrate with graph traversal
4. Memory layout: adjacency list representation, cache-friendliness
5. Thread safety: concurrent search, concurrent insert (if supported)
6. Unsafe code audit: count, necessity, soundness

**Benchmarks** (execute-benchmark formula):
1. Standard benchmark: recall@10 vs QPS Pareto frontier at 128d/384d/768d/1536d on 1M vectors
2. Filtered benchmark: recall@10 and QPS with 10% selectivity filter (hnsw_rs, usearch only)
3. Build time comparison: single-threaded, 1M vectors, all four dimensionalities
4. Memory per vector: RSS measurement protocol from metrics.md
5. Hub Highway Hypothesis test: compare HNSW (multi-layer) vs single-layer search at 128d vs 1536d (if hnsw_rs allows single-layer configuration)

#### Acceptance Criteria

- [ ] All three crates benchmarked at all four dimensionalities with recall@10 vs QPS curves
- [ ] Filtered ANN compared: hnsw_rs vs usearch, API quality and performance
- [ ] Pure Rust vs FFI trade-off characterized: build complexity, performance gap, composability
- [ ] Code quality assessment for all three (unsafe audit, API ergonomics)
- [ ] instant-distance maintenance status resolved (complete/stable vs abandoned)
- [ ] Cohort synthesis document produced with within-family recommendation framework

#### Pre-existing Bead

**research-cfj.2.8** (Research: usearch) — reassign as child of the Cohort A sub-epic when created.

---

### Cohort B: Graph-based — MRNG-derived / DiskANN

**Algorithm family**: Vamana (DiskANN) — MRNG-derived graph with SSD-resident index

**Why this cohort**: DiskANN/Vamana fills the "billion-scale on single node" gap that in-memory HNSW cannot. The existence of pure Rust implementations is significant — this was a confirmed absence until recently. Small cohort (2 crates doing the same thing) makes direct implementation comparison the focus.

#### Crates

| Crate | Tier | Lang | Downloads | Key Differentiator |
|-------|------|------|-----------|-------------------|
| diskann | 1 | Pure Rust | 7.8K | INFINI Labs. Forked from MS Rust port. Active (v0.50.0, 9 versions). |
| diskann-rs | 2 | Pure Rust | — | Alternative impl. Updated Feb 2026. Need to assess vs diskann. |
| rust-diskann | 2 | Pure Rust | 750 | By jianshu93. 3 versions on crates.io. Vamana graph + mmap. Generic distance trait. |

#### Evaluation Dimensions

| Dimension | Relevance | Notes |
|-----------|-----------|-------|
| Performance | Primary | Latency (p50/p99) matters more than QPS for disk-resident — 2-10ms expected |
| Correctness | Primary | Recall accuracy with PQ codes in RAM, full vectors on SSD |
| Scalability | Primary | The raison d'etre — test at 1M and 10M; cite billion-scale from literature |
| Code quality | Primary | Both are relatively new — quality signals matter for adoption |
| Maturity | Primary | diskann has more versions; diskann-rs is newer |
| Composability | Secondary | Can PQ codebook be swapped? Can graph be used without PQ? |
| Filtered search | Primary | DiskANN paper supports it (Filtered-DiskANN); verify Rust impl |
| Incremental updates | Primary | Disk-resident indexes need efficient updates |
| Persistence | Primary | SSD-resident is the defining feature — how is the index laid out on disk? |
| Platform portability | Secondary | Check SIMD requirements and OS support |

#### Evaluation Plan

**Source code examination**:
1. Vamana graph construction: alpha-pruning, medoid entry point selection
2. SSD I/O pattern: beam search over disk-resident graph, PQ code in-memory layout
3. Filtered search support: is it implemented? What API?
4. PQ integration: how are codes stored, how is distance computed (ADC?)
5. Build pipeline: kNN graph initialization, pruning pass, PQ training

**Benchmarks**:
1. Standard benchmark: recall@10 vs QPS at 128d/384d/768d/1536d on 1M vectors
2. Latency distribution: p50/p99 (more important than QPS for disk-resident)
3. Memory per vector: should be dramatically lower than Cohort A (PQ codes only in RAM)
4. Build time: expect longer than HNSW (kNN graph + pruning + PQ training)
5. Index size on disk: bytes per vector
6. Scale test: 10M vectors if feasible (primary differentiator)

#### Acceptance Criteria

- [ ] All three implementations benchmarked: diskann, diskann-rs, rust-diskann
- [ ] Memory per vector compared against Cohort A (HNSW) — should be 10-100x lower
- [ ] SSD I/O pattern characterized: sequential vs random reads, latency under load
- [ ] Filtered search status verified for both implementations
- [ ] Build pipeline documented: time, intermediate artifacts, disk space during build
- [ ] Cohort synthesis with recommendation framework for when DiskANN > HNSW

---

### Cohort C: Tree-based

**Algorithm family**: Random Projection Trees (RPT), KD-tree, VP-tree

**Why this cohort**: Tree-based methods have the strongest Rust ecosystem presence by download count (kiddo: 3.4M). They serve different niches: arroy targets general ANN with persistence, kiddo targets low-d spatial search, vpsearch targets custom metrics. The key research question is dimensionality degradation — at what d do tree-based methods become uncompetitive with graph-based?

#### Crates

| Crate | Tier | Lang | Downloads | Key Differentiator |
|-------|------|------|-----------|-------------------|
| arroy | 1 | Pure Rust | ~30K/mo | RPT. Meilisearch production. LMDB persistence. Filtered search. Evolving toward "Filtered Disk ANN." |
| kiddo | 1 | Pure Rust | 3.4M | KD-tree. Highest downloads. Geo/astro focus. SIMD. no_std. mmap. |
| kd-tree | 2 | Pure Rust | 533K | KD-tree. Simpler alternative to kiddo. |
| vpsearch | 2 | Pure Rust | 62K | VP-tree. Only pure-Rust VP-tree. Custom metric support. |

#### Evaluation Dimensions

| Dimension | Relevance | Notes |
|-----------|-----------|-------|
| Performance | Primary | Especially at varying dimensionalities — degradation curve is the story |
| Correctness | Primary | KD-tree and VP-tree can do exact NN — verify, then compare ANN mode |
| Scalability | Primary | Dimensionality sensitivity is THE key dimension for this cohort |
| Code quality | Primary | All pure Rust — API ergonomics comparison |
| Maturity | Primary | arroy (Meilisearch backing), kiddo (3.4M downloads), kd-tree, vpsearch |
| Composability | Secondary | Can these trees be used as components (e.g., SPTAG-style tree+graph)? |
| Filtered search | **arroy only** | RoaringBitmap integration; others don't support filtered ANN |
| Incremental updates | Secondary | Trees generally support insertion; verify delete |
| Persistence | Primary | arroy (LMDB), kiddo (mmap) — important differentiators |
| Platform portability | Secondary | kiddo uses SIMD — check ARM64 |

#### Evaluation Plan

**Source code examination**:
1. Tree construction: splitting strategy (RPT random hyperplane vs KD-tree axis-aligned vs VP-tree distance-based)
2. Search algorithm: tree traversal + backtracking + candidate merging
3. arroy: LMDB integration, RoaringBitmap filtering, "Filtered Disk ANN" evolution status
4. kiddo: SIMD optimizations, no_std compatibility, mmap serialization
5. vpsearch: custom metric trait design, generic distance support

**Benchmarks**:
1. **Dimensionality degradation curve**: benchmark at 8d/16d/32d/64d/128d/384d/768d to find the crossover point where tree-based becomes uncompetitive
2. Standard benchmark at 128d (where tree-based should still be competitive)
3. kiddo: benchmark at <20d where KD-tree dominates (its primary use case)
4. arroy: filtered ANN benchmark (10% selectivity) vs Cohort A filtered ANN
5. Build time: trees typically build faster than graphs — quantify the gap
6. Memory per vector at each dimensionality

**Note**: For kiddo and kd-tree, the primary use case is low-d spatial search (<20d), not general ANN at 384d+. The evaluation should respect this — excellent performance at 8d is a valid strength, not a failure to compete at 768d.

#### Acceptance Criteria

- [ ] Dimensionality degradation curve established for all four crates
- [ ] KD-tree crates (kiddo, kd-tree) benchmarked in their sweet spot (<20d) AND at ANN-relevant dimensions
- [ ] arroy filtered ANN performance compared against Cohort A (hnsw_rs, usearch)
- [ ] arroy "Filtered Disk ANN" evolution status documented
- [ ] Persistence implementations compared: arroy LMDB vs kiddo mmap
- [ ] Cohort synthesis with guidance on when tree-based > graph-based (dimensionality thresholds, use case alignment)

#### Pre-existing Bead

**research-cfj.2.9** (Research: Arroy) — reassign as child of the Cohort C sub-epic when created.

#### Arroy Cross-Reference: Potential DiskANN Reclassification

The taxonomy notes arroy is "evolving toward Filtered Disk ANN." arroy is evaluated in Cohort C because it IS an RPT implementation today. However, source code examination may reveal DiskANN-like characteristics. The following protocol handles this:

**Epistemic timeline trigger**: Create an entry if source code examination reveals ANY of the following:
- arroy's index structure has been replaced or augmented with a Vamana-style graph
- arroy uses alpha-pruning or MRNG-derived neighbor selection (not RPT random hyperplane splits)
- arroy's persistence model has shifted from LMDB-backed RPT to SSD-resident graph (DiskANN pattern)
- arroy's search algorithm uses beam search over a graph rather than tree traversal + backtracking

**If triggered**: Document the finding in the Cohort C synthesis as a cross-reference to Cohort B. Include: what specifically is DiskANN-like, what remains RPT-like, and how far the evolution has progressed. Tag the timeline entry with both cohort IDs.

**Follow-on evaluation**: If arroy is found to be substantially DiskANN-like (≥2 of the triggers above), create a follow-on bead to evaluate it through the Cohort B lens (Vamana graph construction quality, SSD I/O pattern, memory per vector). This does NOT move arroy out of Cohort C — it gets dual evaluation. The Cohort C evaluation captures its tree-based heritage and filtered search; the Cohort B follow-on captures its graph-based evolution.

**If NOT triggered**: arroy remains purely in Cohort C. Note in the synthesis that the "Filtered Disk ANN" evolution was assessed and found to be aspirational rather than implemented (as of the evaluation date).

---

### Cohort D: Quantization + Partition

**Algorithm family**: Product Quantization, RaBitQ, TurboQuant, IVF, Scalar Quantization

**Why this cohort**: Quantization is a cross-cutting technology used as both standalone search and as a component in composite systems (IVF+PQ, HNSW+PQ, DiskANN+PQ). The Rust landscape spans FFI (faiss) and pure Rust (rabitq-rs, turbovec). The key research question is whether modern quantization methods (RaBitQ, TurboQuant) obsolete classical PQ for typical use cases.

#### Crates

| Crate | Tier | Lang | Downloads | Key Differentiator |
|-------|------|------|-----------|-------------------|
| faiss | 1 | FFI (C++) | 91K | Most comprehensive: PQ, OPQ, SQ, IVF, IVF+PQ, IVF+HNSW. Industry standard. |
| rabitq-rs | 2 | Pure Rust | 36 | IVF+RaBitQ. Modern quantization with theoretical guarantees. x86_64 only. |
| turbo-quant | 2 | Pure Rust | — | Algorithm library: TurboQuant/PolarQuant/QJL (ICLR 2026). 3-8 bit compression, zero calibration. Composability target — can back other indexes. |
| turbovec | 2 | Pure Rust | low | Standalone vector index built on TurboQuant. Claims faster than FAISS IndexPQFastScan. Very new. |

#### Evaluation Dimensions

| Dimension | Relevance | Notes |
|-----------|-----------|-------|
| Performance | Primary | Compression ratio vs recall is THE metric for quantization |
| Correctness | Primary | Quantization introduces controlled error — verify bounds match theory |
| Scalability | Primary | Quantization's value grows with scale and dimensionality |
| Code quality | Primary | faiss FFI boundary quality; rabitq-rs/turbovec pure Rust quality |
| Maturity | Primary | faiss is mature; rabitq-rs and turbovec are very new |
| Composability | **Critical** | Can these be used as backends for Cohort A/B graph indexes? |
| Filtered search | Secondary | Depends on implementation; faiss IVF has nprobe-based filtering |
| Incremental updates | Secondary | PQ requires retraining on distribution shift; RaBitQ/TurboQuant don't |
| Persistence | Secondary | How are codebooks and codes serialized? |
| Platform portability | **Primary** | rabitq-rs is x86_64-only (ARM64 has critical bugs) |

#### Evaluation Plan

**Source code examination**:
1. faiss: Rust FFI boundary quality, which FAISS features are exposed, build complexity
2. rabitq-rs: RaBitQ implementation fidelity (random rotation, binary encoding, correction factors), IVF integration
3. turbovec: TurboQuant/PolarQuant/QJL implementation, compression modes (2-4 bit)
4. Training requirements: faiss PQ needs training data; RaBitQ needs only a random rotation matrix; TurboQuant needs nothing

**Benchmarks**:
1. **Compression ratio vs recall curve**: at equivalent compression (e.g., 32x), compare recall@10 across PQ, RaBitQ, binary quantization
2. Training time: PQ codebook learning vs RaBitQ rotation matrix generation vs TurboQuant (zero)
3. QPS at fixed recall: how fast is distance computation with each encoding?
4. Platform benchmark: test rabitq-rs on x86_64; document ARM64 failure
5. Composability test: can rabitq-rs codes be used with an HNSW graph? Can turbovec be a drop-in for faiss PQ?
6. faiss feature coverage: which of FAISS's ~50 index types are actually exposed in Rust?

#### Acceptance Criteria

- [ ] Compression ratio vs recall compared: PQ vs RaBitQ vs TurboQuant vs naive binary at 128d and 768d
- [ ] Training cost quantified: time and data requirements for each method
- [ ] faiss Rust FFI coverage documented: which index types are accessible?
- [ ] rabitq-rs ARM64 status documented with specific failure mode
- [ ] Composability assessed: can these be used as components in graph-based indexes?
- [ ] Cohort synthesis with guidance on PQ vs RaBitQ vs TurboQuant selection criteria

---

### Cohort X: Distance Computation Utility — simsimd

**Why separate**: simsimd is not a search index — it computes distance/similarity between vectors. It's complementary to all index crates. Evaluating it alongside indexes would produce misleading comparisons.

#### Crate

| Crate | Tier | Lang | Downloads | Key Differentiator |
|-------|------|------|-----------|-------------------|
| simsimd | 2 | FFI (C) | 593K | Fastest SIMD distance library. f32/f16/i8/binary. Multi-platform. |

#### Evaluation Dimensions

| Dimension | Relevance | Notes |
|-----------|-----------|-------|
| Performance | Primary | Distance computation throughput is the only metric |
| Correctness | Secondary | Verify numerical accuracy of SIMD vs scalar at each precision |
| Scalability | N/A | Utility library — O(d) per pair, no index structure |
| Code quality | Secondary | FFI boundary quality, build complexity |
| Maturity | Secondary | 593K downloads, very active — likely mature |
| Composability | **Primary** | Can it replace built-in distance functions in index crates? |
| Filtered search | N/A | Not a search index |
| Incremental updates | N/A | Not a search index |
| Persistence | N/A | Not a search index |
| Platform portability | Primary | x86_64 (SSE/AVX/AVX-512), ARM64 (NEON), WASM — key value prop |

#### Evaluation Plan

1. **Standalone speed**: distance computation throughput at f32/f16/i8/binary across 128d/384d/768d/1536d
2. **Comparison**: vs Rust std, vs ndarray, vs manual SIMD in hnsw_rs/kiddo
3. **Integration potential**: can simsimd be swapped in as the distance backend for hnsw_rs, usearch, arroy?
4. **Platform coverage**: x86_64 (SSE/AVX/AVX-512), ARM64 (NEON), WASM

#### Acceptance Criteria

- [ ] Distance computation throughput benchmarked across precisions and dimensionalities
- [ ] Compared against at least 2 alternatives (Rust std, one crate's built-in SIMD)
- [ ] Integration feasibility assessed for at least 2 index crates
- [ ] Platform support matrix documented

---

## Execution Order

```
Phase 1 (this spec) → Phase 2 (Human Review: research-cfj.2.15.3)
                        ↓
Phase 3: Per-cohort execution (parallel where possible)
  Cohort A ─────────────────────────────┐
  Cohort B ─────────────────────────────┤
  Cohort C ─────────────────────────────┤→ Phase 4: Cross-cohort synthesis
  Cohort D ─────────────────────────────┤   (research-cfj.2.15.2)
  Cohort X (can run in parallel) ───────┘
```

**Dependencies within Phase 3**:
- Cohort X (simsimd) should run first or in parallel — its integration findings inform Cohorts A, C, D
- Cohorts A and B are independent
- Cohort D's composability assessment benefits from Cohort A/B source code examination (but is not blocked by it)

**Formula usage**:
- Per-crate evaluation: `research-targeted` formula → creates container, then child tasks per crate
- Source code examination: `analyze-codebase` formula within each cohort
- Benchmarks: `execute-benchmark` formula within each cohort

## Pre-existing Bead Disposition

| Bead | Original Target | Disposition |
|------|----------------|-------------|
| research-cfj.2.8 | usearch | Reassign to Cohort A sub-epic |
| research-cfj.2.9 | Arroy | Reassign to Cohort C sub-epic |
| research-cfj.2.10 | Similari | **Closed** — out of scope (visual object tracking, not ANN) |
| research-cfj.2.11 | Voy | **Closed** — out of scope (WASM JS library, not Rust crate) |
| research-cfj.2.12 | Fast Vector Similarity | **Closed** — not on crates.io |

## Success Criteria

- [ ] All Tier 1 and Tier 2 crates from Run 2 scoping are assigned to exactly one cohort
- [ ] Each cohort has: membership list, applicable evaluation dimensions, evaluation plan, acceptance criteria
- [ ] Evaluation dimensions are tailored per cohort (not blind application of all 10)
- [ ] Pre-existing beads are dispositioned (reassigned or closed with reason)
- [ ] Execution order and formula usage are specified
- [ ] Human review approved (research-cfj.2.15.3)

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Cohort D crates too immature for meaningful benchmarking (rabitq-rs: 36 downloads, turbovec: very new) | Medium | Medium | Focus on source code quality and theoretical comparison; benchmark what works; document what doesn't |
| diskann-rs or rust-diskann may not exist on crates.io or may be unmaintained | Medium | Low | Verify existence before committing to Cohort B evaluation; fallback to diskann-only if needed |
| faiss FFI build complexity prevents benchmarking on target hardware | Low | Medium | Document build steps; use pre-built binaries if available; fallback to API/code analysis only |
| Cohort C dimensionality degradation curve is well-known and adds limited novelty | Low | Low | Frame as verification of known behavior with specific Rust implementations; the value is in the specific crossover points |
| simsimd integration with index crates may require non-trivial modifications | Medium | Low | Assess feasibility only; don't implement the integration |

## Open Questions

*(none remaining)*

### Resolved

- ~~**rust-diskann**~~: Exists on crates.io (750 downloads, 3 versions, by jianshu93). Added to Cohort B as Tier 2. *(resolved 2026-04-25)*
- ~~**turbo-quant vs turbovec**~~: Both included in Cohort D. turbo-quant is the algorithm library (composability target for backing other indexes); turbovec is the standalone vector index built on it. Both warrant evaluation. *(resolved 2026-04-25)*
- ~~**Benchmark harness**~~: Shared harness approved. Creates cross-cohort comparability for Phase 4 synthesis. Tracked as research-ola (epic with research → spec → plan → implement → review lifecycle). Must complete before any cohort begins Pass 1. *(resolved 2026-04-25)*
- ~~**arroy "Filtered Disk ANN" evolution**~~: arroy stays in Cohort C (it IS an RPT today). Cross-reference protocol added to Cohort C section with 4 specific epistemic triggers. If ≥2 triggers fire, a follow-on Cohort B evaluation is created (dual evaluation, not reclassification). *(resolved 2026-04-25)*

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-25 | Initial draft — 4 cohorts + 1 utility, all tiers assigned | aRustyDev + Claude |
| 2026-04-25 | Self-review fixes: resolved rust-diskann (added to Cohort B), turbo-quant (added to Cohort D), added Cohort X dimension table, expanded In Scope to cover all 5 pre-existing beads, added ruvector to Tier 3 refs | Claude |
| 2026-04-25 | Added Methodology section (search-term matrices, iteration pattern, convergence criteria, scope review, provenance tracking, execution prerequisites). Added arroy cross-reference protocol to Cohort C. Resolved all open questions (benchmark harness → shared, arroy → stays in C with escalation path). | aRustyDev + Claude |
