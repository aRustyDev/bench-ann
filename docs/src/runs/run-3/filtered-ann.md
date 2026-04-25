# Q1.f: Filtered ANN Deep-Dive -- Run 3

## Filtering Approaches Taxonomy

Filtered Approximate Nearest Neighbor Search (FANNS) extends standard ANNS by incorporating scalar attribute filters into the search. The FANNS survey [Lin et al. 2025] proposes a **pruning-focused framework** with four strategy families, superseding the simpler pre/post/in-filter trichotomy:

### Comparison Table

| Strategy | Survey Label | Mechanism | Index Modification | Best Selectivity Range | Representative Algorithms |
|---|---|---|---|---|---|
| **Post-filter** | Vector-Solely Pruning (VSP) | Standard ANN search on full index, discard non-matching results afterward | None | Low selectivity (filter excludes < 30% of data) | VBase [Zhang et al. 2023] |
| **In-filter (vector-centric)** | Vector-Centric Joint Pruning (VJP) | Integrate filter checks into graph/index traversal; skip non-matching nodes during search | Minimal to moderate | Moderate selectivity (30%--70% exclusion) | ACORN, AIRSHIP, Faiss-IVF, NHQ, HQANN |
| **Filter-aware construction** | VJP subclass | Build index with label/filter awareness; edges encode both geometric + attribute proximity | Heavy (construction-time) | Moderate to high selectivity | Filtered-DiskANN [Gollapudi et al. 2023], SeRF, iRangeGraph |
| **Pre-filter** | Scalar-Solely Pruning (SSP) | Apply scalar filter first, then ANN search on filtered subset | None (but may need brute-force for tiny subsets) | High selectivity (> 70% exclusion) | Standard pre-filtering |
| **Hybrid (scalar-centric)** | Scalar-Centric Joint Pruning (SJP) | Partition data by scalar attributes, search relevant partitions, then apply remaining filters | Partitioning structure | High selectivity | Milvus-Partition, HQI, MA-NSW, UNG, WST |
| **Window filter** | (Specialized VJP/SJP) | Build hierarchical tree of sub-indices over label-sorted data; query only sub-indices whose label ranges overlap the filter window | Heavy (tree of indices) | Medium filter fractions (2^-8 to 2^-4 of dataset) | beta-WST [Engels et al. 2024] |

