# Epistemic Timeline: Vector DS&A

> This document records how understanding evolved across R&E Runs 1-3.
> Each entry captures an epistemic event — a change in category, relationship,
> or validity of a concept. Routine updates (typos, detail additions) are
> tracked in git history, not here.
>
> Entries are append-only. When a later entry supersedes an earlier one,
> the earlier entry gets a `superseded_by` link but is never deleted.

---

## Entries

### ET-001: Initial taxonomy established — 6 categories, 16 algorithm families

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Run** | Run 1 (research-cfj.2.14.1) |
| **Concept** | ANN algorithm taxonomy structure |
| **Artifacts affected** | `knowledge/taxonomy.yaml` (entire file — created) |
| **Commit** | `a6be23f` |
| **Supersedes** | N/A (initial creation) |
| **Superseded by** | ET-004 (taxonomy restructured) |

#### Before-state

> No taxonomy existed. The only prior context was the research questions spec
> (research-questions.md) which listed algorithm families as examples:
> "HNSW, IVF, PQ, DiskANN, random projection trees, LSH, VP-trees, etc."

**Source of prior understanding:** research-questions.md Q1, drafted from training knowledge
**Confidence at the time:** [inferred]

#### After-state

> 6 top-level categories established:
> - `graph_based`: HNSW, NSW, NSG, SSG, Vamana, SPTAG, EFANNA
> - `hash_based`: LSH, neural hashing
> - `quantization_based`: PQ, OPQ, ScaNN, SQ, binary quantization, RaBitQ
> - `tree_based`: RPT, KD-tree, ball tree, VP-tree, MRPT
> - `partition_based`: IVF, IVF-PQ, IVF-HNSW
> - `composite`: HNSW+PQ, IVF+HNSW+PQ, DiskANN+PQ, **Filtered ANN**

#### Transition evidence

**Level 1 — What changed:**
The algorithm landscape was organized from an unstructured list into a hierarchical taxonomy for the first time.

**Level 2 — Why this structure:**
Categories were derived from the primary indexing mechanism: graph traversal, hashing, compression, tree partitioning, cluster partitioning, and combinations thereof. This follows the standard classification used in ANN survey literature. [cited: arXiv:1806.09823 (Andoni et al.), arXiv:2204.07922 (Wang)]

**Level 3 — Deeper context:**
The 6-category structure mirrors the classification in Andoni, Indyk & Razenshteyn's 2018 ICM survey [cited: arXiv:1806.09823], which distinguishes tree-based, hash-based, and graph-based families. The addition of quantization, partition, and composite as separate categories reflects the practical reality that these are distinct implementation strategies, even though quantization is often composed with other families (IVF+PQ). The `composite` category was a catch-all for these compositions. [inferred from: no single survey uses this exact 6-category structure — it's a synthesis of multiple sources]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: research-questions.md] |
| Transition evidence | [cited: arXiv:1806.09823, arXiv:2204.07922] |
| Level 3 context | [inferred from: synthesis of survey structures] |

---

### ET-002: Crate ecosystem mapped — 21 entries, all [inferred]

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Run** | Run 1 (research-cfj.2.14.1) |
| **Concept** | Rust ANN crate ecosystem completeness and verification status |
| **Artifacts affected** | Run 1 report crate table; `knowledge/taxonomy.yaml` `rust_crates` fields |
| **Commit** | `a6be23f` |
| **Supersedes** | N/A (initial mapping) |
| **Superseded by** | ET-003 (live verification), ET-005 (DiskANN discovery) |

#### Before-state

> No systematic crate inventory existed. The research-questions spec mentioned
> "hnswlib-rs", "Arroy", "Fast Vector Similarity" as examples of the fragmented ecosystem.

**Source of prior understanding:** research-questions.md Q2
**Confidence at the time:** N/A

#### After-state

> 21 library crates + 3 database-adjacent crates mapped. All entries marked
> `[inferred]` — derived from training knowledge (pre-May 2025), not live-verified.
> Key gaps flagged: no pure Rust IVF-PQ, no standalone DiskANN, no ScaNN bindings,
> no neural hashing implementations.

#### Transition evidence

**Level 1 — What changed:**
First comprehensive Rust ANN crate inventory created.

**Level 2 — Why [inferred] confidence:**
Run 1 used training knowledge as its source for crate data because live crates.io verification was deferred to Run 2 per methodology (Run 1 = broad sweep, Run 2 = targeted verification). The `[inferred]` marker was applied to every entry to make this limitation explicit. [cited: methodology.md "Run 1 (broad): Cast wide net"]

