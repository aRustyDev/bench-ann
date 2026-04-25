# Scope & Boundary Definition: Vector DS&A

> **Bead**: `research-cfj.2.13.2`
> **Status**: draft
> **Date**: 2026-04-24

## In Scope

### Algorithms & Theory
- Approximate Nearest Neighbor (ANN) algorithm families: HNSW, IVF, PQ, DiskANN/Vamana, random projection trees (Annoy), LSH, VP-trees, ball trees, graph-based (NSG, MRNG, relative neighborhood graph)
- Exact kNN as baseline — understanding when exact becomes impractical
- ScaNN and learned quantization approaches
- **Modern quantization paradigms: RaBitQ (randomized binary quantization), TurboQuant/PolarQuant/QJL (data-oblivious quantization)** *(added Run 2 — these are fundamentally different from PQ and are being adopted in production; Rust implementations exist)*
- Neural hashing and learning-to-hash methods
- Vector indexing techniques and their composition (IVF+PQ, HNSW+PQ, IVF+RaBitQ, DiskANN+PQ, etc.)
- Similarity/distance metrics: cosine, L2, dot product, Manhattan, Hamming, Jaccard, angular
- Quantization techniques: scalar, product (PQ, OPQ), binary, randomized binary (RaBitQ), data-oblivious (TurboQuant), learned (ScaNN)
- kNN ↔ ANN crossover analysis (scale, hardware, scenario thresholds)
- **Filtered/constrained ANN search (FANNS): pre-filter, post-filter, in-filter approaches** *(added Run 2 — cross-cutting concern for all algorithm families; 3 Rust crates support it)*
- **Dimensionality-dependent algorithm behavior: Hub Highway Hypothesis and qualitative behavioral shifts across dimensionality ranges** *(added Run 2)*

### Implementations
- **Rust crates** implementing any of the above algorithms (primary focus)
  - Pure Rust implementations
  - Rust bindings/wrappers around C/C++ libraries
  - Rust crates that embed vector search as a feature (e.g., within a larger framework)
- **Reference implementations in other languages** (secondary — for architectural study)
  - FAISS (C++/Python), ScaNN (C++/Python), Annoy (C++/Python), hnswlib (C++), Vamana/DiskANN (C++), SPTAG (C++)
  - Studied for architecture and design patterns, not for direct adoption

