# Q1.h: Dimensionality-Dependent Algorithm Behavior

> **Run**: 3
> **Date**: 2026-04-24
> **Task**: Q1.h -- Characterize how ANN algorithms behave differently across dimensionality ranges

## Paper Sources

| ID | Paper | Venue | Year |
|----|-------|-------|------|
| [HubHwy] | Munyampirwa et al., "Down with the Hierarchy: The 'H' in HNSW Stands for 'Hubs'" (arXiv:2412.01940) | Preprint | 2024 |
| [DimImpact] | Elliott & Clark, "The Impacts of Data, Ordering, and Intrinsic Dimensionality on Recall in HNSW" (arXiv:2405.17813) | Preprint | 2024 |
| [WorstCase] | Indyk & Xu, "Worst-case Performance of Popular ANN Search Implementations" (arXiv:2310.19126) | Preprint | 2023 |
| [RaBitQ] | Gao & Long, "RaBitQ" (arXiv:2405.12497) | SIGMOD 2024 | 2024 |
| [ExtRaBitQ] | Gao et al., "Extended RaBitQ" (arXiv:2409.09913) | SIGMOD 2025 | 2024 |
| [GPU-RaBitQ] | Shi et al., "GPU IVF-RaBitQ" (arXiv:2602.23999) | Preprint | 2026 |

---

## 1. Core Phenomenon: The Curse of Dimensionality in ANN

As dimensionality increases, several effects compound:

1. **Distance concentration**: The ratio of nearest to farthest distances approaches 1, making discrimination harder. [inferred]
2. **Hubness**: A small number of points appear as nearest neighbors of disproportionately many other points. Hub formation intensifies with dimensionality. [cited: HubHwy]
3. **Intrinsic dimensionality divergence**: The gap between ambient dimension D and intrinsic dimensionality d_i determines effective algorithmic difficulty. High d_i (close to D) is hardest. [cited: DimImpact]
4. **Partition quality degradation**: Space-partitioning methods (trees, IVF clusters) produce increasingly overlapping partitions as dimensionality grows. [inferred]

---

## 2. Paper-by-Paper Analysis

### 2.1 Hub Highway Hypothesis [HubHwy]

**Core finding**: In high dimensions, flat NSW (without hierarchy) matches HNSW performance. The hierarchy adds no benefit because hub nodes naturally form "highways" that serve the same routing function as hierarchical layers.

**Dimensionality crossover** [cited: HubHwy]:
- **Below ~32 dimensions**: Hierarchy provides measurable benefit (tested on synthetic data 16D-1536D)
- **Above ~96 dimensions**: "No consistent and discernible gap between FlatNav and HNSW in both the median and tail latency cases"
- At 128D+ (BigANN, SIFT, DEEP): Performance is "essentially identical"
- At 960D (GIST): Identical performance

**Hub highway mechanism** [cited: HubHwy]:
- Hub nodes are points appearing "disproportionately high" in k-nearest neighbor lists across the dataset
- In high dimensions, distance concentration creates natural hubs
- Hubs form through **preferential attachment**: frequently-visited nodes accumulate more connections during graph construction
- Queries "concentrate in the highway structures early in search, shown by the high percentage of hub nodes visited in the first 5-10% of the search steps"
- The hub distribution is "right-skewed for L2 distance-based datasets" (more pronounced hubness)

**Memory savings** [cited: HubHwy]: Flat NSW achieves "38% and 39% memory savings during index construction" on Big-ANN benchmark datasets vs hnswlib, by eliminating the multi-layer structure.

**Datasets tested** [cited: HubHwy]:
| Dataset | Dimensions | Vectors | Hierarchy helps? |
|---------|-----------|---------|-----------------|
| Synthetic | 16-1536 | Varied | Only <32D |
| GloVe | 25, 50, 100, 200 | 1.2M | No (25D marginal) |
| SIFT | 128 | 1M | No |
| BigANN | 128 | 100M | No |
| SpaceV | 100 | 100M | No |
| DEEP | 96 | 100M | No |
| Text-to-Image | 200 | 100M | No |
| GIST | 960 | 1M | No |
| MNIST | 784 | 60K | No |
| NYTimes | 256 | 290K | No |