**Level 3 — Deeper context:**
The decision to accept `[inferred]` data in Run 1 was a deliberate methodological choice: breadth-first discovery followed by depth-first verification. This mirrors the scientific practice of preliminary surveys followed by targeted validation. The risk was that training knowledge would be stale (post-May 2025 crates missed) or wrong (crate names, maintenance status). Run 2 confirmed both risks materialized — 3 crate entries were wrong, 10 new crates were missed. [cited: Run 2 gap-fill report]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: research-questions.md Q2] |
| Transition evidence | [cited: methodology.md] |
| Level 3 context | [cited: Run 2 gap-fill report for materialized risks] |

---

### ET-003: Live verification overturns 3 crate entries, confirms 18

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Run** | Run 2 (research-cfj.2.14.2) |
| **Concept** | Rust ANN crate ecosystem accuracy |
| **Artifacts affected** | `knowledge/taxonomy.yaml` all `rust_crates` fields; Run 2 gap-fill report |
| **Commit** | `96869da` |
| **Supersedes** | ET-002 (all [inferred] entries) |
| **Superseded by** | — |

#### Before-state

> 21 crate entries, all marked `[inferred]`. Specific entries later found to be wrong:
> - `fast-vector-similarity`: listed as a crates.io crate
> - `voyager`: listed as a Rust ANN crate
> - `flann-rs`: listed by this name

**Source of prior understanding:** Run 1 training knowledge survey
**Confidence at the time:** [inferred]

#### After-state

> 18 of 21 crates confirmed via live crates.io checks. 3 corrections:
> - `fast-vector-similarity`: NOT on crates.io (GitHub/PyPI only)
> - `voyager`: crates.io "voyager" is a web crawler, not Spotify's ANN library (Python/Java only)
> - `flann-rs`: actual crate name is `flann` (v0.1.0, abandoned 2019)
>
> All surviving entries upgraded from `[inferred]` to `[verified]`.
> 10 new post-2025 crates discovered (see ET-005, ET-006).

#### Transition evidence

**Level 1 — What changed:**
3 crate entries corrected, 18 verified, all upgraded from [inferred] to [verified].

**Level 2 — Why the corrections:**
- `fast-vector-similarity`: crates.io search returned no results. WebSearch found it on GitHub (Dicklesworthstone/fast_vector_similarity) with PyPI distribution only. [verified: crates.io search 2026-04-24]
- `voyager`: crates.io "voyager" (by mattsse) is a web crawler. Spotify's Voyager ANN library provides Python and Java bindings only; community `voyager-rs` exists on GitHub but is unpublished to crates.io. [verified: crates.io + GitHub search 2026-04-24]
- `flann-rs`: No crate named "flann-rs" on crates.io. The actual crate is `flann` (v0.1.0) + `flann-sys` (v0.1.0), last updated 2019-02-25. [verified: crates.io search 2026-04-24]

**Level 3 — Deeper context:**
These errors illustrate a systematic risk of training-knowledge-based surveys: crate naming conventions and cross-language availability are especially prone to stale or imprecise information. The `voyager` case is particularly instructive — the name collision between an ANN library and a web crawler is invisible from training data alone. This validates the methodology's insistence on live verification in Run 2. [inferred from: pattern of errors — all 3 relate to crate naming/availability assumptions rather than algorithmic content]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: Run 1 report] |
| Transition evidence | [verified: crates.io live search 2026-04-24] |
| Level 3 context | [inferred from: error pattern analysis] |

---

### ET-004: Taxonomy restructured — 6 categories → 5 families + cross-cutting + patterns

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Run** | Run 2 (research-cfj.2.14.2) |
| **Concept** | ANN algorithm taxonomy top-level structure |
| **Artifacts affected** | `knowledge/taxonomy.yaml` (restructured) |
| **Commit** | `96869da` |
| **Supersedes** | ET-001 |
| **Superseded by** | — |

#### Before-state

> 6 top-level categories: `graph_based`, `hash_based`, `quantization_based`,
> `tree_based`, `partition_based`, `composite`.
>
> All graph algorithms (HNSW, NSW, NSG, SSG, Vamana, SPTAG, EFANNA) were in
> a single `graph_based` family. Filtered ANN was listed under `composite`.
> NSW and HNSW were listed as separate algorithm families.

**Source of prior understanding:** Run 1 taxonomy draft (taxonomy.yaml at commit a6be23f)
**Confidence at the time:** [inferred] — draft status explicitly noted

#### After-state

> 5 algorithm families: `navigable_small_world` (HNSW+NSW merged),
> `mrng_derived` (NSG, SSG, EFANNA, Vamana, SPTAG), `hash_based`,
> `quantization_based` (with RaBitQ and TurboQuant as new sub-categories),
> `tree_based`, `partition_based`.
>
> Plus: `cross_cutting` section (filtered ANN, persistence, SIMD),
> `composition_patterns` section (HNSW+PQ, IVF+RaBitQ, etc.),
> `utility_crates`, `removed` sections.

#### Transition evidence

**Level 1 — What changed:**
Graph family split into two sub-families by theoretical basis. Filtered ANN moved from algorithm family to cross-cutting concern. Composite removed as a category. NSW/HNSW merged.

