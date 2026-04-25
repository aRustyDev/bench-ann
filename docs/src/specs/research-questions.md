# Research Questions & Objectives: Vector DS&A

> **Bead**: `research-cfj.2.13.1`
> **Status**: draft
> **Date**: 2026-04-24
> **Revised**: 2026-04-24 (reframed Q4-Q6 to be unbiased; added Q1 sub-questions; added seed references)

## Research Principles

This research is conducted **in a vacuum** — it produces grounded, reusable reference material, not project-specific decisions. The outputs are evidence matrices, trade-off profiles, and comparison frameworks that any project can consult to make their own decisions. Project-specific questions ("what should Forge use?") are answered downstream by consuming these research outputs.

## Primary Research Questions

### Q1: What ANN algorithm families exist and what are their trade-off profiles?

**Why**: Before evaluating any crate, we need to understand the algorithm landscape. Different algorithm families make fundamentally different trade-offs (build time vs query time vs memory vs recall). Without this map, we can't categorize crates or make informed choices.

**What "answered" looks like**: A taxonomy of algorithm families (HNSW, IVF, PQ, DiskANN, random projection trees, LSH, VP-trees, etc.) with a trade-off profile for each (one paragraph + key parameters).

#### Q1.a: When does the kNN ↔ ANN trade-off occur?

At what scale, hardware profile, and scenario does exact kNN become impractical and ANN becomes necessary? Document the crossover points: dataset size thresholds, dimensionality thresholds, latency budgets, hardware constraints (CPU vs GPU, RAM limits). Produce a decision framework for when to use exact vs approximate.

#### Q1.b: What key similarity/distance metrics exist and what are their trade-off profiles?

Map the metric landscape: cosine similarity, Euclidean (L2), dot product, Manhattan (L1), Hamming, Jaccard, angular distance. For each: mathematical definition, when to use it, computational cost, relationship to embedding model training objective (e.g., cosine for normalized embeddings, dot product for unnormalized). Which metrics does each algorithm family support natively?

#### Q1.c: How does ScaNN relate to and compare with other ANN algorithms?

ScaNN (Scalable Nearest Neighbors) by Google is both an algorithm and a system. Understand: its relationship to quantization-based approaches (learned quantization, anisotropic quantization), how SOAR improves on it, its position in the algorithm taxonomy vs HNSW/IVF/PQ, and when it's the right choice vs alternatives.