**Implication**: For modern embedding dimensions (96D+), investing in hierarchical HNSW layers wastes memory without improving search quality. Flat NSW with the same connectivity parameters suffices.

### 2.2 Intrinsic Dimensionality Impacts [DimImpact]

**Core finding**: HNSW recall is directly determined by the vector space's intrinsic dimensionality, and insertion order can shift recall by up to 12 percentage points.

**Intrinsic dimensionality vs recall** [cited: DimImpact]:
- Synthetic data (controlled intrinsic dim, 1024 ambient dim):
  - k_i = 2 (very low intrinsic dim): ~99% recall
  - k_i = 64: ~75% recall
  - k_i = 128: ~65% recall (estimated)
  - k_i = 256: ~60% recall (estimated)
  - k_i = 512: ~55% recall (estimated)
  - k_i = 1024 (full rank): ~50% recall
- This is approximately a 50 percentage point drop from lowest to highest intrinsic dimensionality.

**Insertion order effect** [cited: DimImpact]:
- Maximum shift: **12.8 percentage points** (FAISS, all-MiniLM-L6-v2 model)
- HNSWLib maximum shift: 5.6 percentage points (e5-small-v2)
- Average improvement from descending-LID insertion: 2.6 pp (HNSWLib), 6.2 pp (FAISS)
- Mechanism: Descending LID insertion "mimics simulated annealing" -- high-LID vectors establish initial graph structure, low-LID vectors form tight clusters afterward
- Practical trigger: Category-ordered ingestion (e.g., inserting all "shoes" then all "bags") creates correlated insertion patterns that can shift recall by up to 7.7 percentage points

**Default HNSW parameters surveyed** [cited: DimImpact]:
| System | M | efConstruction |
|--------|---|----------------|
| Median | 16 | 128 |
| FAISS | 32 | 40 |
| Weaviate | 64 | 128 |
| Qdrant | 16 | 100 |
| HNSWLib | 16 | 200 |
| Milvus | 18 | 240 |

**Practical implications** [cited: DimImpact]:
1. Fixed HNSW parameters cannot overcome dimensionality-based degradation -- "it is not feasible to extensively search the parameter space for optimal parameters"
2. Model ranking instability: approximate search can shift model rankings by up to 3 positions vs exact KNN
3. Smaller embedding models (all-MiniLM-L6-v2) can gain relative ranking positions under approximate search vs exact search
4. "Models trained with explicit dimensionality control could improve HNSW robustness"

### 2.3 Worst-Case Performance [WorstCase]

**Core finding**: Popular graph-based ANN algorithms (HNSW, NSG, DiskANN fast-preprocessing) can exhibit linear-time worst-case query performance on adversarial instances. Only DiskANN's slow-preprocessing variant has provable guarantees.

**DiskANN slow preprocessing** [cited: WorstCase]:
- **Guarantee**: Constant approximation ratio with poly-logarithmic query time on datasets with bounded intrinsic dimension
- This is the only variant among the three studied with formal guarantees

**DiskANN fast preprocessing** [cited: WorstCase]:
- Adversarial construction: query can require at least **0.1n steps** before encountering any of the 5 nearest neighbors on instances of size n
- This means on a 10M dataset, 1M distance computations may be needed -- effectively linear scan

**HNSW** [cited: WorstCase]:
- Worst-case: linear-time query performance on constructed adversarial instances
- "Empirical query time required to achieve a 'reasonable' accuracy is linear in instance size" on the family of bad instances

**NSG** [cited: WorstCase]:
- Same family of adversarial instances causes linear-time queries
- Similar limitations to HNSW in worst case

**Dimensionality relationship** [cited: WorstCase]:
- Guarantees for DiskANN slow preprocessing depend critically on **bounded intrinsic dimension**
- The adversarial instances for HNSW/NSG/DiskANN-fast exploit high intrinsic dimensionality
- Practical implication: on datasets with genuinely high intrinsic dimensionality (not just high ambient dimension), graph-based methods may degrade to near-linear scan