### Evaluation Dimensions
- Performance: recall@k, QPS, build time, memory per vector, index size on disk, latency distribution (p50/p99)
- Correctness: recall accuracy at various parameter settings
- Scalability: behavior across dimensionalities (128d-1536d, with 3072d as stretch) and dataset sizes (10K-10M)
- Code quality: unsafe usage, dependency count, API design, documentation
- Maturity: maintenance status, community, production usage signals, **maintenance risk** *(added Run 2 — hora's abandonment shows this is a real concern)*
- Composability: how well components combine (e.g., using one crate's HNSW with another's quantization)
- **Filtered search capability**: which crates support constrained/filtered ANN, which approach (pre/post/in-filter), and API quality *(added Run 2)*
- **Incremental update capability**: insert/delete without full index rebuild, and its cost vs. rebuild *(added Run 2)*
- **Persistence model**: mmap, LMDB, SSD-resident, serialization — and whether indexes survive process restart *(added Run 2)*
- **Platform portability**: x86_64 vs. ARM64 support, SIMD dependency *(added Run 2 — rabitq-rs is x86_64-only)*

### Deployment Model Analysis
- In-process library trade-off profiles
- Embedded database trade-off profiles
- External database/service trade-off profiles
- Pure vector vs hybrid (graph+vector) trade-off profiles
- Documented as frameworks for decision-making, not as recommendations

## Out of Scope

### Explicitly excluded
- **Full vector database evaluation as products** — that's `research-cfj.1` (Vector Databases). We survey them as reference implementations in Q6.a, but don't evaluate their operational characteristics (clustering, replication, cloud deployment, pricing).
- **Hybrid database evaluation** — that's `research-anj.1` (Graph+Vector Databases). We document the pure-vs-hybrid trade-off profile (Q5.b) but don't deep-dive CozoDB/SurrealDB/HelixDB.
- **Embedding model evaluation** — that's `research-ohb.1.2.11` (SentenceTransformers). We take embeddings as input; we don't evaluate how they're generated.
- **Project-specific decisions** — no "Forge should use X" or "we should adopt Y". Produce decision frameworks, not decisions.
- **Implementation** — no code is written during this research. Building comes after.
- **GPU-specific implementations** — focus on CPU-based algorithms. GPU acceleration is relevant context but not a primary evaluation dimension.
- **Distributed/multi-node algorithms** — focus on single-node implementations. Distributed systems are architectural context, not implementation targets.

### Adjacent but separate
- Vector database benchmarking (research-cfj.1 will have its own benchmark phase)
- Graph database vector capabilities (research-anj.1 covers this)
- Embedding generation and fine-tuning (research-ohb.1)

## Inclusion Criteria for Crates

A Rust crate is in scope if it:
1. Implements at least one ANN/kNN algorithm, OR
2. Provides vector distance/similarity computation primitives, OR
3. Provides vector index management (build, query, serialize/deserialize)

A crate is **excluded** if it:
1. Is a full database (evaluated in research-cfj.1 instead)
2. Only provides embedding model inference (evaluated in research-ohb.1 instead)
3. Has zero commits in the last 2 years AND fewer than 100 downloads (abandoned, no reference value)

**Exception**: abandoned crates with interesting architecture are kept for Q6.a reference purposes, tagged as `[abandoned — reference only]`.

## Inclusion Criteria for Reference Implementations (non-Rust)

A non-Rust implementation is in scope if it:
1. Is widely cited or used as a benchmark reference (FAISS, ScaNN, hnswlib), OR
2. Introduces a novel algorithm or architecture pattern worth studying, OR
3. Has an existing Rust port or binding that we're evaluating

**Study rubric for reference implementations**: Document algorithm design, data structure choices, API surface, and architectural patterns. Do NOT document: deployment guides, cloud pricing, operational runbooks, or product-level feature comparisons. The goal is to learn from their architecture, not evaluate them as products.

## Assumptions

These assumptions scope the **primary focus** for Q3 (benchmarks) and Q4 (decision framework). Q1 (taxonomy), Q2 (crate survey), and Q6 (implementation landscape) survey broadly — including sparse vectors, all metrics, multi-threaded scenarios, and larger scales — even where benchmarks focus narrowly.

- Embeddings are dense floating-point vectors (not sparse) — *Q1 may survey sparse/hybrid approaches for completeness*
- Dimensionality range of interest: 128d to 1536d, **with 3072d as stretch target** (frontier embedding models are pushing wider) *(expanded Run 2)*
- Dataset sizes of interest: 10K to 10M vectors for benchmarks — *Q1/Q4 should discuss behavior at 100M+ from cited literature*
- Primary distance metrics of interest: cosine, dot product, L2 — *Q1.b maps all metrics broadly*
- Persistence is desirable but in-memory-only is acceptable for some use cases
- Single-threaded query latency matters more than multi-tenant throughput for initial evaluation
- **Filtered search is a first-class evaluation dimension**, not an optional nice-to-have *(added Run 2 — all production use cases need it)*

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Adjacent initiative specs not yet drafted — boundaries may conflict | Medium | Medium | Validate consistency when research-cfj.1 and research-anj.1 specs are drafted |
| Benchmark results hardware-dependent | High | Medium | Document hardware, cite ann-benchmarks.com for cross-platform data |
| Reference implementation study drifts into product evaluation | Medium | Low | Apply study rubric (architecture only, not operations/pricing) |

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-24 | Initial draft | aRustyDev + Claude |
| 2026-04-24 | Added risks, reference impl rubric, clarified broad-vs-narrow assumptions per review | Claude |
| 2026-04-24 | **Run 2 scope review**: Added modern quantization (RaBitQ, TurboQuant), filtered ANN, dimensionality-dependent behavior to Algorithms & Theory. Added 4 evaluation dimensions (filtered search, incremental updates, persistence model, platform portability). Expanded dimensionality assumption to 3072d stretch. Added maintenance risk and filtered search assumption. Rationale: Run 2 gap-fill discovered these as significant dimensions not covered by original scope — see Run 2 report for evidence. | Claude |