**Level 2 — Why, question by question:**

*Q1 (NSW/HNSW merge):* arXiv:2412.01940 (Munyampirwa et al. 2024) demonstrated that flat NSW matches HNSW in high dimensions via hub highways. The hierarchy is an optimization, not a fundamentally different algorithm. [cited: arXiv:2412.01940]

*Q2 (RaBitQ as own category):* RaBitQ uses random rotation + binary encoding on hypercube vertices — a fundamentally different mechanism from PQ's subspace codebooks. Has asymptotic optimality guarantees PQ lacks. [cited: arXiv:2405.12497]

*Q3 (Filtered ANN → cross-cutting):* Run 2 found 3 crates implementing filtering as an orthogonal capability on different algorithm families: hnsw_rs (HNSW + Filterable trait), usearch (HNSW + predicate fn), arroy (RPT + RoaringBitmap). Same concept, different mechanisms, different algorithms — filtering is a capability, not a family. [verified: docs.rs API analysis 2026-04-24]

*Q4 (NSG/SSG/EFANNA/Vamana grouped as MRNG-derived):* NSG is explicitly an approximation of MRNG (Monotonic Relative Neighborhood Graph), rooted in Toussaint's RNG (1980). This is a different theoretical basis from NSW's small-world network properties (Watts & Strogatz 1998). [cited: arXiv:1707.00143 (Fu et al.)]

*Q5 (Composite removed):* Composites (HNSW+PQ, IVF+HNSW+PQ) are deployment patterns, not algorithm families. Each component belongs in its native category. Documenting them as patterns avoids double-counting. [inferred from: no survey treats composites as a peer category to the base algorithms they compose]

**Level 3 — Deeper context:**
The original single `graph_based` category grouped algorithms with fundamentally different theoretical roots: NSW/HNSW derives from small-world network theory (Watts & Strogatz 1998, applied to nearest neighbor search by Malkov 2014), while NSG derives from computational geometry's Relative Neighborhood Graph (Toussaint 1980, extended to MRNG by Fu et al. 2017). These are distinct mathematical traditions — one from network science, one from geometric graph theory — that converge on similar practical structures (proximity graphs for ANN). The split makes this distinction explicit. [cited: Watts & Strogatz 1998 for small-world; Toussaint 1980 for RNG; Fu et al. arXiv:1707.00143 for MRNG]

The removal of `composite` as a top-level category aligns with how the FANNS survey (Lin et al. 2025) treats index composition — as a design choice orthogonal to the base algorithm, not as a separate algorithm family. [cited: arXiv:2505.06501]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [verified: git show a6be23f:taxonomy.yaml] |
| Q1 transition (NSW/HNSW) | [cited: arXiv:2412.01940] |
| Q2 transition (RaBitQ) | [cited: arXiv:2405.12497] |
| Q3 transition (Filtered ANN) | [verified: docs.rs 2026-04-24] |
| Q4 transition (MRNG-derived) | [cited: arXiv:1707.00143, Toussaint 1980] |
| Q5 transition (composite removal) | [inferred from: no survey treats composites as peer category] |
| Level 3 (theoretical traditions) | [cited: Watts & Strogatz 1998, Toussaint 1980, arXiv:1707.00143] |
| Level 3 (FANNS alignment) | [cited: arXiv:2505.06501] |

---

### ET-005: DiskANN Rust availability overturned — "absent" → 3 standalone crates

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Run** | Run 2 (research-cfj.2.14.2) |
| **Concept** | DiskANN/Vamana Rust implementation availability |
| **Artifacts affected** | `knowledge/taxonomy.yaml` → `families.mrng_derived.algorithms[Vamana].rust_crates` |
| **Commit** | `96869da` |
| **Supersedes** | ET-002 (DiskANN portion) |
| **Superseded by** | — |

#### Before-state

> "DiskANN in Rust: Only lance implements DiskANN-style indexing internally.
> Is there a standalone crate? Need verification."
> — Run 1 report, Open Questions item 3
>
> taxonomy.yaml listed: `rust_crates: ["lance (internal)"]`

**Source of prior understanding:** Run 1 training knowledge survey
**Confidence at the time:** [inferred]

#### After-state

> Three standalone pure-Rust DiskANN crates discovered:
> - `diskann` (v0.50.0, INFINI Labs, 7,878 downloads, 9 versions) — forked from Microsoft's partial Rust port, extended to full implementation with Filtered-DiskANN support
> - `diskann-rs` (updated Feb 2026) — alternative implementation with filter-aware construction API (`Filter::label_eq`/`label_range`/`and`)
> - `rust-diskann` (by jianshu93) — native Rust with generic distance trait

#### Transition evidence

**Level 1 — What changed:**
DiskANN went from "absent in standalone Rust" to "3 independent implementations available."

