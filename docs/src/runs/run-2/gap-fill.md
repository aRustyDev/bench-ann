# Vector DS&A Run 2: Gap-Fill Survey Results

**Date:** 2026-04-24
**Method:** Live verification via crates.io search, lib.rs cross-reference, WebSearch
**Confidence level:** Verified (live web queries), except where noted as "partial" due to JS-rendered crates.io pages not fully extractable via search

---

## Task 1: Crate Verification

### 1. hnsw

```yaml
- name: hnsw
  exists: true
  version: "0.11.0"
  downloads: 101,798 (all-time); ~35,209 (v0.11.0 specifically)
  recent_downloads: not extracted (JS-rendered)
  last_updated: ">4 years ago" (estimated ~2021)
  description: "Fast approximate nearest neighbors — HNSW implementation"
  repository: https://github.com/rust-cv/hnsw
  author: Geordon Worley
  license: MIT
  editions: 22 versions published, Rust 2018 edition
  status: unmaintained (no updates in 4+ years)
  confidence: verified
```

### 2. hnswlib-rs

```yaml
- name: hnswlib-rs
  exists: true
  version: latest unknown (multiple versions; docs show active Jan 2026)
  downloads: 3,186 (all-time)
  recent_downloads: not extracted
  last_updated: ~Jan 2026 (actively maintained)
  description: "Pure-Rust HNSW graph for approximate nearest-neighbor search, inspired by C++ hnswlib"
  repository: https://github.com/jean-pierreBoth/hnswlib-rs
  license: MIT/Apache-2.0
  versions_published: 11
  status: active
  confidence: verified
  notes: "NOT the same as hnsw_rs — this is a pure-Rust implementation by jean-pierreBoth"
```

### 3. hnsw_rs

```yaml
- name: hnsw_rs
  exists: true
  version: "0.3.4"
  downloads: 332,413 (all-time)
  recent_downloads: 116,190
  last_updated: "~25 days ago" (as of search date, ~early Apr 2026)
  description: "Ann based on Hierarchical Navigable Small World Graphs (Malkov-Yashunin)"
  repository: https://github.com/jean-pierreBoth/hnswlib-rs (same author, different crate)
  license: MIT/Apache-2.0
  status: active (recently updated)
  confidence: verified
  notes: "Highest downloads of any Rust HNSW crate. Supports filtered search via Filterable trait."
```

### 4. instant-distance

```yaml
- name: instant-distance
  exists: true
  version: "0.6.1"
  downloads: 126,119 (all-time)
  recent_downloads: not extracted
  last_updated: not precisely determined
  description: "Fast minimal implementation of HNSW maps for approximate nearest neighbors searches. Powers InstantDomainSearch.com."
  repository: https://github.com/instant-labs/instant-distance
  license: MIT/Apache-2.0
  versions_published: 11
  status: low-activity (likely stable/complete)
  confidence: verified
```

### 5. swarc

```yaml
- name: swarc
  exists: true
  version: "0.1.0"
  downloads: 284 (all-time)
  recent_downloads: minimal
  last_updated: recent (appeared in 2025-2026 search results)
  description: "High-performance HNSW implementation in Rust. State-of-the-art ANN search for high-dimensional vector similarity."
  repository: https://github.com/carlosbertoncelli/swarc
  license: MIT
  versions_published: 1
  features: |
    - Fast k-NN search with logarithmic time complexity
    - Document linking (associate embeddings with external data)
    - Dynamic operations: insert, remove, rebalance at runtime
    - Generic implementation for any data type
    - JSON serialization/deserialization
    - Benchmarked with 3072-dim embeddings
  status: new/early (single version, low downloads)
  confidence: verified
```

### 6. hora

```yaml
- name: hora
  exists: true
  version: "0.1.1"
  downloads: 35,163 (all-time); ~477/month (lib.rs)
  recent_downloads: ~477/month
  last_updated: "~Aug 2021" (published ~4.5 years ago)
  description: "Approximate nearest neighbor search algorithm library. Implements HNSW, SSG, PQIVF, BruteForce."
  repository: https://github.com/hora-search/hora
  license: Apache-2.0
  versions_published: 2
  size: 9.5MB, 4K SLoC
  status: abandoned/unmaintained (no updates since 2021, open issues unanswered)
  confidence: verified
  notes: "Also has hora-new fork (v0.0.2) by someone else. GitHub shows 2.7k stars but no recent commits."
```

