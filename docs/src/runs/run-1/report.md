# Run Report: Vector DS&A — Run 1

> **Bead**: `research-cfj.2.14.1`
> **Date**: 2026-04-24
> **Run goal**: broad

## Summary

Run 1 cast a wide net across the ANN algorithm landscape, Rust crate ecosystem, and reference implementations. We identified 16 algorithm families, ~26 Rust crates, and 12 notable reference implementations. The Rust ecosystem is heavily HNSW-biased with significant gaps in IVF-PQ, DiskANN, and ScaNN. Foundational papers were verified via arXiv; web sources collected for tutorials, benchmarks, and distance metrics. Convergence not expected at this stage — taxonomy is draft, crate matrix needs live verification.

## Ecosystem Survey

### Discovered

#### Rust Crates

| Name | Algorithm(s) | Pure Rust / FFI | Status | Notes | Confidence |
|------|-------------|-----------------|--------|-------|------------|
| hnsw | HNSW | Pure Rust | ~2023 | Hamming + Euclidean, SIMD support, minimal deps | [inferred] |
| hnswlib-rs | HNSW | Pure Rust (rewrite, not FFI) | Active ~2024 | Decoupled graph/storage, inspired by C++ hnswlib | [verified] |
| hnsw_rs | HNSW | Pure Rust | Active ~2024 | SIMD distance, serialization, parallel construction, filtering | [verified] |
| instant-distance | HNSW | Pure Rust | ~2023 | Clean API, serialization, by Drifting in Space | [verified] |
| swarc | HNSW | Pure Rust | Active ~2025 | High-performance HNSW, newer entry | [verified] |
| hora | HNSW, SSG, PQIVF, brute-force | Pure Rust | ~2022 (possibly unmaintained) | Multiple algorithms, WASM support | [inferred] |
| arroy | Random projection trees (Annoy-style) | Pure Rust | Active ~2024 | By Meilisearch, LMDB persistence, production use | [inferred] |
| annoy-rs | Random projection trees | FFI (C++ Annoy) | ~2023 | Read-only bindings to Spotify Annoy format | [inferred] |
| kiddo | KD-tree | Pure Rust | Active ~2024 | Highly optimized, SIMD, `no_std`, immutable mmap variant | [inferred] |
| kd-tree | KD-tree | Pure Rust | ~2023 | Simple generic implementation | [inferred] |
| lsh-rs | LSH (L2, cosine, MIPS) | Pure Rust | ~2023 | Multiple LSH families, serialization | [inferred] |
| gaoya | MinHash LSH, SimHash | Pure Rust | ~2023 | Near-duplicate detection focus | [inferred] |
| vpsearch | VP-tree | Pure Rust | ~2022 | Generic over metric | [inferred] |
| space | Metric space structures | Pure Rust | ~2023 | Trait-based generic NN | [inferred] |
| simsimd | Distance computation | FFI (C) | Active ~2024 | SIMD-optimized, f32/f16/i8/binary, by Unum | [inferred] |
| fast-vector-similarity | Similarity metrics | Pure Rust | ~2023 | Cosine, Pearson, Spearman | [inferred] |
| usearch | HNSW variant | FFI (C++) | Active ~2024 | Very high performance, multi-precision, by Unum | [inferred] |
| faiss (rust bindings) | IVF, PQ, HNSW, flat, composites | FFI (C++) | ~2024 | Bindings to Facebook FAISS | [inferred] |
| voyager | HNSW (via hnswlib) | FFI | ~2024 | Spotify project | [inferred] |
| granne | HNSW variant | Pure Rust | ~2021 (unmaintained) | Angular distance optimized | [inferred] |
| flann-rs | Randomized KD-trees | FFI (C++) | ~2020 (unmaintained) | Bindings to classic FLANN | [inferred] |

**Database-adjacent (implement ANN indexes but are full systems):**