**Level 2 — Why it changed:**
Run 1 relied on training knowledge (pre-May 2025) which did not include these crates. Run 2's live crates.io search for "diskann" and "vamana" discovered all three. The `diskann` crate alone has 9 published versions, suggesting it was actively developed during the period Run 1's training data couldn't see. [verified: crates.io search 2026-04-24]

**Level 3 — Deeper context:**
The `diskann` crate (INFINI Labs) was forked from Microsoft's own partial Rust port within the DiskANN repository [cited: GitHub infinilabs/diskann README]. Microsoft published a partial Rust implementation alongside their C++ DiskANN repo, suggesting institutional interest in Rust for this algorithm class. The `diskann-rs` crate independently implemented Filtered-DiskANN (Gollapudi et al. WWW'23), including filter-aware graph construction with label combinators [verified: diskann-rs crates.io page]. Three independent implementations appearing in 2025-2026 suggests DiskANN adoption is accelerating in the Rust ecosystem. [inferred from: temporal pattern of 3 independent implementations — could be driven by the popularity of vector search in Rust-based systems like Meilisearch, LanceDB, and Qdrant, all of which have DiskANN-adjacent needs]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: Run 1 report Open Questions #3] |
| Transition evidence | [verified: crates.io search 2026-04-24] |
| Level 3 (Microsoft fork) | [cited: GitHub README] |
| Level 3 (ecosystem acceleration) | [inferred from: temporal pattern + Rust vector DB ecosystem growth] |

---

### ET-006: Modern quantization paradigm discovered — RaBitQ/TurboQuant alongside PQ

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 (Run 2 discovery) → 2026-04-25 (Run 3 deep-dive) |
| **Run** | Run 2 (discovery) + Run 3 (characterization) |
| **Concept** | Quantization method landscape — PQ-centric → multi-paradigm |
| **Artifacts affected** | `knowledge/taxonomy.yaml` → `families.quantization_based.algorithms`; `specs/research-questions.md` Q1.g |
| **Commit** | `96869da` (Run 2 taxonomy), `e048eb8` (Run 3 comparison) |
| **Supersedes** | ET-001 (quantization section) |
| **Superseded by** | — |

#### Before-state

> Quantization category listed: PQ, OPQ, ScaNN, SQ, binary quantization, RaBitQ.
> RaBitQ was included but noted as "Newer method, needs deeper investigation in Run 2."
> Q1.c asked specifically about ScaNN but not about alternative quantization paradigms.
> The implicit assumption was that PQ was the dominant quantization approach with
> ScaNN as its main competitor.

**Source of prior understanding:** Run 1 taxonomy; research-questions.md Q1.c
**Confidence at the time:** [inferred] for RaBitQ characterization

#### After-state

> Three distinct quantization paradigms identified:
> 1. **Subspace codebook** (PQ family): PQ, OPQ — learned, data-dependent, no guarantees
> 2. **Randomized binary** (RaBitQ family): RaBitQ, Extended RaBitQ — data-oblivious, O(1/sqrt(D)) error bound, asymptotically optimal
> 3. **Data-oblivious scalar** (TurboQuant family): TurboQuant, PolarQuant, QJL — data-oblivious, near-optimal but contested
>
> RaBitQ dominates at 32x compression. TurboQuant contested by 2 independent teams.
> PQ has no theoretical guarantees and fails on some real datasets (<60% recall on MSong).
> New Q1.g added to research questions to cover this landscape.
> Rust crates: rabitq-rs, turbo-quant, turbovec (all new, post-2025).

#### Transition evidence

**Level 1 — What changed:**
Quantization landscape went from PQ-centric to multi-paradigm with fundamentally different mechanism families.

**Level 2 — Why it changed:**
Run 2 discovered rabitq-rs, turbo-quant, and turbovec on crates.io — Rust implementations of methods published after the original research questions were written. Run 3's deep-dive confirmed RaBitQ uses a fundamentally different mechanism from PQ (random rotation + hypercube vertex encoding, not subspace codebooks) with theoretical guarantees PQ lacks. [cited: arXiv:2405.12497 for RaBitQ mechanism; verified: crates.io for Rust implementations]

The RaBitQ paper explicitly demonstrates PQ's failure modes: "maximum relative error can reach ~100% on MSong and Word2Vec datasets" while RaBitQ provides O(1/sqrt(D)) error bound matching information-theoretic lower bounds. [cited: arXiv:2405.12497]

**Level 3 — Deeper context:**
RaBitQ (SIGMOD 2024) and TurboQuant (ICLR 2026) both exploit the same mathematical insight: random rotation concentrates vector coordinates toward a Beta distribution, enabling effective per-coordinate quantization. However, RaBitQ uses hypercube vertex encoding while TurboQuant uses per-coordinate scalar quantization with analytically derived boundaries. [cited: arXiv:2405.12497, arXiv:2504.19874]