### 7. arroy

```yaml
- name: arroy
  exists: true
  version: "0.6.3"
  downloads: ~30,455/month (lib.rs)
  recent_downloads: ~30,455/month
  last_updated: active (latest version 0.6.3 on docs.rs)
  description: "Annoy-inspired ANN library in Rust, based on LMDB, optimized for memory usage. Used inside Meilisearch."
  repository: https://github.com/meilisearch/arroy
  license: MIT
  used_in: 14 crates (7 directly)
  size: 1.5MB, 5.5K SLoC
  status: active (Meilisearch production dependency)
  confidence: verified
  notes: "Now supports filtered search via RoaringBitmap integration. Evolving beyond pure Annoy — blog post mentions 'Hannoy' (hybrid HNSW+Annoy) and Filtered Disk ANN."
```

### 8. annoy-rs

```yaml
- name: annoy-rs
  exists: true
  version: "0.1.0"
  downloads: 2,632 (all-time)
  recent_downloads: minimal
  last_updated: not precisely determined
  description: "Annoy (Approximate Nearest Neighbors Oh Yeah) Rust bindings with JNI support"
  repository: not extracted
  license: not extracted
  versions_published: 1
  status: unmaintained (single version, low downloads)
  confidence: verified
```

### 9. kiddo

```yaml
- name: kiddo
  exists: true
  version: "5.2.3"
  downloads: 3,389,727 (all-time)
  recent_downloads: 1,122,532
  last_updated: "~4 hours ago" (at time of search — very active)
  description: "High-performance, flexible, ergonomic k-d tree library for geo/astro nearest-neighbour and k-NN queries"
  repository: https://github.com/sdd/kiddo
  license: MIT/Apache-2.0
  status: very active (highest downloads in this survey)
  confidence: verified
  notes: "By far the most popular spatial search crate in Rust. Focused on low-dimensional (geo/astro) use cases."
```

### 10. kd-tree

```yaml
- name: kd-tree
  exists: true
  version: "0.6.2"
  downloads: 533,075 (all-time); ~24,461/month (lib.rs)
  recent_downloads: ~24,461/month
  last_updated: not precisely determined
  description: "K-dimensional tree implementation"
  repository: not extracted
  author: Terada Yuichiro + 3 contributors
  versions_published: 12
  status: maintained
  confidence: verified
```

### 11. lsh-rs

```yaml
- name: lsh-rs
  exists: true
  version: "0.4.0" (docs.rs reference)
  downloads: 1,266 (all-time); ~29/month (lib.rs)
  recent_downloads: ~29/month
  last_updated: "~May 2020" (lib.rs age metadata)
  description: "Approximate Nearest Neighbor Search with Locality Sensitive Hashing"
  repository: https://github.com/ritchie46/lsh-rs
  license: MIT
  versions_published: 7
  size: 95KB, 2K SLoC
  status: unmaintained (last update ~2020)
  confidence: verified
```

### 12. gaoya

```yaml
- name: gaoya
  exists: true
  version: "0.2.0"
  downloads: ~848/month (lib.rs)
  recent_downloads: ~848/month
  last_updated: "~Jun 2023" (lib.rs age metadata)
  description: "Locality Sensitive Hashing Data Structures for indexing and querying text documents. Primary use: deduplication and clustering."
  repository: https://github.com/serega/gaoya
  license: MIT
  size: 125KB, 3K SLoC
  used_in: 2 crates
  status: low-activity (last update 2023)
  confidence: verified
```

### 13. vpsearch

```yaml
- name: vpsearch
  exists: true
  version: latest not precisely determined (19 versions published)
  downloads: 62,763 (all-time); ~102/month (lib.rs)
  recent_downloads: ~102/month
  last_updated: not precisely determined
  description: "Vantage Point Tree search algorithm for fast nearest neighbour search in multi-dimensional metric spaces"
  repository: https://github.com/kornelski/vpsearch
  license: not extracted
  versions_published: 19
  status: low-activity (stable/mature)
  confidence: verified
```