---

## 3. Dimensionality Sensitivity Table

| Family | Low-d (<20) | Medium-d (128) | High-d (768) | Very high-d (1536+) | Sensitivity | Source |
|--------|-------------|----------------|--------------|----------------------|-------------|--------|
| **HNSW** | Excellent recall; hierarchy provides measurable benefit | Excellent recall; hierarchy unnecessary (hub highways form) | Good recall with tuning; intrinsic dim determines actual difficulty; insertion order matters (up to 12pp shift) | Recall degrades with intrinsic dim; parameters cannot compensate | Moderate | [HubHwy], [DimImpact] |
| **NSW (flat)** | Slightly worse than HNSW (no hierarchy routing) | Identical to HNSW; 38-39% less memory | Identical to HNSW | Identical to HNSW | Moderate (same as HNSW) | [HubHwy] |
| **NSG / Vamana (DiskANN)** | Excellent; Vamana's alpha parameter gives slight edge in routing | Strong performance; shorter average search paths than HNSW | Strong; Vamana maintains edge in path length. DiskANN slow-preprocessing retains formal guarantees | Formal guarantees only for slow preprocessing + bounded intrinsic dim. Fast preprocessing worst-case is O(n). | Moderate-High | [WorstCase], [inferred from benchmarks] |
| **IVF** | Good; cluster boundaries well-separated | Adequate; nprobe must increase for high recall. Cluster overlap increases | Cluster quality degrades; higher nprobe needed. PQ sub-quantizer errors compound | Severe cluster overlap; nprobe approaches nlist for high recall. Approaches brute-force cost | High | [inferred from FAISS benchmarks] |
| **PQ** | Poor choice (sub-vectors too short for meaningful clustering) | Good compression/recall at 32x. Sub-vectors of 8-16 dims cluster well | Sub-vector structure still works but correlations across sub-vectors cause unbounded errors on some datasets | Maximum relative error can reach ~100% on hard datasets regardless of dimension | High (failure modes) | [RaBitQ] |
| **RaBitQ** | Error bound O(1/sqrt(D)) is weakest at low D. Not designed for D<32 | Error bound tightens: epsilon ~ 5.75/sqrt(128) ~ 0.51. Strong performance | epsilon ~ 5.75/sqrt(768) ~ 0.21. Excellent performance | epsilon ~ 5.75/sqrt(1536) ~ 0.15. Performance improves with dimension | **Inverse** (better at higher D) | [RaBitQ], [ExtRaBitQ] |
| **RPT (Annoy)** | Best regime. Trees partition space effectively when D<~20 | Functional but HNSW dominates recall-speed tradeoff. Trees require many more leaves | Performance degrades; hyperplane splits become less discriminative as D grows | Poor recall-speed tradeoff vs graph methods. "HNSW often achieves higher recall at similar speed levels, especially when dimensions exceed 100" | Very High | [inferred from benchmarks, web search] |
| **KD-tree** | Optimal regime. O(D log N) query time when D < ~20 | Degrades toward linear scan. Effective only with very low intrinsic dim | Approaches brute-force. Backtracking dominates | Equivalent to linear scan | Extreme | [inferred, classical result] |
| **LSH** | Outperformed by trees and graphs | Competitive on recall (~0.90) but memory-intensive (many hash tables). DET-LSH approaches HNSW on mid-sized datasets | Requires many hash tables for high recall. Sub-linear guarantees hold but constants are large | Theoretical guarantees hold (dimension-independent sublinear bounds) but practical constants make it slower than graph methods | Low (theoretically) / High (practically) | [inferred from benchmarks, web search] |

---

## 4. Qualitative Crossover Points

These are dimensionality thresholds where the relative ranking of algorithm families changes qualitatively (not just in degree).

### Crossover 1: HNSW hierarchy becomes irrelevant (~32D)