The EDEN team (arXiv:2604.18555) claims TurboQuant's core mechanism was established by DRIVE (NeurIPS 2021) and EDEN (ICML 2022) for gradient compression, 4 years before TurboQuant applied it to ANN search. They demonstrate that EDEN's optimized scale parameter yields strictly better results than TurboQuant's fixed S=1. [cited: arXiv:2604.18555] The DRIVE/EDEN team is directly aware of both RaBitQ and TurboQuant. Their note (arXiv:2604.18555) cites the RaBitQ paper in References [3]-[4] and states in Footnote 2: "the authors of the RaBitQ paper have expressed similar concerns regarding their paper." They explicitly claim temporal priority: "EDEN and DRIVE also predate the RaBitQ work." There is an active three-way priority dispute between the DRIVE/EDEN, RaBitQ, and TurboQuant teams. [cited: arXiv:2604.18555 Footnote 2, References [3]-[4]; verified: V4]

The RaBitQ team's independent evaluation (arXiv:2604.19528) found TurboQuant performs worse than RaBitQ "in many tested configurations" with reproducibility concerns about published benchmarks (~100x slower quantization times than claimed). [cited: arXiv:2604.19528]

Production adoption is already underway: RaBitQ is integrated into NVIDIA cuVS (GPU IVF-RaBitQ, arXiv:2602.23999), Elasticsearch, and LanceDB. [cited: arXiv:2602.23999 for cuVS; verified: Elasticsearch Labs blog for ES adoption]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: Run 1 taxonomy.yaml, research-questions.md] |
| RaBitQ mechanism and guarantees | [cited: arXiv:2405.12497] |
| PQ failure modes | [cited: arXiv:2405.12497] |
| TurboQuant contestation | [cited: arXiv:2604.19528, arXiv:2604.18555] |
| DRIVE/EDEN lineage | [cited: arXiv:2604.18555] |
| DRIVE/EDEN-RaBitQ-TurboQuant awareness | [cited: arXiv:2604.18555 — active 3-way priority dispute confirmed] |
| Production adoption | [cited: arXiv:2602.23999] + [verified: ES Labs blog] |

---

### ET-007: Filtered ANN reclassified — algorithm family → cross-cutting concern

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 (Run 2 reclassification) → 2026-04-25 (Run 3 deep characterization) |
| **Run** | Run 2 (reclassification) + Run 3 (Q1.f deep-dive) |
| **Concept** | Filtered ANN's place in the taxonomy |
| **Artifacts affected** | `knowledge/taxonomy.yaml` → moved from `composite.patterns` to `cross_cutting.filtered_ann`; `specs/research-questions.md` Q1.f added |
| **Commit** | `96869da` (reclassification), `e048eb8` (Q1.f investigation) |
| **Supersedes** | ET-001 (composite section) |
| **Superseded by** | — |

#### Before-state

> Filtered ANN was listed under `composite` as a composition pattern:
> ```yaml
> composite:
>   patterns:
>     - name: "Filtered ANN"
>       description: "ANN with scalar attribute constraints (pre/post/in-filter)"
>       key_paper: "arXiv:2505.06501 (survey); Filtered-DiskANN"
> ```
>
> The research-questions.md Open Questions item #2 asked: "Filtered ANN search:
> Pre-filter vs post-filter vs in-filter approaches should be an explicit
> evaluation dimension — add to Q1 or Q6.b?" — flagged but never resolved.

**Source of prior understanding:** Run 1 taxonomy (composite section); research-questions.md Open Question #2
**Confidence at the time:** [inferred] — placement in composite was provisional

#### After-state

> Filtered ANN is a **cross-cutting concern**, not an algorithm family or composition
> pattern. It's a capability that any algorithm can support orthogonally.
>
> Run 3 further revealed that the simple pre/post/in-filter taxonomy is itself
> superseded by the FANNS survey's pruning-focused framework:
> - VSP (Vector-Solely Pruning = post-filter)
> - VJP (Vector-Centric Joint Pruning = in-filter, with filter-aware construction as subclass)
> - SSP (Scalar-Solely Pruning = pre-filter)
> - SJP (Scalar-Centric Joint Pruning = hybrid/partition-based)
> - Window filters (specialized for numeric range queries)
>
> 5 Rust crates support filtering: hnsw_rs, usearch, arroy, diskann-rs, diskann (infinilabs).

#### Transition evidence

**Level 1 — What changed:**
Filtered ANN moved from "composite algorithm pattern" to "cross-cutting concern." The filtering approach taxonomy expanded from 3 to 6 approaches.

**Level 2 — Why reclassified (Run 2):**
Run 2 found that 3 crates implement filtering as an orthogonal capability on different base algorithms:
- hnsw_rs: in-filter on HNSW graph via Filterable trait
- usearch: in-filter on HNSW graph via predicate function
- arroy: pre-filter with bitmap intersection on random projection trees
Same capability, different algorithms, different mechanisms. This is the definition of a cross-cutting concern. [verified: docs.rs API analysis 2026-04-24]