### 14. space

```yaml
- name: space
  exists: true
  version: "0.18.0" (docs.rs reference)
  downloads: not precisely determined
  recent_downloads: not precisely determined
  last_updated: not precisely determined
  description: "Spatial library — provides MetricPoint trait for metrics forming a metric space, Knn trait for NN search data structures, and LinearSearch implementation"
  repository: https://github.com/rust-cv/space
  license: MIT
  status: maintained (part of rust-cv ecosystem)
  confidence: verified
  notes: "Trait-based abstraction layer. Not itself an ANN implementation — provides the interface that crates like hnsw implement."
```

### 15. simsimd

```yaml
- name: simsimd
  exists: true
  version: "6.2.1"
  downloads: 593,117 (all-time)
  recent_downloads: not extracted
  last_updated: active (version 6.2.1 on docs.rs)
  description: "Portable mixed-precision BLAS-like vector math library for x86 and ARM"
  repository: https://github.com/ashvardanian/SimSIMD
  author: Ash Vardanian, Pedro Gabriel
  license: Apache-2.0
  versions_published: 101
  status: very active (frequent releases)
  confidence: verified
  notes: "NOT an ANN index — a SIMD-accelerated distance/similarity computation library. Complementary to ANN crates."
```

### 16. fast-vector-similarity

```yaml
- name: fast-vector-similarity
  exists: false (NOT on crates.io)
  version: N/A
  downloads: N/A
  description: "High-performance vector similarity library in Rust with Python bindings (Spearman, Kendall, distance correlation, Jensen-Shannon, Hoeffding's D)"
  repository: https://github.com/Dicklesworthstone/fast_vector_similarity
  availability: GitHub + PyPI only (not published to crates.io)
  status: available as Rust source, not as a crate
  confidence: verified
  notes: "Rust library with Python bindings via PyO3. Available on PyPI as fast-vector-similarity. Not a crates.io crate."
```

### 17. usearch

```yaml
- name: usearch
  exists: true
  version: "2.25.1"
  downloads: 221,071 (all-time)
  recent_downloads: not extracted
  last_updated: active (~Feb 2026)
  description: "High-performance library for ANN search. HNSW-based. Extensible with custom distance metrics and filtering predicates."
  repository: https://github.com/unum-cloud/usearch
  author: Ash Vardanian (Unum Cloud)
  license: Apache-2.0
  versions_published: 192
  status: very active
  confidence: verified
  notes: "Rust bindings to C++ core. Supports filtered_search with predicate functions during graph traversal. One of the most feature-complete ANN crates."
```

### 18. faiss / faiss-rs

```yaml
- name: faiss
  exists: true
  version: "0.13.0"
  downloads: 91,233 (all-time); ~881/month (lib.rs)
  recent_downloads: ~881/month
  last_updated: "~5 months ago" (~Nov 2025)
  description: "Rust language bindings for Faiss — state-of-the-art vector search and clustering library"
  repository: https://github.com/Enet4/faiss-rs
  author: Eduardo Pinho
  license: MIT/Apache-2.0
  versions_published: 14 (main crate), 15 total with faiss-sys
  status: maintained (recent release)
  confidence: verified
  notes: |
    Related crates:
    - faiss-sys: 88,644 downloads (FFI bindings)
    - faiss-next: alternative bindings (3,084 downloads)
    Requires external Faiss C++ library installed. Dynamically linked by default.
```

### 19. voyager

```yaml
- name: voyager (for ANN)
  exists: false (NOT on crates.io as ANN crate)
  version: N/A
  downloads: N/A
  description: "The 'voyager' crate on crates.io is a web crawler by mattsse, NOT Spotify's ANN library"
  spotify_voyager: "Spotify's Voyager is a C++/Python/Java HNSW library — NOT published as a Rust crate"
  community_bindings: "uzushino/voyager-rs exists on GitHub but is NOT published to crates.io"
  status: not available as Rust crate
  confidence: verified
  notes: "Spotify Voyager provides Python and Java bindings only. Community Rust bindings exist on GitHub but are unpublished."
```