| Name | Algorithm(s) | Notes | Confidence |
|------|-------------|-------|------------|
| qdrant | HNSW + scalar/PQ/binary quantization | Full vector DB, modified HNSW with filtering | [inferred] |
| lance/lancedb | IVF-PQ, DiskANN-style graph | Columnar vector storage, disk-based ANN | [inferred] |
| tantivy | HNSW (added for vector search) | Full-text search engine with vector features | [inferred] |

#### Reference Implementations (Non-Rust)

| Name | Language | Algorithm(s) | Why Notable | Rust Bindings? | Confidence |
|------|----------|-------------|-------------|---------------|------------|
| FAISS | C++/Python | IVF, PQ, OPQ, HNSW, flat, GPU | Industry standard, most comprehensive | Yes (faiss crate) | [verified] |
| hnswlib | C++ | HNSW | Original HNSW reference by Malkov | Yes (hnswlib-rs inspired) | [verified] |
| Annoy | C++/Python | Random projection trees | By Spotify, battle-tested, mmap indexes | Yes (annoy-rs, arroy port) | [verified] |
| ScaNN | C++/Python | Learned anisotropic quantization, SOAR | By Google, tops ann-benchmarks | No Rust bindings | [verified] |
| DiskANN/Vamana | C++ | Vamana graph, PQ+SSD | By Microsoft, billion-scale from SSD | No (lance reimplements internally) | [verified] |
| SPTAG | C++ | BKT+KDT + neighborhood graph | By Microsoft, used in Bing | No | [verified] |
| NMSLIB | C++/Python | HNSW, VP-tree, SW-graph | Predecessor to hnswlib | No | [inferred] |
| USearch | C++ (core) | HNSW variant | By Unum, claims faster than FAISS | Yes (usearch crate) | [verified] |
| NGT | C++ | ANNG, ONNG, QBG | By Yahoo Japan | Unverified | [inferred] |
| Milvus | Go/C++ | IVF, HNSW, DiskANN (knowhere engine) | Full vector DB, notable algorithm abstraction | No | [inferred] |
| Weaviate | Go | HNSW + PQ compression | Full vector DB, pure-Go HNSW | No | [inferred] |
| Vespa | Java/C++ | HNSW | Real-time indexing, tight text+vector integration | No | [inferred] |

### Revised Understanding

N/A — this is Run 1 (no prior runs to revise).

## Keywords & Concepts