**Level 2 — Why 3→6 approaches (Run 3):**
The FANNS survey (Lin et al. 2025, arXiv:2505.06501) introduced the pruning-focused framework (VSP/VJP/SSP/SJP) which makes finer distinctions than pre/post/in-filter. For example, "in-filter" doesn't distinguish between runtime-only filtering (VJP) and filter-aware index construction (VJP subclass). The survey also identified SJP (scalar-centric joint pruning) and window filters as distinct approaches not captured by the original trichotomy. [cited: arXiv:2505.06501]

**Level 3 — Deeper context:**
The pre/post/in-filter trichotomy originated as industry terminology from vector database vendors (Pinecone, Weaviate, Qdrant, Milvus documentation), likely between 2020-2022. The FANNS survey (Lin et al. 2025, arXiv:2505.06501) cites 6 papers that use it but attributes no single originating paper. The earliest datable academic usage is Gollapudi et al. (WWW 2023, Filtered-DiskANN). [verified: V1 — FANNS survey citations checked, no origin paper found; Gollapudi et al. 2023 is earliest academic use] The FANNS survey explicitly critiques the trichotomy as insufficient and proposes a replacement pruning-focused framework because the pre/post/in-filter scheme is "too coarse to distinguish between algorithms" and "not enough to cover all algorithms." [cited: arXiv:2505.06501]

