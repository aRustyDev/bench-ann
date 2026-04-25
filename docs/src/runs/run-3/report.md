# Run Report: Vector DS&A — Run 3

> **Bead**: `research-cfj.2.14.3`
> **Date**: 2026-04-25
> **Run goal**: scope-expansion coverage (Q1.f, Q1.g, Q1.h)

## Summary

Run 3 covered the three new research questions added during Run 2's scope review. All questions are now answered with cited evidence. No further scope expansions needed. Key findings: (1) filtered ANN has 6 approaches, not 3, with the FANNS survey's pruning framework superseding the pre/post/in-filter trichotomy; (2) RaBitQ dominates modern quantization at 32x compression with theoretical guarantees PQ lacks, while TurboQuant's claims are contested; (3) HNSW hierarchy is unnecessary above ~32D, and RaBitQ exhibits an inverse curse of dimensionality (quality improves with dimension).

## What Changed from Run 2

### Q1.f: Filtered ANN (new question, fully answered)

- The simple pre/post/in-filter taxonomy is superseded by the FANNS survey's 4-strategy pruning framework: VSP, VJP, SSP, SJP (arXiv:2505.06501)
- **6 filtering approaches** documented with selectivity thresholds and failure modes
- **5 Rust crates** have filtering support (up from 3 in Run 2): hnsw_rs (in-filter), usearch (in-filter), arroy (bitmap-intersection), diskann-rs (filter-aware construction), diskann/infinilabs (filter-aware construction)
- The diskann-rs crate implements Filtered-DiskANN with `Filter::label_eq`/`label_range`/`and` combinators — this was not known in Run 2
- **Distribution factor** matters beyond selectivity: spatial clustering of filtered points relative to query affects performance independently of filter selectivity ratio
- **Window filters** (arXiv:2402.00943) achieve 75x speedup for numeric range filters in the "dead zone" where both pre-filter and post-filter degrade

### Q1.g: Modern Quantization (new question, fully answered)

- **7-method comparison** produced across 9 dimensions (mechanism, compression, training, guarantees, recall, distance estimation, Rust availability, production adoption)
- **RaBitQ dominates at 32x compression** with O(1/sqrt(D)) error bound matching information-theoretic lower bounds. PQ has no guarantees and fails catastrophically on some real datasets (MSong: <60% recall)
- **TurboQuant's claims contested** by two independent teams: RaBitQ team (arXiv:2604.19528) found it worse in many configurations with reproducibility issues; EDEN team (arXiv:2604.18555) shows it's a suboptimal special case of their 2022 work
- **GPU IVF-RaBitQ** (arXiv:2602.23999) achieves 2.2x higher QPS than CAGRA at recall ~0.95, first quantization method to consistently beat graph-based GPU methods
- **Data-oblivious property** of RaBitQ/TurboQuant matters for production: zero training, no distribution drift, streaming-friendly
- **Lineage diagram** produced showing PQ → OPQ, RaBitQ → ExtRaBitQ → GPU IVF-RaBitQ, DRIVE → EDEN, PolarQuant + QJL → TurboQuant
- **RaBitQ original paper** (arXiv:2405.12497) is now the canonical reference (Run 1 only had the extension)

### Q1.h: Dimensionality-Dependent Behavior (new question, fully answered)

- **5 qualitative crossover points** identified with specific dimension thresholds:
  1. HNSW hierarchy becomes irrelevant at ~32D (Hub Highway Hypothesis)
  2. Tree-based methods become uncompetitive at ~20-30D
  3. RaBitQ surpasses PQ reliability at ~128D+
  4. IVF cluster quality degrades at ~256D+
  5. LSH theoretical guarantees never practically relevant (no observed crossover)
- **Intrinsic dimensionality** (not ambient) determines algorithmic difficulty: 50pp recall range from low to full intrinsic dim on HNSW (arXiv:2405.17813)
- **Insertion order** shifts HNSW recall by up to 12 percentage points
- **RaBitQ inverse curse**: quantization quality *improves* with dimension, opposite to graph methods — making IVF-RaBitQ especially powerful at 512D+ where it compensates for cluster degradation
- **Algorithm selection guide** by dimension range produced (KD-tree <20D → HNSW/NSW 20-512D → IVF-RaBitQ 512D+)

## Scope Review (Run 3 findings vs specs)

### Do Run 3 findings suggest further scope changes?

Reviewing the three investigation outputs against the amended specs:

1. **diskann-rs filter-aware construction** — Run 2 knew about 3 DiskANN crates but didn't identify diskann-rs as having Filtered-DiskANN API with label combinators. This is already captured under Q1.f; no spec change needed.

2. **EDEN/DRIVE lineage** — The quantization lineage is richer than anticipated (DRIVE 2021 → EDEN 2022 → TurboQuant 2026). This doesn't require a spec change but is noted for Targeted Research depth.