**Seed references**:
- [SOAR: New algorithms for even faster vector search with ScaNN](https://research.google/blog/soar-new-algorithms-for-even-faster-vector-search-with-scann/)
- [Vertex AI Vector Search overview](https://docs.cloud.google.com/vertex-ai/docs/vector-search/overview)

#### Q1.d: What vector indexing techniques exist and what are their trade-off profiles?

Beyond algorithm families, map the indexing layer: flat (brute-force), inverted file (IVF), hierarchical navigable small world (HNSW), Vamana (DiskANN), tree-based (KD-tree, ball tree, random projection), hash-based (LSH, multi-probe LSH), graph-based (relative neighborhood graph, NSG), quantization as indexing (PQ, OPQ, ScaNN). How do these compose (e.g., IVF+PQ, HNSW+PQ)?

#### Q1.e: What is Neural Hashing, and how does it relate to ANN algorithms?

Understand: learning-to-hash approaches, how neural networks can learn hash functions for approximate search, relationship to LSH (handcrafted vs learned), deep hashing methods, and whether any Rust implementations exist.

### Q2: Which Rust crates implement ANN algorithms, and what is their quality?

**Why**: The Rust ecosystem for vector search is fragmented. Some crates are wrappers around C/C++ libraries (hnswlib-rs), some are pure Rust (Arroy), some are minimal (Fast Vector Similarity). We need a complete map before narrowing.

**What "answered" looks like**: A matrix of every relevant Rust crate, categorized by algorithm family, with maturity signals (maintenance status, downloads, contributor count, last release).

### Q3: What are the performance/accuracy/memory trade-offs between implementations?

**Why**: Two HNSW implementations can differ drastically in performance due to implementation details (SIMD, memory layout, graph construction heuristics). Algorithm family alone isn't sufficient — implementation quality matters.

**What "answered" looks like**: Benchmark data (measured, not inferred) comparing at least the top 3 candidates on: recall@10, QPS, build time, memory per vector, across multiple dimensionalities (128d, 384d, 768d, 1536d) and dataset sizes (10K, 100K, 1M).

### Q4: What models and metrics determine "best fit" for a given project?

**Why**: Different projects have different constraints (latency budgets, dataset sizes, persistence needs, concurrency models). Rather than answering "what do WE need", produce a decision framework that any project can use to determine its own best fit.

**What "answered" looks like**: A decision framework document with:
- Key dimensions to evaluate (dimensionality, dataset size, query latency, update frequency, persistence model, concurrency)
- How each dimension maps to algorithm family and implementation choices
- Reference profiles for common scenarios (small/local, medium/service, large/distributed)
- Threshold tables: "if your dataset is >X vectors at >Y dimensions with <Z ms latency budget, consider..."

### Q5: What are the architectural trade-off profiles for deployment models?

**Why**: The deployment model (in-process library vs embedded database vs external service, pure vector vs hybrid) is a fundamental architectural choice with cascading consequences. Rather than recommending one, map the trade-offs so the choice can be made per-project.

#### Q5.a: In-process library ↔ Embedded database ↔ External database

Document the trade-off profile across: operational complexity, latency, scalability ceiling, persistence model, failure modes, resource isolation, deployment flexibility. At what scale/complexity does each model start to struggle?

#### Q5.b: Pure vector database ↔ Hybrid graph+vector database

Document the trade-off profile across: query expressiveness (vector-only vs vector+graph traversal), operational simplicity (one engine vs two), performance (specialized vs generalized), feature coverage, maturity of hybrid implementations.

**Scope guard**: This documents the architectural trade-off profile. Specific hybrid database evaluation (CozoDB, SurrealDB, HelixDB) is deferred to research-anj.1.

### Q6: What is the landscape of implementations, and how do they compare?

**Why**: Produce a comprehensive, grounded survey and comparison that serves as the reference for any future adoption/fork/build decision. Not a recommendation — a reference.

#### Q6.a: Robust survey of relevant implementations

Map not only Rust crates but also notable implementations in other languages that serve as architectural references (FAISS, ScaNN, Annoy, hnswlib C++, Vamana/DiskANN, SPTAG). For each: what it does, how it's architected, what makes it notable, and whether Rust bindings or ports exist.

#### Q6.b: In-depth comparison of surveyed implementations

For each implementation, grounded (verified, not inferred) comparison across:
- **Implemented features**: algorithm support, distance metrics, quantization, persistence, filtering
- **Community & support**: contributor count, release cadence, issue response time, documentation quality
- **Maturity**: production usage, age, breaking change history, API stability
- **Complexity**: code size, dependency count, unsafe usage, build complexity
- **Gaps/overlaps/trade-offs/uniqueness**: what each does that others don't, where they overlap, what's missing

## Seed References

The following have been identified as starting points for R&E. Not curated — collection and curation happen during runs.

- [ANN Algorithm Selection Trade-offs (apxml.com course)](https://apxml.com/courses/advanced-vector-search-llms/chapter-1-ann-algorithms/ann-algorithm-selection-tradeoffs)
- [A Comprehensive Survey on Vector Database (arXiv 2204.07922)](https://arxiv.org/abs/2204.07922)
- [Billion-scale ANN Search with Modified HNSW (arXiv 1806.09823)](https://arxiv.org/abs/1806.09823)
- [Efficient and robust ANN using HNSW (arXiv 1603.09320)](https://arxiv.org/abs/1603.09320)
- [Range Search with Milvus (Zilliz)](https://zilliz.com/blog/unlock-advanced-recommendation-engines-with-milvus-new-range-search)
- [Recent survey on ANN algorithms (arXiv 2502.05575v1)](https://arxiv.org/html/2502.05575v1)
- [DiskANN and the Vamana Algorithm (Zilliz)](https://zilliz.com/learn/DiskANN-and-the-Vamana-Algorithm)
- [Relative neighborhood graph (Wikipedia)](https://en.wikipedia.org/wiki/Relative_neighborhood_graph)
- [SPTAG: Space Partition Tree And Graph (Microsoft)](https://github.com/microsoft/SPTAG)
- [SOAR: faster vector search with ScaNN (Google Research)](https://research.google/blog/soar-new-algorithms-for-even-faster-vector-search-with-scann/)
- [Vertex AI Vector Search overview (Google Cloud)](https://docs.cloud.google.com/vertex-ai/docs/vector-search/overview)

## Success Criteria

- [ ] Algorithm taxonomy (Q1) is stable, complete, and includes sub-questions (Q1.a-e)
- [ ] Similarity metric comparison (Q1.b) covers all common metrics with trade-off profiles
- [ ] Rust crate matrix (Q2) is comprehensive with maturity signals
- [ ] At least 3 implementations have been benchmarked with measured data (Q3)
- [ ] Decision framework (Q4) is produced — not a decision, but a framework for making one
- [ ] Deployment model trade-off profiles (Q5.a, Q5.b) are documented
- [ ] Implementation survey (Q6.a) covers Rust + reference implementations in other languages
- [ ] Implementation comparison (Q6.b) is grounded (verified features, not inferred)
- [ ] All findings are project-agnostic — reusable across any project

## Downstream Consumers (not part of this research)

These projects will consume these research outputs to make their own decisions:
1. **Forge**: embedding storage for skill matching, JD similarity, resume alignment
2. **AI/ML crate ecosystem** (research-ohb.1): provider abstraction for embeddings
3. **Architecture decisions**: in-process vs external, pure vs hybrid
4. **Cross-cutting**: whether graph DBs (HelixDB) can also serve as vector storage

## Execution Order

Questions have implicit dependencies:
1. **Q1** (taxonomy) → **Q2** (crate search — needs taxonomy to categorize)
2. **Q2** (crate matrix) → **Q3** (benchmarks — needs candidate list)
3. **Q1 + Q3** → **Q4** (decision framework — needs taxonomy + performance data)
4. **Q4** can proceed in parallel with **Q5** once Q1 is stable
5. **Q6** spans the full lifecycle — Q6.a (survey) starts early, Q6.b (comparison) completes last

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Rust ANN ecosystem too thin for meaningful top-3 comparison | Medium | High | Include non-Rust reference implementations; benchmark fewer candidates with deeper analysis |
| Benchmark results are hardware-dependent and may not generalize | High | Medium | Document hardware precisely; cite ann-benchmarks.com for cross-hardware data; state confidence levels |
| Q6.a reference impl study drifts into product evaluation | Medium | Low | Apply rubric: study architecture/algorithms only, not deployment/pricing/operations |
| Adjacent initiative specs (research-cfj.1, research-anj.1) not yet drafted — boundaries may conflict | Medium | Medium | Validate boundaries for consistency when those specs are drafted |
| DiskANN/ScaNN advantages only manifest at scales beyond our benchmark range | Low | Medium | Discuss large-scale behavior from cited literature even when not benchmarked first-party |

## Open Questions

1. **Benchmark methodology**: What hardware? Standard datasets (SIFT1M, GloVe) or synthetic? Do published third-party benchmarks (ann-benchmarks.com) qualify as "measured" or only first-party runs?
2. **Filtered ANN search**: Pre-filter vs post-filter vs in-filter approaches should be an explicit evaluation dimension — add to Q1 or Q6.b?
3. **Sparse vectors**: SPLADE, BM25-as-vectors, hybrid dense+sparse are increasingly relevant. Currently excluded by assumption. Revisit after R&E Run 1?
4. **Scale ceiling**: Q4 decision framework should discuss behavior at 100M+ vectors based on cited literature, even if not benchmarked first-party. How much depth?

## Non-Goals

- Evaluating embedding models (that's SentenceTransformers research in research-ohb.1.2.11)
- Evaluating full vector databases as products (that's research-cfj.1 — though Q6.a surveys them as reference implementations)
- Making project-specific adoption decisions (those happen downstream)
- Building anything (implementation comes after research)

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-24 | Initial draft (Q1-Q6, success criteria, downstream consumers) | aRustyDev + Claude |
| 2026-04-24 | Reframed Q4-Q6 to unbiased; added Q1.a-e sub-questions; added seed references | aRustyDev |
| 2026-04-24 | Added execution order, risks, open questions per code review feedback | Claude |