The Filtered-DiskANN paper (Gollapudi et al., WWW'23) demonstrated that filter-aware graph construction achieves 6x better QPS than in-filter approaches on standard graphs, establishing filter-aware construction as a distinct and superior approach for moderate-to-high selectivity workloads. This was the key evidence that "in-filter" was an insufficient category — it lumped together fundamentally different strategies. [cited: Filtered-DiskANN, Gollapudi et al. WWW'23]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [verified: git show a6be23f:taxonomy.yaml] |
| Reclassification evidence | [verified: docs.rs API analysis] |
| FANNS framework | [cited: arXiv:2505.06501] |
| Pre/post/in-filter origin | [verified: V1 — industry terminology, no single academic source; earliest academic use Gollapudi et al. WWW 2023] |
| Filtered-DiskANN superiority | [cited: Gollapudi et al. WWW'23] |

---

### ET-008: HNSW hierarchy found unnecessary above ~32D

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 (Run 2 noted) → 2026-04-25 (Run 3 characterized) |
| **Run** | Run 2 (noted in taxonomy decision) + Run 3 (Q1.h investigation) |
| **Concept** | HNSW hierarchy necessity as a function of dimensionality |
| **Artifacts affected** | `knowledge/taxonomy.yaml` → `families.navigable_small_world` (NSW/HNSW merged); `knowledge/glossary.yaml` → NSW, HNSW entries |
| **Commit** | `96869da` (taxonomy merge), `e048eb8` (dimensionality analysis) |
| **Supersedes** | ET-001 (HNSW/NSW as separate families) |
| **Superseded by** | — |

#### Before-state

> HNSW and NSW were listed as separate algorithm families under `graph_based`:
> - HNSW: "Multi-layer proximity graph with skip-list-like hierarchy..."
> - NSW: "Single-layer small-world graph (predecessor to HNSW)..."
>   with note: "Hub Highway Hypothesis suggests flat NSW matches HNSW in high-d"
>
> The implicit understanding was that HNSW's hierarchy was a core feature that
> made it fundamentally superior to flat NSW.

**Source of prior understanding:** Run 1 taxonomy; standard ANN literature treating HNSW as an improvement over NSW
**Confidence at the time:** [cited: arXiv:1603.09320 for HNSW; Malkov 2014 for NSW] — but the relationship between them was conventional wisdom, not critically examined

#### After-state

> HNSW and NSW merged into a single `navigable_small_world` family.
> The hierarchy is classified as an optional optimization effective only below ~32D.
> Above ~32D (which includes all modern embedding dimensions: 96D+), hub nodes
> naturally form "highways" making flat NSW equivalent to HNSW with 38-39% less memory.

#### Transition evidence

**Level 1 — What changed:**
HNSW hierarchy went from "core algorithmic feature" to "optional optimization for low-dimensional data."

**Level 2 — Why:**
Munyampirwa et al. (arXiv:2412.01940, 2024) tested on 10 datasets spanning 16D to 960D and 60K to 100M vectors. Key findings: "no consistent and discernible gap between FlatNav and HNSW" above 96D. Flat NSW achieves "38% and 39% memory savings during index construction" on Big-ANN benchmarks. The hierarchy's routing function is replicated by naturally occurring hub nodes in high dimensions. [cited: arXiv:2412.01940]

Elliott & Clark (arXiv:2405.17813, 2024) independently showed that intrinsic dimensionality, not ambient dimension, determines HNSW recall — with a 50 percentage point recall range from low to full intrinsic dimensionality. This provides additional evidence that the hierarchy's value is dimensionality-dependent. [cited: arXiv:2405.17813]

**Level 3 — Deeper context:**
The original HNSW paper (Malkov & Yashunin, arXiv:1603.09320, 2016) presented the hierarchy as central to HNSW's logarithmic complexity claim. The hierarchy was inspired by skip lists (Pugh 1990) and was considered the key innovation over the earlier NSW algorithm (Malkov et al. 2014). The HNSW paper directly cites Watts & Strogatz 1998, Kleinberg 2000, and Travers & Milgram 1977 — the NSW/HNSW research program is explicitly grounded in small-world network theory. Malkov's 2015 paper "Growing Homophilic Networks Are Natural Navigable Small Worlds" (PLOS ONE) is entirely a theoretical exploration of navigability in Watts-Strogatz networks. [cited: arXiv:1603.09320 reference list — Watts & Strogatz directly cited; verified: V2]

The Hub Highway Hypothesis challenges this narrative by showing that in high dimensions, the small-world property of the graph naturally produces "hub" nodes that are visited disproportionately often during search. These hubs form navigable highways that serve the same routing function as hierarchical layers. Hub formation is driven by distance concentration — a well-known high-dimensional phenomenon where the ratio of nearest-to-farthest distances approaches 1, creating nodes that are "close to everything." [cited: arXiv:2412.01940 for hub highway mechanism]

The ~32D crossover point is approximate. The paper explicitly states "the vector dimensionality and not the size of the collection is the main driver of eliminating the need for hierarchical search" — however, no systematic size ablation was performed (datasets ranged from 60K to 100M vectors but differed in dimensionality, distribution, and structure simultaneously). The claim is well-supported across all tested sizes but not rigorously isolated from confounding variables. The exact crossover likely also depends on intrinsic dimensionality. [cited: arXiv:2412.01940 for the explicit dimensionality-not-size claim; verified: V3 — no size ablation found in experimental setup]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [verified: git show a6be23f:taxonomy.yaml] |
| Hub Highway findings | [cited: arXiv:2412.01940] |
| Memory savings | [cited: arXiv:2412.01940] |
| ~32D crossover | [cited: arXiv:2412.01940] + [verified: V3 — dimensionality is the driver per paper, but no size ablation performed] |
| Watts & Strogatz lineage | [verified: V2 — directly cited in HNSW paper reference list] |
| Distance concentration as mechanism | [cited: arXiv:2412.01940] |

---

### ET-009: hora confirmed abandoned

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 |
| **Run** | Run 2 (research-cfj.2.14.2) |
| **Concept** | hora crate maintenance status |
| **Artifacts affected** | `knowledge/taxonomy.yaml` → `removed` section |
| **Commit** | `96869da` |
| **Supersedes** | ET-002 (hora portion) |
| **Superseded by** | — |

#### Before-state

> hora listed as: "HNSW, SSG, PQIVF, brute-force | Pure Rust | ~2022
> (possibly unmaintained) | Multiple algorithms, WASM support | [inferred]"
>
> Run 1 Open Question #4: "hora maintenance: Last known activity ~2022.
> Confirm if abandoned or still maintained."

**Source of prior understanding:** Run 1 training knowledge
**Confidence at the time:** [inferred] — "possibly unmaintained" was a hedge

#### After-state

> hora confirmed abandoned: last crate publish ~Aug 2021, 4.5+ years stale.
> Only 2 versions ever published (0.1.0, 0.1.1). 35,163 all-time downloads
> but only ~477/month (residual dependency usage). GitHub shows 2.7k stars
> but no recent commits. Open issues unanswered. Fork `hora-new` exists
> (v0.0.2) but also appears inactive. Moved to `removed` section in taxonomy.

#### Transition evidence

**Level 1 — What changed:**
hora went from "possibly unmaintained" to "confirmed abandoned."

**Level 2 — Why:**
Live crates.io verification showed last publish ~Aug 2021. lib.rs metadata confirmed ~477 downloads/month (residual, not active adoption). [verified: crates.io + lib.rs 2026-04-24]

**Level 3 — Deeper context:**
hora was one of the more ambitious pure-Rust ANN projects, claiming HNSW, SSG, PQIVF, and brute-force support with WASM capabilities — all in 4K SLoC within a 9.5MB crate (suggesting bulk was test data/assets). Its GitHub star count (~2,659) is disproportionately high relative to its downloads and maintenance. hora 0.1.0 was announced on the Rust forum on July 31, 2021; the final publish (0.1.1) was August 6, 2021 — a total active life of 6 days on crates.io. [verified: V5 — crates.io dates confirmed; ~2,659 stars approximately matches 2.7k claim] The high star count likely reflects the initial wave of Rust community interest from the forum announcement, never sustained into production quality. [inferred from: 6-day active window + star-to-download ratio — the project attracted attention but shipped only one patch before going silent]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: Run 1 report] |
| Transition evidence | [verified: crates.io + lib.rs 2026-04-24] |
| Level 3 context | [inferred from: star-to-download ratio pattern] |