*Sources: [Lin et al. 2025, arXiv:2505.06501]; [Gollapudi et al. 2023, WWW'23]; [Engels et al. 2024, arXiv:2402.00943]*

### Detailed Approach Descriptions

#### Post-filter (VSP)

Performs standard ANN search on the full, unfiltered index and discards results that do not satisfy the scalar filter. Requires minimal modifications to existing ANNS indices [Lin et al. 2025].

**When it works:** Unselective filters where most data passes (selectivity < 30%). The ANN search returns enough matching candidates naturally.

**When it fails:** Highly selective filters. If only 5% of the dataset matches and you request k=20, a standard top-100 ANN result may contain only ~5 matching items, yielding very low recall [Milvus blog; arXiv:2602.11443]. At extreme selectivity (< 1%), post-filter recall collapses entirely.

#### Pre-filter (SSP)

Applies the scalar filter first to obtain a candidate set, then performs ANN search exclusively on that subset. Can use brute-force scan for very small subsets or a pre-built sub-index [Lin et al. 2025].

**When it works:** Highly selective filters that eliminate most data. The filtered subset is small enough for efficient search.

**When it fails:** Unselective filters. Pre-filtering is "inefficient when scalar filters are unselective" [Lin et al. 2025] because the filtered subset remains nearly as large as the full dataset, gaining no speedup while incurring filter overhead. On 1M vectors, pre-filtering with < 2% selectivity was ~7x slower than post-filtering [Manticore Search benchmark].

#### In-filter / Interleaved (VJP)

Integrates filter checks directly into graph traversal. During HNSW or Vamana greedy search, each candidate node is checked against the scalar filter before being added to the result set. Non-matching nodes may still be traversed for graph connectivity but are excluded from results [Lin et al. 2025].

**Implementations vary:**
- **ACORN** [Weaviate]: Traverses only the predicate subgraph exclusively, skipping edges to non-matching nodes
- **AIRSHIP**: Uses a probabilistic visited approach on graphs
- **Faiss-IVF**: Skips similarity calculations for non-matching scalars within clusters

**When it works:** Moderate selectivity (30--70%). Balances the cost of filter checking against search efficiency.

**When it fails:** Can suffer from "subgraph connectivity" problems -- if the filtered subset of the graph is poorly connected, greedy search gets stuck in local minima [Lin et al. 2025]. The survey notes "results may be less reliable" for some VJP methods.

#### Filter-aware Construction (VJP subclass)

Builds the index knowing about filter attributes. Edges are chosen based on both vector similarity and shared labels, ensuring the graph is navigable within any filter context [Gollapudi et al. 2023].

**Two algorithms from Filtered-DiskANN:**

1. **FilteredVamana**: Incremental algorithm. Inserts each point using FilteredGreedySearch that considers both geometric distance and label overlap. Neighbors are selected based on shared labels, ensuring each label's induced subgraph is well-connected. Uses per-label medoid start nodes for efficient entry [Gollapudi et al. 2023].

2. **StitchedVamana**: Bulk-indexing algorithm. Builds a separate Vamana index for each label's point set, then overlays all per-label graphs by unioning their edges. Runs RobustPrune on nodes exceeding degree bound R to control index size [Gollapudi et al. 2023].

**Performance:** StitchedVamana achieves 6x better QPS at 90% recall than the next best prior technique; FilteredVamana achieves 2.5x. StitchedVamana offers 2x better QPS than FilteredVamana, but FilteredVamana has faster total indexing time [Gollapudi et al. 2023].

**When it works:** Any selectivity level, but especially strong at moderate-to-high selectivity. The filter-aware graph structure ensures connectivity within every label's subgraph.

**When it fails:** High index construction cost (must build per-label or label-aware structures). Not suitable for frequently changing filter schemas.

#### Window Filters (beta-WST)

Specialized for numeric range filters (timestamps, prices). Points are sorted by label value, recursively partitioned into a beta-ary tree, with a Vamana index built at each internal node. Queries traverse only nodes whose label ranges intersect the filter window [Engels et al. 2024].

**Performance:** Up to 75x speedup over baselines at 0.95 recall on Deep-1B (9.9M points, filter fraction 2^-7). On SIFT/GloVe/Redcaps: 9--17x speedups [Engels et al. 2024].

**Trade-offs:** Memory overhead of 3--8x a single Vamana index (beta-WST) or 5--14x (SuperPostfiltering variant). Construction time scales significantly (SIFT: 1min single index to 8min for 2-WST; Deep: 17min to 2hrs) [Engels et al. 2024].

**When it works:** Medium filter fractions (2^-8 to 2^-4), i.e., the query window covers 0.4%--6.25% of the label range. This is precisely the "dead zone" where both pre-filter and post-filter degrade [Engels et al. 2024].

**When it fails:** Very narrow windows (pre-filter is better) or very wide windows (post-filter suffices). Also requires numeric/ordinal label types -- not applicable to categorical filters.

---

## Filter Selectivity Trade-offs

### The "Selectivity Cliff"

Filter selectivity is the fraction of data excluded by the filter. All filtering approaches exhibit a "comfort zone" beyond which recall or throughput degrades sharply [arXiv:2602.11443; Milvus blog].

### Selectivity Thresholds by Approach

| Approach | Comfort Zone | Degradation Onset | Failure Mode |
|---|---|---|---|
| **Post-filter** | Selectivity < 30% (most data passes) | ~30% selectivity | At 95% selectivity (5% match rate), a top-100 search yields ~5 matches for k=20 -- recall collapses [Milvus blog] |
| **Pre-filter** | Selectivity > 70% (small filtered subset) | < 30% selectivity | Filtered subset nearly as large as full dataset; 7x slower than post-filter at < 2% selectivity on 1M vectors [Manticore] |
| **In-filter (graph)** | 20--80% selectivity | < 20% or > 80% | Subgraph connectivity loss at high selectivity; overhead of per-node filter checks at low selectivity [Lin et al. 2025] |
| **Filter-aware construction** | 10--95% selectivity | < 10% (construction overhead wasted) | Construction cost scales with label cardinality; diminishing returns when filters are unselective |
| **Window filter** | Filter fraction 2^-8 to 2^-4 (~0.4--6.25%) | Outside this range | 3--8x memory overhead; not applicable to categorical filters [Engels et al. 2024] |

### System-Level Adaptive Thresholds

Production systems use adaptive strategies based on observed selectivity:

- **Milvus**: Detects when ~99% of data is excluded by the filter and falls back to brute-force scanning, bypassing the graph index entirely [Milvus blog]
- **Weaviate (ACORN)**: Activates ACORN-1 in-filter mode when fewer than 60% of documents pass the filter; above that threshold, naive prefiltering suffices [Weaviate blog]
- **Azure AI Search**: Recommends pre-filter for selective queries, post-filter for broad queries, with automatic strategy selection [Microsoft Learn]

### The Distribution Factor

The FANNS survey [Lin et al. 2025] identifies a critical insight: **selectivity alone does not fully characterize query difficulty**. Two queries with identical selectivity can produce very different performance if the filtered points are spatially clustered near the query vector vs. dispersed throughout the space. The survey proposes measuring distributional divergence via Wasserstein distance between the filtered subset's distribution and the full dataset's distribution. High divergence (filtered points far from query neighborhood) makes all approaches harder.

---

## Rust Crate Filtering Support

### hnsw_rs

| Attribute | Value |
|---|---|
| **Crate** | [hnsw_rs](https://crates.io/crates/hnsw_rs) |
| **Source** | [github.com/jean-pierreBoth/hnswlib-rs](https://github.com/jean-pierreBoth/hnswlib-rs) |
| **Docs** | [docs.rs/hnsw_rs](https://docs.rs/hnsw_rs/latest/hnsw_rs/) |
| **Filtering approach** | **In-filter** (filtering during graph traversal, not post-filter) |
| **Quality** | Mature; native Rust HNSW with flexible filtering |

**API -- Two filtering modes:**

1. **Sorted ID list**: Pass a `Vec<DataId>` of allowed IDs (sorted) as a parameter. The search only includes results whose IDs are in this list. Efficient when the allowed set is precomputed (e.g., from a database query).

2. **Predicate function**: Define a function/closure that is called before each candidate ID is added to the result set. Returns `true` to include, `false` to exclude.

3. **Custom `Filterable` trait**: Users can implement the `Filterable` trait for custom types (e.g., bitvector-backed filters) to provide arbitrary filter logic.

Filtering is performed **during search traversal** -- candidates are checked against the filter before being added to the result set, making this an in-filter approach. Non-matching nodes are still traversed for graph connectivity but excluded from final results.

*Sources: [hnsw_rs docs.rs](https://docs.rs/hnsw_rs/latest/hnsw_rs/); [hnswlib-rs README](https://github.com/jean-pierreBoth/hnswlib-rs)*

---

### usearch

| Attribute | Value |
|---|---|
| **Crate** | [usearch](https://crates.io/crates/usearch) |
| **Source** | [github.com/unum-cloud/USearch](https://github.com/unum-cloud/USearch) |
| **Docs** | [docs.rs/usearch](https://docs.rs/usearch/latest/usearch/) |
| **Filtering approach** | **In-filter** (predicate applied during graph traversal) |
| **Quality** | Production-grade; C++ core with Rust FFI bindings |

**API:**

```rust
// Predicate signature: Key -> bool
// Key is u64 (type Key = u64)
let is_odd = |key: Key| key % 2 == 1;
let results = index.filtered_search(&query, 10, is_odd).unwrap();
```

The `filtered_search` method on `Index` takes:
- `query`: reference to the query vector
- `count`: number of results to return (k)
- `filter`: a closure/function `Fn(Key) -> bool` where `Key = u64`

The predicate is applied **directly during graph traversal** in the C++ HNSW core. As the USearch documentation states: "In USearch you can simply pass a predicate function to the search method, which will be applied directly during graph traversal" -- this avoids the paging/post-filtering approach where you manually request successively larger result sets.

*Sources: [usearch docs.rs](https://docs.rs/usearch/latest/usearch/); [USearch Rust README](https://github.com/unum-cloud/usearch/tree/main/rust)*

---

### arroy

| Attribute | Value |
|---|---|
| **Crate** | [arroy](https://crates.io/crates/arroy) |
| **Source** | [github.com/meilisearch/arroy](https://github.com/meilisearch/arroy) |
| **Docs** | [docs.rs/arroy](https://docs.rs/arroy/latest/arroy/) |
| **Filtering approach** | **Pre-filter with optimized intersection** (RoaringBitmap candidates passed to search) |
| **Quality** | Production-grade; powers Meilisearch vector search |

**API:**

Arroy's `Reader` provides search methods that accept a `candidates: &RoaringBitmap` parameter representing the pre-filtered subset of item IDs to search within:

```rust
// Conceptual API (arroy::Reader)
reader.nns_by_vector(
    &rtxn,           // LMDB read transaction
    &query_vector,   // query
    k,               // number of neighbors
    None,            // optional search_k parameter
    Some(&candidates) // RoaringBitmap of allowed item IDs
)?;
```

**How RoaringBitmap filtering works:**

Arroy stores descendant item IDs at tree leaf nodes as RoaringBitmaps (replacing the raw integer lists from Spotify's Annoy). During search, the tree traversal intersects each node's descendant bitmap with the `candidates` bitmap. If the intersection is empty for a subtree, that entire branch is pruned. This is effectively a **pre-filter that is deeply integrated into the tree traversal** -- the candidate set is provided upfront, but pruning happens at each tree node via bitmap intersection rather than as a flat pre-filter step [Kerollmops blog, 2023].

**Meilisearch integration:**

Meilisearch computes the `candidates` RoaringBitmap from its filter expressions (e.g., `genre = "sci-fi" AND year > 2020`) using its inverted index, then passes this bitmap to arroy. The bitmap intersection at each tree node ensures only matching documents are considered, implementing what the Kerollmops blog calls "Filtered Disk ANN" -- inspired by Microsoft's Filtered-DiskANN but adapted to arroy's random-projection tree structure [Kerollmops blog, 2023].

*Sources: [arroy GitHub](https://github.com/meilisearch/arroy); [Kerollmops blog: "Meilisearch Expands Search Power with Arroy's Filtered Disk ANN"](https://blog.kerollmops.com/meilisearch-expands-search-power-with-arroy-s-filtered-disk-ann); [arroy docs.rs](https://docs.rs/arroy/latest/arroy/)*

---

### diskann-rs

| Attribute | Value |
|---|---|
| **Crate** | [diskann-rs](https://crates.io/crates/diskann-rs) |
| **Source** | [github.com/lukaesch/diskann-rs](https://github.com/lukaesch/diskann-rs) |
| **Docs** | [docs.rs/diskann-rs](https://docs.rs/diskann-rs/latest/diskann_rs/) |
| **Filtering approach** | **Filter-aware construction** (labels at build time + filtered search) |
| **Quality** | Newer crate; implements ideas from Filtered-DiskANN paper |

**API:**

```rust
// Build with labels (multi-label per vector)
let labels: Vec<Vec<u64>> = /* e.g., category IDs per vector */;
FilteredDiskANN::<DistL2>::build(&vectors, &labels, "filtered.db")?;

// Filtered search
let filter = Filter::label_eq(42);  // equality filter
let filter = Filter::label_range(10, 50);  // range filter
let filter = Filter::label_eq(42).and(Filter::label_range(10, 50));  // compound

let results = index.search_filtered(&query, k, beam_width, &filter)?;
```

**Filter-aware construction:** Labels are provided at index build time (`Vec<Vec<u64>>`, supporting multiple labels per vector). The index construction incorporates label information into graph edge selection, following the Filtered-DiskANN paper's approach. This means the resulting Vamana graph has edges that respect label structure, ensuring good connectivity within each label's subgraph.

**Also notable -- infinilabs/diskann crate:**

| Attribute | Value |
|---|---|
| **Crate** | [diskann](https://crates.io/crates/diskann) |
| **Source** | [github.com/infinilabs/diskann](https://github.com/infinilabs/diskann) |
| **Features** | Pure Rust DiskANN with filtered search, incremental updates, compaction |

The infinilabs crate is a more comprehensive pure-Rust DiskANN implementation with disk-based query support, `FilteredDiskANN` API, incremental index updates, and compaction. It is "based on ideas from the DiskANN, Fresh-DiskANN, and Filtered-DiskANN papers" [infinilabs GitHub]. Trades ~60% slower build time for 6--10x lower memory usage and 15x faster incremental updates compared to alternatives.

*Sources: [diskann-rs crates.io](https://crates.io/crates/diskann-rs); [diskann-rs GitHub](https://github.com/lukaesch/diskann-rs); [infinilabs/diskann GitHub](https://github.com/infinilabs/diskann)*

---

### Summary: Rust Crate Filtering Comparison

| Crate | Index Type | Filter Approach | Filter Input | Filter Timing | Multi-label | Production Use |
|---|---|---|---|---|---|---|
| **hnsw_rs** | HNSW graph | In-filter | Sorted ID list, predicate fn, or `Filterable` trait | During traversal | N/A (external) | Research/production |
| **usearch** | HNSW graph | In-filter | `Fn(Key) -> bool` predicate | During traversal | N/A (external) | Production (Unum) |
| **arroy** | Random-projection trees | Pre-filter (bitmap intersection) | `RoaringBitmap` candidates | At each tree node | N/A (external) | Production (Meilisearch) |
| **diskann-rs** | Vamana graph | Filter-aware construction | `Filter` type (eq/range/compound) | Construction + search | Yes (`Vec<Vec<u64>>`) | Newer |
| **diskann** (infinilabs) | Vamana graph | Filter-aware construction | `FilteredDiskANN` API | Construction + search | Yes | Newer |

---

## Key Findings from Literature

### FANNS Survey (Lin et al. 2025) -- arXiv:2505.06501

1. **No single algorithm dominates.** Performance depends on selectivity, data distribution, and filter type. The survey explicitly states: "No algorithm dominates across all conditions; context-dependent selection is essential."

2. **The pruning-focused framework** (VSP/VJP/SJP/SSP) provides finer-grained classification than the traditional pre/post/in-filter trichotomy. VJP (Vector-Centric Joint Pruning) is further divided into graph-based, IVF-based, fusion-based, and construction-aware subcategories.

3. **Distribution factor matters beyond selectivity.** Two queries with identical selectivity can yield very different performance based on spatial distribution of filtered points relative to the query. The survey proposes Wasserstein distance as a quantitative measure.

4. **Fusion-based methods** (NHQ, HQANN) achieve high efficiency under their assumptions but have "questionable reliability" and "applicability may be limited by restrictive assumptions."

5. **Graph indices offer superior speed/accuracy** while IVF indices provide better memory efficiency -- this trade-off extends to filtered search.

### Window Filters (Engels et al. 2024) -- arXiv:2402.00943

1. **75x speedup** achieved on Deep-1B dataset (9.9M points) at 0.95 recall for medium filter fractions (2^-7). This is a real and reproducible result but applies specifically to numeric range filters, not categorical.

2. **Fills the dead zone** between pre-filter and post-filter. At medium filter fractions (0.4%--6.25% of label range), both pre-filter and post-filter degrade; window filters provide the only efficient option.

3. **Significant memory and construction costs:** 3--8x memory overhead (beta-WST) and construction times 8--50x longer than a single Vamana index.

4. **Theoretical guarantees** provided: O(beta * log_beta(N) * d) query time with provable c-approximate correctness.

### Filtered-DiskANN (Gollapudi et al. 2023) -- WWW'23

1. **Filter-aware graph construction is the key innovation.** By building edges that respect both geometric proximity and label co-occurrence, the resulting graph is navigable within any label's subgraph -- solving the "subgraph connectivity" problem that plagues in-filter approaches on standard graphs.

2. **StitchedVamana vs FilteredVamana trade-off:** StitchedVamana is 2x faster at query time but slower to build (bulk indexing). FilteredVamana is incremental and suitable for streaming data.

3. **6x improvement** over prior state-of-the-art at 90% recall for StitchedVamana.

4. **Per-label medoid entry points** improve search efficiency by starting greedy search from a geometrically central node within each label's point set, rather than a global start node.

---

## References

### Papers

| Ref | Citation | Access |
|---|---|---|
| [Lin et al. 2025] | Lin, Y., Zhang, K., He, Z., Jing, Y., & Wang, X.S. (2025). "Survey of Filtered Approximate Nearest Neighbor Search over the Vector-Scalar Hybrid Data." | [arXiv:2505.06501](https://arxiv.org/abs/2505.06501) |
| [Engels et al. 2024] | Engels, J., Landrum, B., Yu, S., Dhulipala, L., & Shun, J. (2024). "Approximate Nearest Neighbor Search with Window Filters." | [arXiv:2402.00943](https://arxiv.org/abs/2402.00943) |
| [Gollapudi et al. 2023] | Gollapudi, S., Karia, N., et al. (2023). "Filtered-DiskANN: Graph Algorithms for Approximate Nearest Neighbor Search with Filters." WWW'23. | [ACM DL](https://dl.acm.org/doi/10.1145/3543507.3583552) / [PDF](https://harsha-simhadri.org/pubs/Filtered-DiskANN23.pdf) |
| [arXiv:2602.11443] | "Filtered Approximate Nearest Neighbor Search in Vector Databases: System Design and Performance Analysis." (2026). | [arXiv:2602.11443](https://arxiv.org/abs/2602.11443) |
| [arXiv:2509.07789] | "Filtered Approximate Nearest Neighbor Search: A Unified Benchmark and Systematic Experimental Study." (2025). | [arXiv:2509.07789](https://arxiv.org/abs/2509.07789) |

### Rust Crate Documentation

| Crate | Docs | Source |
|---|---|---|
| hnsw_rs | [docs.rs/hnsw_rs](https://docs.rs/hnsw_rs/latest/hnsw_rs/) | [GitHub](https://github.com/jean-pierreBoth/hnswlib-rs) |
| usearch | [docs.rs/usearch](https://docs.rs/usearch/latest/usearch/) | [GitHub](https://github.com/unum-cloud/USearch) |
| arroy | [docs.rs/arroy](https://docs.rs/arroy/latest/arroy/) | [GitHub](https://github.com/meilisearch/arroy) |
| diskann-rs | [docs.rs/diskann-rs](https://docs.rs/diskann-rs/latest/diskann_rs/) | [GitHub](https://github.com/lukaesch/diskann-rs) |
| diskann (infinilabs) | [crates.io](https://crates.io/crates/diskann) | [GitHub](https://github.com/infinilabs/diskann) |

### Blog Posts and Industry Sources

| Source | URL |
|---|---|
| Kerollmops: "Meilisearch Expands Search Power with Arroy's Filtered Disk ANN" | [blog.kerollmops.com](https://blog.kerollmops.com/meilisearch-expands-search-power-with-arroy-s-filtered-disk-ann) |
| Milvus: "How to Filter Efficiently Without Killing Recall" | [milvus.io/blog](https://milvus.io/blog/how-to-filter-efficiently-without-killing-recall.md) |
| Weaviate: "How we speed up filtered vector search with ACORN" | [weaviate.io/blog](https://weaviate.io/blog/speed-up-filtered-vector-search) |
| TDS: "Effects of filtered HNSW searches on Recall and Latency" | [towardsdatascience.com](https://towardsdatascience.com/effects-of-filtered-hnsw-searches-on-recall-and-latency-434becf8041c/) |
| Microsoft: "Vector Query Filters - Azure AI Search" | [learn.microsoft.com](https://learn.microsoft.com/en-us/azure/search/vector-search-filters) |
| Manticore: "KNN prefiltering in Manticore Search" | [manticoresearch.com](https://manticoresearch.com/blog/knn-prefiltering/) |