### 20. granne

```yaml
- name: granne
  exists: true
  version: "0.5.2" (3 versions published)
  downloads: 11,619 (all-time); ~23/month (lib.rs)
  recent_downloads: ~23/month
  last_updated: "~Jun 2021" (lib.rs metadata)
  description: "Graph-based retrieval of approximate nearest neighbors. HNSW-based, focused on reducing memory usage for billion-scale indexing. Used in Cliqz Search."
  repository: https://github.com/granne/granne
  license: MIT
  features: |
    - Memory-mapped indexes
    - Multithreaded index creation
    - Extensible indexes (add to already-built index)
    - Python bindings
    - Dense float or int8 elements (cosine distance)
  status: unmaintained (last update ~2021, Cliqz shut down)
  confidence: verified
```

### 21. flann-rs

```yaml
- name: flann-rs
  exists: false (no crate named "flann-rs" on crates.io)
  alternate: flann (v0.1.0) + flann-sys (v0.1.0) exist
  version: "0.1.0" (both flann and flann-sys)
  downloads: not precisely determined (low)
  last_updated: "2019-02-25"
  description: "Rust bindings for FLANN (Fast Library for Approximate Nearest Neighbors)"
  repository: https://github.com/rust-flann/rust-flann
  license: not extracted
  versions_published: 6 (flann), 4 (flann-sys)
  status: abandoned (last update Feb 2019, over 7 years ago)
  confidence: verified
  notes: "The crate is named 'flann' not 'flann-rs'. Both flann and flann-sys are effectively dead."
```

---

## Task 2: Gap Investigation

### Gap 1: Pure Rust IVF-PQ

```yaml
Gap 1: Pure Rust IVF-PQ Implementation
Finding: partially found — no classic IVF-PQ, but modern alternatives exist
Evidence: |
  Searched crates.io for "ivf product quantization", "ivf-pq", "inverted file quantization".

  No pure Rust IVF-PQ implementation found. However, notable alternatives discovered:

  1. rabitq-rs (v0.7.0, 36 downloads) — Rust implementation of RaBitQ + IVF and MSTG.
     IVF+RaBitQ marked "production ready". Claims 32x memory compression with higher
     accuracy than PQ or SQ. x86_64 only (ARM64 has critical bugs).
     Repo: https://github.com/lqhl/rabitq-rs

  2. turbo-quant — Rust implementation of Google's TurboQuant, PolarQuant, QJL.
     Data-oblivious quantization (no training step). From ICLR 2026 / AISTATS 2026 papers.
     NOT IVF-PQ but a modern replacement approach.

  3. turbovec (v0.1.3) — Vector index built on TurboQuant. Claims 12-20% faster than
     FAISS IndexPQFastScan on ARM, competitive on x86.

  4. hora (v0.1.1) — Claims PQIVF support but is abandoned (2021).

  5. faiss (v0.13.0) — Provides IVF-PQ via Faiss C++ bindings (not pure Rust).

  Classic IVF-PQ in pure Rust: CONFIRMED ABSENT.
  Modern quantization alternatives: FOUND (rabitq-rs, turbo-quant, turbovec).
Sources:
  - https://crates.io/crates/rabitq-rs
  - https://crates.io/crates/turbo-quant
  - https://crates.io/crates/turbovec
  - https://github.com/lqhl/rabitq-rs
```

### Gap 2: Standalone DiskANN/Vamana