---

### ET-010: Scope expanded — 3 research questions and 4 evaluation dimensions added

| Field | Value |
|-------|-------|
| **Date** | 2026-04-24 (Run 2 discovery) → 2026-04-25 (specs amended) |
| **Run** | Run 2 (scope review) |
| **Concept** | Research questions and scope completeness |
| **Artifacts affected** | `specs/research-questions.md` (Q1.f, Q1.g, Q1.h added); `specs/scope.md` (4 evaluation dimensions added) |
| **Commit** | `afba501` |
| **Supersedes** | N/A (expansion, not replacement) |
| **Superseded by** | — |

#### Before-state

> research-questions.md defined Q1 with sub-questions Q1.a through Q1.e.
> Q1.c asked about ScaNN specifically. No question addressed filtered ANN,
> modern quantization paradigms beyond ScaNN, or dimensionality-dependent behavior.
>
> scope.md listed 6 evaluation dimensions: performance, correctness, scalability,
> code quality, maturity, composability.
>
> Open Question #2 flagged filtered ANN but left it unresolved.

**Source of prior understanding:** Original specs drafted during Phase 0
**Confidence at the time:** N/A — these were scoping decisions, not factual claims

#### After-state

> 3 new sub-questions added:
> - Q1.f: Filtered ANN approaches and Rust support
> - Q1.g: Modern quantization landscape (RaBitQ, TurboQuant)
> - Q1.h: Dimensionality-dependent algorithm behavior
>
> 4 evaluation dimensions added: filtered search capability, incremental update
> capability, persistence model, platform portability.
>
> Run 2 convergence reset for expanded scope → triggered Run 3.

#### Transition evidence

**Level 1 — What changed:**
The research scope expanded based on findings that revealed dimensions the original specs didn't anticipate.

**Level 2 — Why each addition:**
- Q1.f: Run 2 found 3+ crates implementing filtered ANN, a dedicated survey paper, and production systems using it. This was the biggest gap — for any real application, constrained search is table stakes. [verified: crate APIs + arXiv:2505.06501]
- Q1.g: RaBitQ and TurboQuant are fundamentally different from PQ (different mechanism, theoretical guarantees, zero training). Being adopted in production (Elasticsearch, LanceDB, NVIDIA cuVS). [cited: arXiv:2405.12497; verified: crates.io for rabitq-rs]
- Q1.h: The Hub Highway Hypothesis shows algorithm rankings change qualitatively with dimensionality, not just in degree. [cited: arXiv:2412.01940]
- Evaluation dimensions: each emerged from specific Run 2 findings (hora abandonment → maintenance risk; arroy LMDB vs kiddo mmap → persistence model; rabitq-rs ARM64 bugs → platform portability). [verified: Run 2 gap-fill findings]

**Level 3 — Deeper context:**
This scope expansion illustrates a general pattern in research methodology: initial scoping is necessarily incomplete because you don't know what you don't know until you start exploring. The formulas were updated with a "Scope Review" step (rule #10, formula v4) to institutionalize this feedback loop for all future research initiatives. [inferred from: this being the first time the pattern was recognized and formalized — prior research initiatives may have had the same problem without the mechanism to address it]

#### Confidence assessment

| Claim | Confidence |
|-------|-----------|
| Before-state accuracy | [cited: research-questions.md, scope.md at original commit] |
| Each Q1.f/g/h rationale | [cited/verified: as listed per question] |
| Level 3 (pattern recognition) | [inferred from: first instance of formalized scope review] |

---

## Verification Log

> Verification questions extracted during provenance synthesis and resolved via
> targeted grounding research. Full details in `runs/run-3/provenance-verification.md`.

| # | Question | Entry | Resolution | Confidence |
|---|----------|-------|-----------|------------|
| V1 | Pre/post/in-filter trichotomy origin | ET-007 | Industry terminology, no single academic source; earliest academic use Gollapudi et al. WWW 2023 | [verified: FANNS survey citations checked] |
| V2 | Watts & Strogatz → Malkov lineage | ET-008 | **Confirmed** — W&S 1998 directly cited in HNSW paper | [verified: reference list] |
| V3 | ~32D crossover size-dependent? | ET-008 | Paper claims dimensionality is the driver; no size ablation performed | [verified: paper's explicit claim + noted limitation] |
| V4 | DRIVE/EDEN aware of RaBitQ? | ET-006 | **Yes** — direct citation, footnote, active 3-way priority dispute | [cited: arXiv:2604.18555] |
| V5 | hora activity after Aug 2021? | ET-009 | Last publish Aug 6, 2021; ~2,659 stars; likely no code activity since | [verified: crates.io + web search] |