3. **Intrinsic dimensionality as primary driver** — Q1.h originally asked about "dimensionality-dependent behavior" meaning ambient dimension. The finding that *intrinsic* dimensionality matters more is a refinement of the question, not a new question. Already captured.

4. **Matryoshka + quantization interaction** — Open question from Q1.g: how RaBitQ interacts with dimensionality-reduced embeddings. This is a valid future question but belongs in Targeted Research, not R&E scope expansion.

5. **Filtered ANN + dimensionality interaction** — Open question from Q1.h: none of the dimensionality papers study how filtering changes the dimensionality-performance relationship. Same: Targeted Research, not scope expansion.

**Verdict: No further scope changes needed.** The new questions (Q1.f-h) are adequately answered. Open questions from the investigation are deferred to Targeted Research as appropriate.

## Convergence Assessment (Expanded Scope)

### Convergence Criteria (from methodology.md)

- [x] **Algorithm families fully enumerated** — Yes. Stable since Run 2. No new families in Run 3.
- [x] **All relevant Rust crates identified** — Yes. Run 3 added diskann-rs filtering API detail but no new crates.
- [x] **Category boundaries stable** — Yes. Stable since Run 2. Run 3 refined the quantization sub-categories (RaBitQ lineage, EDEN/DRIVE context) but boundaries unchanged.
- [x] **Evaluation criteria operationalized** — Yes. Run 2 metrics + Run 3's new evaluation dimensions (filtered search capability, dimensionality sensitivity) all characterized.
- [x] **Candidate list per category finalized** — Yes. Stable from Run 2 scoping.
- [x] **Knowledge artifacts stable** — Yes. Minor updates to glossary (filtering terms) and taxonomy (RaBitQ lineage) but structurally stable.

### Expanded Scope Criteria

- [x] **Q1.f (Filtered ANN) answered** — Yes. 6 approaches, 5 crates, selectivity thresholds, distribution factor, all cited.
- [x] **Q1.g (Modern quantization) answered** — Yes. 7-method comparison, RaBitQ mechanism detailed, TurboQuant contested, lineage mapped, all cited.
- [x] **Q1.h (Dimensionality behavior) answered** — Yes. 5 crossover points, intrinsic vs ambient analysis, inverse curse for RaBitQ, selection guide, all cited.
- [x] **4 new evaluation dimensions characterized** — Yes. Filtered search (Q1.f), incremental updates (documented for diskann crate), persistence model (documented in Run 2 scoping), platform portability (rabitq-rs ARM64 bugs documented).

### Convergence Verdict

**ALL convergence criteria met, including expanded scope. R&E phase is COMPLETE.**

No Run 4 needed. The next phase is Targeted Research (cohort-based evaluation of Tier 1/2 crates).

## Artifacts Updated

| Artifact | Status | Run 3 Changes |
|----------|--------|---------------|
| `runs/run-3/filtered-ann.md` | New | Q1.f complete investigation |
| `runs/run-3/quantization-comparison.md` | New | Q1.g complete investigation |
| `runs/run-3/dimensionality-behavior.md` | New | Q1.h complete investigation |
| `runs/run-3/report.md` | New | This report |

### Artifacts NOT updated (stable from Run 2)

| Artifact | Reason |
|----------|--------|
| `knowledge/taxonomy.yaml` | No structural changes. RaBitQ lineage detail is in the quantization-comparison.md, doesn't change the taxonomy categories. |
| `knowledge/glossary.yaml` | No new terms needed. FANNS terms are documented inline in filtered-ann.md. |
| `knowledge/references.yaml` | Run 3 papers are cited inline in the investigation docs. Could be backfilled but not blocking. |

## Summary of All Scope Changes Across R&E (for human review)

| Change | Type | Run | Rationale |
|--------|------|-----|-----------|
| Q1.f: Filtered ANN | Expansion | 2→3 | Run 2 found 3+ crates with filtering, a dedicated survey paper, and production systems using it. Not in original questions. |
| Q1.g: Modern quantization | Expansion | 2→3 | RaBitQ/TurboQuant are fundamentally different from PQ (random rotation, no training, theoretical guarantees). Being adopted in production. |
| Q1.h: Dimensionality behavior | Expansion | 2→3 | Hub Highway Hypothesis shows algorithm rankings change qualitatively with dimension, not just in degree. |
| Q3: incremental updates + filtered search | Refinement | 2 | Added as secondary benchmark dimensions. Emerged from Run 2 crate verification. |
| Scope: 4 new evaluation dimensions | Expansion | 2 | Filtered search, incremental updates, persistence model, platform portability. All discovered during Run 2 crate verification. |
| Scope: 3072d stretch | Refinement | 2 | Frontier models pushing wider. swarc benchmarks at 3072d. |
| Scope: maintenance risk | Refinement | 2 | hora abandonment showed this is a real evaluation dimension. |