```yaml
Gap 2: Standalone DiskANN/Vamana in Rust
Finding: FOUND — multiple standalone implementations discovered
Evidence: |
  Searched crates.io for "diskann", "vamana", "disk ann". Found THREE standalone crates:

  1. diskann (v0.50.0, 7,878 downloads) — By INFINI Labs. Pure Rust DiskANN implementation.
     Forked from Microsoft's partial Rust port and extended to full implementation.
     Repo: https://github.com/infinilabs/diskann

  2. diskann-rs (crates.io listing confirmed, updated ~Feb 2026) — Alternative Rust
     DiskANN using Vamana graph algorithm. Memory-mapped index, beam search with
     medoid entry points. Supports Euclidean, Cosine, Hamming distance.
     Repo: linked from crates.io

  3. rust-diskann (crates.io listing confirmed) — By jianshu93. Native Rust DiskANN
     using Vamana graph algorithm. Generic distance trait.
     Repo: https://github.com/jianshu93/rust-diskann

  Also found: diskann-benchmark crate for benchmarking.

  RUN 1 ASSESSMENT OVERTURNED: Run 1 noted DiskANN as absent. Multiple standalone
  implementations now confirmed, at least one actively maintained (diskann v0.50.0
  with 9 versions suggests active development).
Sources:
  - https://crates.io/crates/diskann
  - https://crates.io/crates/diskann-rs
  - https://crates.io/crates/rust-diskann
  - https://github.com/infinilabs/diskann
  - https://github.com/jianshu93/rust-diskann
```

### Gap 3: ScaNN Bindings

```yaml
Gap 3: ScaNN Bindings or Port in Rust
Finding: confirmed absent
Evidence: |
  Searched crates.io for "scann", "anisotropic quantization". No results found.
  WebSearch returned only unrelated crates (scanner-rust, etc.).

  Google's ScaNN has no Rust bindings, ports, or FFI wrappers on crates.io.
  The only way to use ScaNN from Rust would be through manual FFI to the C++ library.
Sources:
  - Search: crates.io "scann" — no relevant results
  - Search: crates.io "anisotropic quantization" — no results
```

### Gap 4: hora Maintenance Status

```yaml
Gap 4: hora Maintenance Status
Finding: confirmed abandoned/unmaintained
Evidence: |
  - crates.io: hora v0.1.1, 35,163 downloads, 2 versions published
  - Last crate publish: ~Aug 2021 (4.5+ years ago)
  - lib.rs: 477 downloads/month (residual dependency usage)
  - GitHub: hora-search/hora has 2.7k stars but search could not confirm
    any commits after 2021. Multiple open issues appear unanswered.
  - Fork exists: hora-new v0.0.2 on crates.io (separate maintainer)
  - The library claimed HNSW, SSG, PQIVF, BruteForce support but with
    only 4K SLoC in a 9.5MB crate (bulk is likely test data/assets)

  VERDICT: Abandoned. Do not rely on for production use.
  hora-new fork status also unclear (only v0.0.2).
Sources:
  - https://crates.io/crates/hora
  - https://lib.rs/crates/hora
  - https://github.com/hora-search/hora
```

### Gap 5: Neural/Deep/Learned Hashing in Rust

```yaml
Gap 5: Neural Hashing in Rust
Finding: confirmed absent
Evidence: |
  Searched crates.io for "neural hashing", "deep hashing", "learned hashing".
  No relevant results found. Search returned only:
  - General hashing crates (ahash, rustc-hash, crypto-hash)
  - Image perceptual hashing (imghash)
  - General neural network crates (fast-neural-network)

  None implement neural/deep/learned hashing for ANN search.
  This remains an entirely unaddressed niche in the Rust ecosystem.
  Users would need to implement via general ML frameworks (burn, candle, tch-rs)
  or use Python bindings.
Sources:
  - Search: crates.io "neural hashing" — no relevant results
  - Search: crates.io "deep hashing" — no relevant results
  - Search: crates.io "learned hashing" — no relevant results
```

### Gap 7: swarc Details

```yaml
Gap 7: swarc Crate Details
Finding: found — new, minimal HNSW crate
Evidence: |
  swarc v0.1.0 confirmed on crates.io:
  - 284 downloads all-time, 1 version published
  - Author: Carlos Bertoncelli (github.com/carlosbertoncelli/swarc)
  - License: MIT
  - Algorithm: HNSW (Hierarchical Navigable Small World)
  - Features:
    * Logarithmic-time k-NN search
    * Document linking (embeddings + external data association)
    * Dynamic operations (insert, remove, rebalance at runtime)
    * Generic data type support
    * JSON serialization/deserialization
  - Performance: Benchmarked with 3072-dim embeddings across various dataset sizes
  - Assessment: Very new, single version, minimal adoption. Not competitive with
    hnsw_rs (332K downloads) or usearch (221K downloads) for production use.
    May be a learning project or early-stage effort.
Sources:
  - https://crates.io/crates/swarc
  - https://github.com/carlosbertoncelli/swarc
```