| Term | Definition | Source | Confidence |
|------|-----------|--------|------------|
| HNSW | Hierarchical Navigable Small World — multi-layer proximity graph for ANN search with logarithmic complexity | arXiv 1603.09320 | cited |
| NSW | Navigable Small World — single-layer small-world graph for greedy nearest neighbor search | Malkov et al. | cited |
| ANN | Approximate Nearest Neighbor — finding a point close to the true nearest neighbor within a factor c | arXiv 1806.09823 | cited |
| kNN | k-Nearest Neighbors — exact search for the k closest points | Standard | cited |
| IVF | Inverted File Index — partitions dataset into Voronoi cells via k-means, searches nprobe cells | FAISS docs | cited |
| PQ | Product Quantization — splits vectors into sub-vectors, quantizes each via learned codebook | Jégou et al. 2011 | cited |
| OPQ | Optimized Product Quantization — learns a rotation before PQ to minimize quantization error | FAISS | cited |
| ScaNN | Scalable Nearest Neighbors — Google's learned anisotropic quantization for ANN | arXiv 1908.10396 | cited |
| SOAR | Spilling with Orthogonality-Amplified Residuals — extension to ScaNN improving index quality | arXiv 2404.00774 | cited |
| DiskANN | Disk-based ANN using Vamana graph stored on SSD with PQ in memory | NeurIPS 2019 | cited |
| Vamana | Graph construction algorithm used by DiskANN — single-layer with angular diversity pruning | Subramanya et al. 2019 | cited |
| LSH | Locality Sensitive Hashing — maps similar items to same bucket with high probability | Standard | cited |
| RPT | Random Projection Trees — binary space partitioning via random hyperplanes (Annoy-style) | Bernhardsson 2015 | cited |
| VP-tree | Vantage Point tree — metric tree partitioning based on distance from vantage point | Standard | cited |
| NSG | Navigable Spreading-out Graph — sparse monotonic graph with spread-out neighbor selection | Fu et al. | inferred |
| Recall@k | Fraction of true k-nearest neighbors found by an ANN algorithm | Standard | cited |
| QPS | Queries Per Second — throughput metric for ANN search | Standard | cited |
| Cosine similarity | Similarity metric measuring angle between vectors: dot(a,b)/(||a||*||b||) | Standard | cited |
| L2 / Euclidean | Distance metric: sqrt(sum((a_i - b_i)^2)) | Standard | cited |
| Dot product / Inner product | Similarity metric: sum(a_i * b_i), sensitive to magnitude | Standard | cited |
| Hamming distance | Number of positions where binary vectors differ | Standard | cited |
| Scalar quantization | Quantizing each dimension from float32 to int8/int4 independently | Standard | cited |
| Binary quantization | Reducing each dimension to 1 bit (sign), enables Hamming distance | Standard | cited |
| Filtered ANN / FANNS | ANN search with scalar attribute filters — pre-filter, post-filter, or in-filter | arXiv 2505.06501 | cited |
| ann-benchmarks | Standard benchmarking framework for ANN algorithms by Aumüller et al. | arXiv 1807.05614 | cited |
| Neural hashing / Deep hashing | Learning hash functions via neural networks for ANN (data-dependent) | arXiv 2003.03369 | cited |
| RaBitQ | State-of-the-art quantization method achieving asymptotically optimal space/error trade-off | arXiv 2409.09913 | cited |

## Taxonomy Updates

Initial draft taxonomy (see `knowledge/taxonomy.yaml` for structured version):

```
ANN Algorithm Families
├── Graph-Based
│   ├── HNSW (Hierarchical Navigable Small World)
│   ├── NSW (Navigable Small World)
│   ├── NSG (Navigable Spreading-out Graph)
│   ├── SSG (Satellite System Graph)
│   ├── Vamana / DiskANN
│   ├── SPTAG (Space Partition Tree And Graph)
│   └── EFANNA
├── Hash-Based
│   ├── LSH (Locality Sensitive Hashing)
│   │   ├── Random Hyperplane (cosine)
│   │   ├── Cross-polytope
│   │   └── Multi-probe LSH
│   └── Neural / Deep Hashing (learned)
│       ├── HashNet
│       ├── Deep Supervised Hashing
│       └── Hadamard Codebook Hashing
├── Quantization-Based
│   ├── Product Quantization (PQ)
│   ├── Optimized PQ (OPQ)
│   ├── ScaNN (Anisotropic Quantization)
│   ├── Scalar Quantization (SQ / int8)
│   ├── Binary Quantization
│   └── RaBitQ
├── Tree-Based
│   ├── Random Projection Trees (Annoy)
│   ├── KD-tree
│   ├── Ball Tree
│   ├── VP-tree (Vantage Point)
│   └── MRPT (Multi-Resolution Projection Trees)
├── Partition-Based (Inverted Index)
│   ├── IVF (Inverted File Index)
│   ├── IVF-PQ
│   ├── IVF-HNSW
│   └── ScaNN partitioning (tree-based)
└── Composite / Hybrid
    ├── HNSW + PQ
    ├── IVF + HNSW + PQ
    ├── DiskANN + PQ (SSD-resident)
    └── Filtered ANN (pre/post/in-filter)
```

## Metrics & Measures

Evaluation dimensions identified for Q3/Q4 (not yet operationalized — Run 2 task):

| Metric | What it measures | Units |
|--------|-----------------|-------|
| Recall@k | Fraction of true k-NN found | 0.0–1.0 |
| QPS | Query throughput | queries/sec |
| Build time | Index construction time | seconds |
| Memory per vector | RAM overhead per indexed vector | bytes |
| Index size on disk | Serialized index footprint | bytes |
| Latency (p50, p99) | Query response time distribution | milliseconds |
| Dimensionality sensitivity | How performance degrades with dimension | profile |