**Below 32D**: Hierarchical layers in HNSW provide measurable latency improvement over flat NSW. The hierarchy serves as an effective long-range routing mechanism.

**Above 32D**: Hub nodes in the flat graph naturally form "highways" that replicate the hierarchy's routing function. Flat NSW matches HNSW on latency and recall while using 38-39% less memory. [cited: HubHwy]

**Practical impact**: For modern embedding dimensions (96D+), the HNSW hierarchy is pure overhead. Systems could save significant memory by using flat NSW graphs.

### Crossover 2: Tree-based methods become uncompetitive (~20-30D)

**Below ~20D**: KD-trees are optimal (O(D log N) queries). Random projection trees (Annoy) are competitive.

**20D-100D**: Transitional zone. Trees still function but require exponentially more leaves/backtracking. Graph methods begin to dominate.

**Above ~100D**: Trees are strictly dominated by graph methods on the recall-speed Pareto frontier. Annoy requires vastly more trees to match HNSW recall. KD-trees degenerate to linear scan. [inferred from benchmarks]

### Crossover 3: RaBitQ surpasses PQ reliability (~128D+)

**Below ~64D**: RaBitQ's error bound O(1/sqrt(D)) is relatively loose. PQ can be competitive if the dataset is well-suited (uniform sub-vector variance).

**128D+**: RaBitQ's error bound tightens substantially (epsilon < 0.51 at 128D). PQ's lack of guarantees becomes a liability -- maximum relative error can spike to ~100% on hard datasets. RaBitQ provides strictly better accuracy-efficiency trade-off at equivalent compression. [cited: RaBitQ]

**768D+**: RaBitQ's advantage is pronounced (epsilon < 0.21). This is also where Extended RaBitQ's variable-rate option makes high recall (95-99%) achievable with moderate compression (4.5-6.4x). [cited: ExtRaBitQ]

### Crossover 4: IVF cluster quality degrades (~256D+)

**Below ~128D**: K-means clusters in IVF have well-separated boundaries. Moderate nprobe (1-5% of nlist) achieves high recall.

**256D+**: Cluster overlap increases due to distance concentration. Achieving 95% recall requires probing a larger fraction of clusters. The cost advantage over brute-force narrows.

**768D+**: IVF alone (without quantization) offers diminishing returns. The combination IVF+RaBitQ rescues cluster-based search by providing tight within-cluster distance estimation. GPU IVF-RaBitQ achieves 2.2x higher QPS than graph-based CAGRA at recall ~0.95 precisely at these dimensions (512D-3072D tested). [cited: GPU-RaBitQ]

### Crossover 5: LSH theoretical guarantees become practically relevant (???)

LSH has dimension-independent sublinear query guarantees (via the theory of locality-sensitive hash families). In practice, however, the constant factors make LSH slower than graph methods at all tested dimensions. There is no observed crossover point where LSH dominates in practice, despite its superior asymptotic worst-case guarantees. [inferred]

The exception is **streaming/dynamic settings**: LSH's O(1) insertion time gives it an advantage when the dataset changes frequently and graph re-construction is prohibitive. [inferred from web search]

---

## 5. Intrinsic vs Ambient Dimensionality

A critical distinction emerging from [DimImpact] and [WorstCase] is that **intrinsic dimensionality** (d_i), not ambient dimensionality (D), determines algorithmic difficulty:

| Ambient D | Intrinsic d_i | Effective difficulty | Example |
|-----------|---------------|---------------------|---------|
| 768 | 50-100 | Low-moderate | Clustered domain-specific embeddings |
| 768 | 200-400 | Moderate-high | General-purpose text embeddings |
| 768 | 600+ | Very high | Diverse multi-domain embeddings |
| 1536 | 100-200 | Moderate | OpenAI embeddings (observed LID range) |

**Key finding** [cited: DimImpact]: The recall drop from intrinsic dim k_i=2 to k_i=1024 is approximately 50 percentage points on HNSW with default parameters. This means two datasets with the same ambient dimension D=768 can have wildly different HNSW recall depending on their intrinsic structure.