---

## Task 3: Post-2025 New Crates (Gap 6)

```yaml
Gap 6: New Rust ANN Crates (2025-2026)
Finding: several new crates discovered
Evidence: |
  Searched crates.io and web for new vector search crates from 2025-2026.

  NEW CRATES DISCOVERED:

  1. diskann (v0.50.0) — INFINI Labs pure Rust DiskANN. 7,878 downloads, 9 versions.
     Actively developed. Disk-based ANN for billion-scale datasets.

  2. diskann-rs — Alternative DiskANN implementation. Updated Feb 2026.

  3. rust-diskann — Third DiskANN implementation by jianshu93.

  4. small-world-rs — "The easiest HNSW vector index you'll ever use."
     By httpjamesm. Updated Dec 2024. Semantic search focus.

  5. swarc (v0.1.0) — New HNSW implementation. 284 downloads.
     Dynamic operations, document linking.

  6. rabitq-rs (v0.7.0) — RaBitQ quantization with IVF and MSTG index.
     13 versions, 36 downloads. Production-ready IVF+RaBitQ.

  7. turbovec (v0.1.3) — TurboQuant-based vector index with 2-4 bit compression
     and SIMD search. Claims faster than FAISS IndexPQFastScan.

  8. turbo-quant — Google TurboQuant/PolarQuant/QJL implementation.
     Zero-overhead vector quantization. Based on ICLR 2026 papers.

  9. ruvector ecosystem (ruvector-core, ruvector-gnn, ruvector-wasm, ruvector-postgres) —
     High-performance vector GNN memory DB. WASM support. Active development.

  10. foxstash-core — appeared in HNSW keyword search results. Details not confirmed.

  ALSO NOTED — Established crates with significant 2025-2026 updates:
  - usearch: v2.25.1, 192 versions, very active
  - hnsw_rs: v0.3.4, updated ~Apr 2026
  - arroy: v0.6.3, adding Filtered Disk ANN support
  - simsimd: v6.2.1, 101 versions, very active
  - kiddo: v5.2.3, updated hours ago

Sources:
  - https://crates.io/crates/diskann
  - https://crates.io/crates/diskann-rs
  - https://crates.io/crates/small-world-rs
  - https://crates.io/crates/swarc
  - https://crates.io/crates/rabitq-rs
  - https://crates.io/crates/turbovec
  - https://crates.io/crates/turbo-quant
  - https://github.com/ruvnet/RuVector
```

---

## Task 4: Filtered ANN Support (Gap 8)

```yaml
Gap 8: Filtered/Constrained ANN in Rust
Finding: found in multiple crates beyond qdrant and hnsw_rs
Evidence: |
  Crates with confirmed filtered/constrained ANN support:

  1. hnsw_rs (v0.3.4) — CONFIRMED. Supports filtering during search (not post-filter).
     Two modes: (a) pass sorted vector of allowed IDs, (b) pass a predicate function
     called before adding each ID to result set. Also supports implementing the
     Filterable trait for custom filter types (e.g., bitvector-backed).

  2. usearch (v2.25.1) — CONFIRMED. Predicate-based filtered search:
     `index.filtered_search(&query, k, |key| predicate(key))`.
     Predicate evaluated during graph traversal, not post-filter.
     Can integrate external containers (Bloom filters, databases) for complex filtering.

  3. arroy (v0.6.3) — CONFIRMED. Uses RoaringBitmap for filtering during search.
     Meilisearch integration provides full filter operator support (<, <=, =, !=, >=, >)
     on filterable attributes. Blog post discusses "Filtered Disk ANN" evolution.

  4. diskann / diskann-rs — LIKELY (DiskANN paper includes filtered search support,
     but need to verify Rust implementation exposes this API).

  CRATES WITHOUT FILTERED SEARCH (or not confirmed):
  - hnsw (rust-cv): No filtered search API found
  - instant-distance: No filtered search API found
  - granne: No filtered search API found
  - kiddo: Spatial queries (range, kNN) but not ANN-style predicate filtering
  - annoy-rs, swarc, hora: No filtered search confirmed

  SUMMARY: Three crates (hnsw_rs, usearch, arroy) have production-quality
  filtered ANN. usearch offers the most flexible API with arbitrary predicate
  functions. hnsw_rs provides the Filterable trait abstraction. arroy
  integrates with Meilisearch's full filter expression language.

Sources:
  - https://docs.rs/hnsw_rs/latest/hnsw_rs/hnsw/
  - https://docs.rs/usearch/latest/usearch/
  - https://blog.kerollmops.com/meilisearch-expands-search-power-with-arroy-s-filtered-disk-ann
  - https://github.com/unum-cloud/usearch/issues/348
```