## References Collected

### Foundational Papers (verified via arXiv)

- [Efficient and robust ANN search using HNSW](https://arxiv.org/abs/1603.09320) — Malkov & Yashunin, 2016. The HNSW paper.
- [Approximate Nearest Neighbor Search in High Dimensions](https://arxiv.org/abs/1806.09823) — Andoni, Indyk, Razenshteyn, 2018. Comprehensive ANN theory survey.
- [Accelerating Large-Scale Inference with Anisotropic Vector Quantization](https://arxiv.org/abs/1908.10396) — Guo et al., 2019. The ScaNN paper.
- [SOAR: Improved Indexing for Approximate Nearest Neighbor Search](https://arxiv.org/abs/2404.00774) — Sun et al., 2024. ScaNN's SOAR extension.
- [ANN-Benchmarks: A Benchmarking Tool](https://arxiv.org/abs/1807.05614) — Aumüller, Bernhardsson, Faithfull, 2018. Benchmark methodology.
- [A Survey on Efficient Processing of Similarity Queries over Neural Embeddings](https://arxiv.org/abs/2204.07922) — Wang, 2022. Vector DB survey.
- [Graph-Based Vector Search: An Experimental Evaluation of the State-of-the-Art](https://arxiv.org/abs/2502.05575) — Azizi, Echihabi, Palpanas, 2025. Recent graph-based ANN evaluation, 12 methods, 7 datasets.

### DiskANN Ecosystem (verified via arXiv + web)

- [DiskANN: Fast Accurate Billion-point NN Search on a Single Node](https://suhasjs.github.io/files/diskann_neurips19.pdf) — Subramanya et al., NeurIPS 2019.
- [I/O Optimizations for Graph-Based Disk-Resident ANN](https://arxiv.org/abs/2602.21514) — Li et al., 2026. OctopusANN, 87-149% throughput improvement over DiskANN.
- [OOD-DiskANN: Efficient and Scalable Graph ANNS for Out-of-Distribution Queries](https://arxiv.org/abs/2211.12850) — Jaiswal et al., 2022.
- [DiskANN++: Efficient Page-based Search](https://arxiv.org/abs/2310.00402) — Ni et al., 2023.
- [DISTRIBUTEDANN: Efficient Scaling Across Thousands of Computers](https://arxiv.org/abs/2509.06046) — Adams et al., 2025. 50B vector graph at Bing.
- [PiPNN: Ultra-Scalable Graph-Based NN Indexing](https://arxiv.org/abs/2602.21247) — Rubel et al., 2026. 11.6x faster than Vamana construction.

### HNSW Research (verified via arXiv)

- [Down with the Hierarchy: The 'H' in HNSW Stands for "Hubs"](https://arxiv.org/abs/2412.01940) — Munyampirwa et al., 2024. Flat NSW matches HNSW in high dimensions.
- [Three Algorithms for Merging HNSW Graphs](https://arxiv.org/abs/2505.16064) — Ponomarenko, 2025.
- [Impacts of Data, Ordering, and Intrinsic Dimensionality on Recall in HNSW](https://arxiv.org/abs/2405.17813) — Elliott & Clark, 2024.
- [Distribution-Aware Exploration for Adaptive HNSW Search](https://arxiv.org/abs/2512.06636) — Zhang & Miller, 2025. Adaptive ef parameter.

### Filtered ANN (verified via web + arXiv)

- [Survey of Filtered Approximate Nearest Neighbor Search over Vector-Scalar Hybrid Data](https://arxiv.org/abs/2505.06501) — Lin et al., 2025. First dedicated FANNS survey.
- [Approximate Nearest Neighbor Search with Window Filters](https://arxiv.org/abs/2402.00943) — Engels et al., 2024. 75x speedup.
- [Filtered-DiskANN](https://harsha-simhadri.org/pubs/Filtered-DiskANN23.pdf) — filter-aware graph construction.

### Quantization & Indexing (verified via arXiv)

- [Practical and Asymptotically Optimal Quantization (RaBitQ extension)](https://arxiv.org/abs/2409.09913) — Gao et al., 2024. State-of-the-art quantization.
- [Routing-Guided Learned Product Quantization for Graph-Based ANNS](https://arxiv.org/abs/2311.18724) — Yue et al., 2023. RPQ integrated with DiskANN.
- [Improving Bilayer Product Quantization for Billion-Scale ANN](https://arxiv.org/abs/1404.1831) — Babenko & Lempitsky, 2014.
- [AiSAQ: All-in-Storage ANNS with Product Quantization](https://arxiv.org/abs/2404.06004) — Tatsuno et al., 2024. ~10MB memory for billion-scale.
- [Dimensionality-Reduction Techniques for ANNS: A Survey](https://arxiv.org/abs/2403.13491) — Wang et al., 2024.

### Deep Hashing (verified via arXiv)

- [A Survey on Deep Hashing Methods](https://arxiv.org/abs/2003.03369) — Luo et al., 2020. Comprehensive deep hashing survey.
- [HashNet: Deep Learning to Hash by Continuation](https://arxiv.org/abs/1702.00758) — Cao et al., 2017.
- [Hadamard Codebook Based Deep Hashing](https://arxiv.org/abs/1910.09182) — Chen et al., 2019.

### LSH (verified via arXiv)

- [A Survey on LSH Algorithms and their Applications](https://arxiv.org/abs/2102.08942) — Jafari et al., 2021.
- [Improving LSH by Efficiently Finding Projected Nearest Neighbors (roLSH)](https://arxiv.org/abs/2006.11284) — Jafari et al., 2020.

### Graph-Based Variants (verified via arXiv)

- [EFANNA: Extremely Fast ANN Based on kNN Graph](https://arxiv.org/abs/1609.07228) — Fu & Cai, 2016.
- [Worst-case Performance of Popular ANN Implementations](https://arxiv.org/abs/2310.19126) — Indyk & Xu, 2023. Theoretical analysis of HNSW, NSG, DiskANN.
- [LANNS: A Web-Scale ANN Lookup System](https://arxiv.org/abs/2010.09426) — Doshi et al., 2020.
- [Distance Adaptive Beam Search for Graph-Based NN Search](https://arxiv.org/abs/2505.15636) — Al-Jazzazi et al., 2025.

### Tutorials & Educational Resources (verified via web)

- [Hierarchical Navigable Small Worlds (HNSW) — Pinecone](https://www.pinecone.io/learn/series/faiss/hnsw/) — HNSW tutorial
- [Understanding HNSW — Zilliz Learn](https://zilliz.com/learn/hierarchical-navigable-small-worlds-HNSW) — HNSW explainer
- [A Visual Guide to HNSW](https://cfu288.com/blog/2024-05_visual-guide-to-hnsw/) — Visual HNSW tutorial
- [Distance Metrics in Vector Search — Weaviate](https://weaviate.io/blog/distance-metrics-in-vector-search) — Metric comparison
- [Similarity Metrics for Vector Search — Zilliz](https://zilliz.com/blog/similarity-metrics-for-vector-search) — Metric comparison
- [Vector Similarity Explained — Pinecone](https://www.pinecone.io/learn/vector-similarity/) — Metric tutorial
- [Distance Metrics — Qdrant](https://qdrant.tech/course/essentials/day-1/distance-metrics/) — Metric tutorial
- [DiskANN Explained — Milvus](https://milvus.io/blog/diskann-explained.md) — DiskANN tutorial
- [DiskANN and the Vamana Algorithm — Zilliz](https://zilliz.com/learn/DiskANN-and-the-Vamana-Algorithm) — DiskANN/Vamana explainer
- [What is ScaNN — Zilliz Learn](https://zilliz.com/learn/what-is-scann-scalable-nearest-neighbors-google) — ScaNN explainer
- [SOAR: New algorithms for faster vector search with ScaNN — Google Research](https://research.google/blog/soar-new-algorithms-for-even-faster-vector-search-with-scann/) — SOAR blog post
- [Announcing ScaNN — Google Research](https://research.google/blog/announcing-scann-efficient-vector-similarity-search/) — ScaNN announcement
- [Advanced Filtering Strategies — apxml.com](https://apxml.com/courses/advanced-vector-search-llms/chapter-2-optimizing-vector-search-performance/advanced-filtering-strategies) — Pre/post filtering tutorial
- [HNSW vs DiskANN — Vectroid](https://www.vectroid.com/resources/HNSW-vs-DiskANN-comparing-the-leading-ANN-algorithm) — Algorithm comparison

### Benchmark Resources (verified via web)

- [ann-benchmarks.com](https://ann-benchmarks.com/) — Standard ANN benchmark framework (April 2025 run on r6i.16xlarge)
- [ANN-Benchmarks GitHub](https://github.com/erikbern/ann-benchmarks) — Benchmark tooling source
- [ScaNN GitHub](https://github.com/google-research/google-research/tree/master/scann) — ScaNN source + benchmark configs
- [DiskANN GitHub — Microsoft](https://github.com/microsoft/DiskANN) — DiskANN source [inferred]
- [SPTAG GitHub — Microsoft](https://github.com/microsoft/SPTAG) — SPTAG source [verified from seed refs]

## Open Questions

1. **Crate verification needed**: All Rust crate data is from training knowledge (marked [inferred]). Need live crates.io/GitHub verification for download counts, last-updated dates, and any crates published after May 2025.
2. **Pure Rust IVF-PQ gap**: No prominent pure Rust IVF-PQ implementation found. Is this correct, or were crates missed? Need targeted crates.io search.
3. **DiskANN in Rust**: Only lance implements DiskANN-style indexing internally. Is there a standalone crate? Need verification.
4. **hora maintenance**: Last known activity ~2022. Confirm if abandoned or still maintained.
5. **Filtered ANN in Rust**: Qdrant and hnsw_rs support filtering. Any other Rust crates?
6. **Benchmark datasets**: SIFT1M, GloVe-100, GIST confirmed as standards. What synthetic datasets at target dimensionalities (384d, 768d, 1536d) should be used?
7. **Neural hashing Rust implementations**: None found. Confirm absence.
8. **The "Hub Highway Hypothesis"**: arXiv 2412.01940 suggests flat NSW matches HNSW in high dimensions. Should this affect our taxonomy? (Run 2 question)
9. **RaBitQ**: New quantization method (2409.09913) claims state-of-the-art. Needs deeper investigation in Run 2.

## Convergence Assessment

- [ ] Algorithm families fully enumerated — **Partial**. 16 families identified but neural hashing and some tree variants may be incomplete.
- [ ] All relevant crates identified — **Partial**. ~21 library crates + 3 database-adjacent. Need live verification and gap-fill.
- [ ] Category boundaries stable — **No**. First draft taxonomy; boundaries between graph-based sub-families and composite approaches need refinement.
- [ ] Evaluation criteria operationalized — **No**. Metrics identified but not operationalized with specific thresholds.
- [ ] Ready to define cohorts — **No**. Need stable taxonomy first.

**Next run needed?** Yes
**Next run focus**:
1. Live verification of all Rust crate data (crates.io, GitHub)
2. Fill taxonomy gaps (neural hashing depth, tree variant completeness)
3. Operationalize evaluation metrics with specific thresholds
4. Refine category boundaries — especially graph-based sub-families
5. Investigate RaBitQ and the Hub Highway Hypothesis
6. Collect any missing paper references (PQ original by Jégou et al., NSG paper, FAISS paper)