**Implication for algorithm selection**: Measuring intrinsic dimensionality (e.g., via Maximum Likelihood Estimation of LID) before choosing an algorithm or tuning parameters is more informative than relying on ambient dimension alone.

---

## 6. Dimensionality and Quantization Interaction

Dimensionality affects quantization methods differently than it affects graph/tree methods:

| Method | Dimensionality effect on quantization quality |
|--------|-----------------------------------------------|
| **PQ** | More dimensions = more sub-vectors = finer quantization... but also more opportunities for cross-sub-vector correlation errors. Net effect: unpredictable, dataset-dependent. [cited: RaBitQ] |
| **RaBitQ** | More dimensions = tighter error bound (O(1/sqrt(D))). **Quantization quality improves with dimension.** This is the inverse of the curse of dimensionality. [cited: RaBitQ, ExtRaBitQ] |
| **TurboQuant** | Same inverse relationship claimed: higher D concentrates the Beta distribution more tightly, improving scalar quantizer effectiveness. [cited: TurboQuant] |
| **SQ (int8)** | Dimension-agnostic per-coordinate quantization. Error scales linearly with D for aggregate distance. [inferred] |
| **Binary** | Angular preservation improves with dimension (Johnson-Lindenstrauss effect). But absolute recall still poor without re-ranking. [inferred] |

This creates an important asymmetry: graph-based search gets harder with dimension (or intrinsic dimension), but RaBitQ-family quantization gets more accurate. This is why the combination IVF+RaBitQ is particularly powerful at high dimensions -- the quantization compensates for the cluster-based search's increasing difficulty. [cited: GPU-RaBitQ]

---

## 7. Summary of Algorithm Selection by Dimension

| Dimension range | Recommended approach | Rationale |
|----------------|---------------------|-----------|
| D < 20 | KD-tree or ball tree | Exact or near-exact in O(D log N). Trees are optimal here. |
| 20 <= D < 100 | HNSW or flat NSW | Trees degrade; graphs dominate. Hierarchy still marginally useful below ~32D. |
| 100 <= D < 512 | Flat NSW + SQ or RaBitQ | Hierarchy unnecessary. RaBitQ provides 32x compression with tightening error bounds. |
| 512 <= D < 1536 | IVF-RaBitQ (GPU) or flat NSW + Extended RaBitQ | RaBitQ error bound is tight (epsilon < 0.26). IVF-RaBitQ on GPU outperforms CAGRA at 2.2x QPS. Variable-rate for recall > 95%. |
| D >= 1536 | IVF-RaBitQ or brute-force + RaBitQ | Error bound very tight (epsilon < 0.15). At very high D, even brute-force with quantized vectors can be competitive for moderate dataset sizes. |

**Caveat**: These recommendations assume intrinsic dimensionality is a moderate fraction of ambient dimensionality. For datasets with very low intrinsic dimensionality relative to D, graph methods will perform much better than this table suggests. Measure intrinsic dimensionality before committing to an approach.

---

## 8. Open Questions

1. **Quantifying the hub highway**: [HubHwy] provides qualitative evidence but no formula relating dimensionality to hub strength. A model predicting when hierarchy helps (as a function of intrinsic dim and dataset size) would be valuable.
2. **Insertion order in production**: [DimImpact] shows 12pp recall shifts from insertion order. No vector database currently exposes LID-aware insertion ordering. This is a low-hanging optimization.
3. **Worst-case frequency in practice**: [WorstCase] constructs adversarial instances, but how often real datasets approach worst-case behavior is unknown. Characterizing "distance to worst case" for common embedding models would help risk assessment.
4. **RaBitQ at very low D**: RaBitQ's error bound weakens below ~32D. Whether a modified approach (e.g., padding to 32D) is beneficial has not been studied.
5. **Filtered ANN and dimensionality**: None of these papers study how dimensionality interacts with attribute filtering (a cross-cutting concern identified in Run 2). Filtering reduces effective dataset size within clusters, potentially changing the dimensionality-performance relationship.
