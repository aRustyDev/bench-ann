# Scoping & Target Outcome Refinement — Run 2

> **Bead**: `research-cfj.2.14.2.6`
> **Date**: 2026-04-24

## Purpose

Narrow the candidate set based on Run 2 findings. Define which crates and algorithm families are worth deep investigation in Targeted Research, and which can be ruled out.

## Scoping Dimensions

These dimensions define what matters for the downstream evaluation. They are documented as trade-off axes, not project-specific decisions (per research principle: research produces reference material, not decisions).

### Distance Metrics

| Metric | Priority | Rationale |
|--------|----------|-----------|
| Cosine / Inner product | P0 | Dominant for NLP/LLM embeddings. Most libraries support via pre-normalization. |
| Euclidean (L2) | P0 | Standard. Universal support. |
| Dot product (MIPS) | P1 | Distinct from cosine on non-normalized vectors. Some libraries treat differently. |
| Hamming | P2 | Only relevant for binary quantization candidate filtering. |
| Custom metrics | P3 | Nice-to-have. VP-tree (vpsearch) and space crate support arbitrary metrics. |

### Dimensionality Range

| Range | Use Case | Algorithm Suitability |
|-------|----------|----------------------|
| <20d | Spatial/geo queries | KD-tree (kiddo) dominates. Not typical ANN territory. |
| 128d | Traditional CV features (SIFT) | All ANN algorithms work well. Standard benchmark dimension. |
| 384d | Small embedding models | Graph-based preferred. Tree-based start to degrade. |
| 768d | Medium embeddings (BERT) | Graph-based and quantization. LSH viable with many tables. |
| 1536d | Large embeddings (ada-002) | Graph-based. High-d where Hub Highway Hypothesis applies. |
| 3072d+ | Frontier models | Largely untested territory. RaBitQ/binary quantization gains importance. |

### Dataset Scale

| Scale | Vectors | In-scope algorithms |
|-------|---------|---------------------|
| Small | <100K | Any algorithm works. HNSW, brute force, KD-tree all viable. |
| Medium | 100K–10M | HNSW, RPT, IVF. Pure in-memory is fine. |
| Large | 10M–100M | HNSW with quantization, IVF-PQ. Memory optimization matters. |
| Billion | >100M | DiskANN, IVF-PQ, distributed systems. Single-node HNSW infeasible. |

### Deployment Model

| Model | Characteristics | Relevant crates |
|-------|----------------|-----------------|
| In-process library | Embedded in application. No network overhead. | hnsw_rs, instant-distance, kiddo, arroy |
| Persistent index | Index survives process restart. mmap or LMDB. | arroy, kiddo, diskann |
| Client-server | Separate search service. | qdrant, lancedb (out of pure-library scope) |

## Candidate Tiers

Based on Run 2 findings, crates fall into three tiers for Targeted Research:

### Tier 1: High Priority (active, significant adoption, unique capability)

| Crate | Algorithm | Why Tier 1 |
|-------|-----------|-----------|
| hnsw_rs | HNSW | Most downloaded pure-Rust HNSW (332K). Active. Filtered search. |
| kiddo | KD-tree | Highest downloads of any spatial crate (3.4M). Very active. Best for low-d. |
| usearch | HNSW (FFI) | Very active (192 versions). Feature-complete: filtered search, multi-precision. |
| arroy | RPT | Meilisearch production dependency. Active. Filtered search. LMDB persistence. |
| diskann | Vamana/DiskANN | Pure Rust DiskANN. Actively developed (v0.50.0, 9 versions). Fills major gap. |
| faiss | IVF/PQ/HNSW (FFI) | Most comprehensive algorithm coverage via FAISS C++. 91K downloads. |

### Tier 2: Worth Investigating (active or unique, moderate adoption)

| Crate | Algorithm | Why Tier 2 |
|-------|-----------|-----------|
| instant-distance | HNSW | Clean API, 126K downloads. May be complete/stable rather than abandoned. |
| kd-tree | KD-tree | 533K downloads, maintained. Simpler alternative to kiddo. |
| rabitq-rs | IVF+RaBitQ | Modern quantization with theoretical guarantees. Early stage but novel. |
| simsimd | Distance compute | Fastest SIMD distance library. Complementary to all index crates. |
| diskann-rs | Vamana/DiskANN | Alternative DiskANN impl. Need to compare against diskann. |
| vpsearch | VP-tree | Only pure-Rust VP-tree. 62K downloads. Useful for custom metrics. |
| turbovec | TurboQuant | Modern quantization approach. Very new but novel. |

### Tier 3: Low Priority (unmaintained, very new, or niche)

| Crate | Algorithm | Why Tier 3 |
|-------|-----------|-----------|
| hnswlib-rs | HNSW | Low downloads (3K). Same author as hnsw_rs — may be predecessor. |
| swarc | HNSW | Very new (v0.1.0, 284 downloads). Not yet competitive. |
| small-world-rs | HNSW | New. Insufficient data to evaluate. |
| gaoya | MinHash LSH | Near-duplicate detection niche. Low activity. |
| lsh-rs | LSH | Unmaintained since 2020. |
| granne | HNSW | Unmaintained since 2021 (Cliqz shut down). |
| hnsw | HNSW | Unmaintained for 4+ years. |
| ruvector | GNN | New ecosystem. Unclear algorithm. |

### Removed from Consideration

| Crate | Reason |
|-------|--------|
| hora | Abandoned since Aug 2021. Unreliable. |
| voyager | Not available as Rust crate. Python/Java only. |
| flann-rs / flann | Abandoned since 2019. FLANN itself is superseded. |
| fast-vector-similarity | Not on crates.io. GitHub/PyPI only. |
| annoy-rs | Single version, unmaintained. Superseded by arroy. |

## Algorithm Families: Scope Assessment

| Family | In Scope? | Notes |
|--------|-----------|-------|
| Navigable Small World (HNSW/NSW) | Yes (P0) | Most crates, best ecosystem support |
| MRNG-derived (NSG/SSG/Vamana) | Yes (P0) | DiskANN crates fill a major gap. NSG/SSG have no Rust crates. |
| Quantization (PQ/RaBitQ/TurboQuant/SQ) | Yes (P0) | Core technology, even when used as component in composites |
| Tree-based (RPT/KD-tree/VP-tree) | Yes (P1) | Strong Rust presence (kiddo, arroy, vpsearch) |
| Partition-based (IVF) | Yes (P1) | Only via FAISS FFI and rabitq-rs |
| Hash-based (LSH) | Partial (P2) | Weak Rust ecosystem. Theoretical interest only. |
| Neural hashing | No | Zero Rust implementations. Out of scope for crate evaluation. |

## Confirmed Absences (stable findings)

These are NOT gaps to fill — they represent genuine ecosystem absences that are unlikely to change soon:

1. **Pure Rust IVF-PQ**: Absent. Modern alternatives exist (RaBitQ, TurboQuant).
2. **ScaNN in Rust**: Absent. No bindings or ports. Google-only.
3. **Neural hashing in Rust**: Absent. No implementations. Research-only domain.
4. **NSG/SSG standalone Rust crate**: Absent. Only in hora (abandoned).