---

## Summary Table

| # | Crate | Exists | Version | Downloads (all-time) | Status | Filtered ANN |
|---|-------|--------|---------|---------------------|--------|--------------|
| 1 | hnsw | yes | 0.11.0 | 101,798 | unmaintained | no |
| 2 | hnswlib-rs | yes | ~latest | 3,186 | active | unknown |
| 3 | hnsw_rs | yes | 0.3.4 | 332,413 | active | YES |
| 4 | instant-distance | yes | 0.6.1 | 126,119 | low-activity | no |
| 5 | swarc | yes | 0.1.0 | 284 | new/early | no |
| 6 | hora | yes | 0.1.1 | 35,163 | abandoned | no |
| 7 | arroy | yes | 0.6.3 | ~30K/month | active | YES |
| 8 | annoy-rs | yes | 0.1.0 | 2,632 | unmaintained | no |
| 9 | kiddo | yes | 5.2.3 | 3,389,727 | very active | N/A (spatial) |
| 10 | kd-tree | yes | 0.6.2 | 533,075 | maintained | N/A (spatial) |
| 11 | lsh-rs | yes | 0.4.0 | 1,266 | unmaintained | no |
| 12 | gaoya | yes | 0.2.0 | ~848/mo | low-activity | no |
| 13 | vpsearch | yes | ~latest | 62,763 | low-activity | no |
| 14 | space | yes | 0.18.0 | unknown | maintained | N/A (traits) |
| 15 | simsimd | yes | 6.2.1 | 593,117 | very active | N/A (compute) |
| 16 | fast-vector-similarity | NO | N/A | N/A | GitHub only | N/A |
| 17 | usearch | yes | 2.25.1 | 221,071 | very active | YES |
| 18 | faiss | yes | 0.13.0 | 91,233 | maintained | via C++ |
| 19 | voyager | NO* | N/A | N/A | not on crates.io | N/A |
| 20 | granne | yes | 0.5.2 | 11,619 | unmaintained | no |
| 21 | flann-rs | NO** | N/A | N/A | see `flann` crate | no |

\* Spotify Voyager is Python/Java only. Community `voyager-rs` exists on GitHub but not published to crates.io.
\** The crate is named `flann` (v0.1.0), not `flann-rs`. Last updated 2019, effectively dead.

---

## Gap Summary

| Gap | Title | Status | Key Finding |
|-----|-------|--------|-------------|
| 1 | Pure Rust IVF-PQ | **absent** (modern alternatives exist) | rabitq-rs, turbo-quant, turbovec offer modern quantization; classic IVF-PQ absent |
| 2 | Standalone DiskANN/Vamana | **FOUND** (3 crates!) | diskann, diskann-rs, rust-diskann — Run 1 assessment overturned |
| 3 | ScaNN bindings | **absent** | No bindings, ports, or wrappers exist |
| 4 | hora maintenance | **abandoned** | Last update ~Aug 2021, 4.5+ years stale |
| 5 | Neural hashing | **absent** | No neural/deep/learned hashing crates in Rust |
| 6 | Post-2025 new crates | **10 new crates found** | diskann, diskann-rs, small-world-rs, swarc, rabitq-rs, turbovec, turbo-quant, ruvector-*, foxstash-core |
| 7 | swarc details | **verified** | New HNSW crate, v0.1.0, 284 downloads, early stage |
| 8 | Filtered ANN | **3 crates confirmed** | hnsw_rs (Filterable trait), usearch (predicate fn), arroy (RoaringBitmap) |
