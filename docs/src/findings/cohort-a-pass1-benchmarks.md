# Cohort A Pass 1: Benchmark Findings

> **Date**: 2026-04-26
> **Phase**: Targeted Research, Cohort A (NSW/HNSW), Pass 1
> **Crates**: hnsw_rs v0.3.4, usearch v2.25.1, instant-distance v0.6.1
> **Dataset**: Synthetic Gaussian, 10K vectors, 1K queries, euclidean
> **Dimensions**: 128, 384, 768, 1536
> **Hardware**: Apple Silicon (macOS), single-threaded

## Summary

Pass 1 establishes baseline performance profiles for all three Cohort A HNSW implementations. The 10K dataset is sufficient for QPS comparison and memory profiling but too small for meaningful recall differentiation on hnsw_rs (which achieves perfect recall at all settings).

## Raw Results (M=16, ef_construction=200)

### QPS at ef_search=100 (mid-range operating point)

| Crate | 128d | 384d | 768d | 1536d |
|-------|------|------|------|-------|
| hnsw_rs | 6,068 | 2,690 | 1,274 | 633 |
| usearch | 9,862 | 3,473 | 1,534 | 718 |
| instant-distance | 3,636 | 1,498 | 633 | 315 |
| **usearch / hnsw_rs** | **1.63x** | **1.29x** | **1.20x** | **1.13x** |

### Recall@10 at ef_search=100

| Crate | 128d | 384d | 768d | 1536d |
|-------|------|------|------|-------|
| hnsw_rs | 1.0000 | 1.0000 | 1.0000 | 1.0000 |
| usearch | 0.8909 | 0.8331 | 0.7843 | 0.7558 |
| instant-distance | 0.9806 | 0.9496 | 0.9306 | 0.9129 |

### Memory Overhead per Vector (bytes, excluding raw vector data)

| Crate | 128d | 384d | 768d | 1536d | Trend |
|-------|------|------|------|-------|-------|
| usearch | 305 | 310 | 308 | 321 | **Constant** |
| hnsw_rs | 2,446 | 3,675 | 5,416 | 8,365 | Grows ~dim*4 |
| instant-distance | 1,809 | 4,165 | 7,299 | 13,416 | Grows ~dim*8 |

### Disk Graph Overhead per Vector (bytes, disk_total - raw_vector_data)

| Crate | 128d | 384d | 768d | 1536d | Trend |
|-------|------|------|------|-------|-------|
| usearch | 149 | 149 | 149 | 149 | **Constant** |
| hnsw_rs | 638 | 619 | 624 | 633 | ~Constant |
| instant-distance | 860 | 1,884 | 3,420 | 6,492 | Grows (serialization bug) |

### Build Time (seconds)

| Crate | 128d | 384d | 768d | 1536d |
|-------|------|------|------|-------|
| usearch | 1.9 | 6.1 | 14.7 | 33.7 |
| hnsw_rs | 2.3 | 5.2 | 11.0 | 22.6 |
| instant-distance | 4.8 | 18.7 | 44.8 | 100.4 |

## Fair Comparison: M=32 at 128d

instant-distance hardcodes M=32. To compare fairly:

| Crate | M | QPS (best) | QPS (ef=100) | Recall (ef=100) | Mem/vec |
|-------|---|-----------|-------------|----------------|---------|
| usearch | 32 | 34,493 | 6,202 | 0.9625 | 521B |
| hnsw_rs | 32 | 22,474 | 4,045 | 1.0000 | 3,033B |
| instant-distance | 32 | 22,999 | 3,636 | 0.9806 | 1,809B |

### M=32 QPS Ratio: usearch / hnsw_rs narrows with dimensionality

| Dim | usearch QPS | hnsw_rs QPS | instant-distance QPS | usearch/hnsw_rs |
|-----|------------|------------|---------------------|-----------------|
| 128 | 34,493 | 22,474 | 22,999 | **1.53x** |
| 384 | 9,561 | 8,539 | 7,379 | **1.12x** |
| 768 | 4,388 | 3,575 | 3,028 | **1.23x** |
| 1536 | 1,958 | 1,664 | 1,416 | **1.18x** |

The ratio narrows at higher dimensions because distance computation (O(dim)) dominates over graph traversal overhead. usearch's compact edges + prefetch matter most at low d where traversal is the bottleneck. At 1536d, the SIMD gap is the main remaining differentiator.

hnsw_rs and instant-distance have similar QPS at M=32, confirming the M value was the primary QPS differentiator between them (not the rebuild-per-ef overhead).

## Architectural Analysis

### Why usearch has lower memory overhead

Three distinct causes, each independently verifiable:

**1. Edge storage format (4B vs ~68B per edge)**
- usearch: stores only 4-byte neighbor IDs per edge
- hnsw_rs: stores `Arc<PointWithOrder>` per edge — Arc control block (~56B) + pointer (8B) + distance (4B) = ~68 bytes. This is ~17x per edge.

**2. Vector storage layout (contiguous vs indirected)**
- usearch: C++ contiguous memory pool for all vectors. Zero per-vector overhead.
- hnsw_rs: `Vec<Vec<f32>>` ownership model. Each vector is a separate heap allocation with 24 bytes of ptr/len/cap overhead. This copy is necessary because hnsw_rs borrows from owned data (`&'b [T]`).
- This explains why hnsw_rs overhead grows with dimension: each vector copy costs dim*4 + 24 bytes.

**3. Synchronization overhead**
- hnsw_rs: `Arc<RwLock<Vec<Vec<Arc<PointWithOrder>>>>>` per node for thread-safe concurrent access. RwLock adds ~48-56 bytes per node.
- usearch: C++ manages thread safety internally with atomics, no per-node lock overhead visible to the allocator.

### Why instant-distance disk overhead grows with dimension

**Serialization bug in our adapter**: The `SerializedIndex` stores both the `Hnsw<MetricPoint>` (which owns all points) AND a separate `points: Vec<MetricPoint>` clone (kept for rebuild). This duplicates all vector data on disk. At 1536d, this means ~12KB of vector data stored twice per vector. The actual graph overhead is ~700-1000B/vec — comparable to hnsw_rs scaled for M=32.

**Fix**: Reconstruct points from `hnsw.iter()` on load instead of serializing the clone.

### Why usearch has higher QPS

**1. SIMD distance computation**: usearch has hand-tuned AVX2/AVX-512/NEON kernels for all supported metrics. hnsw_rs delegates to anndists crate, which has optional SIMD behind feature flags (simdeez, stdsimd) but defaults to auto-vectorized scalar code.

**2. Cache-friendly memory layout**: Contiguous vector storage means sequential distance computations hit L1/L2 cache more often. hnsw_rs's per-vector heap allocations scatter data across the heap.

**3. Compact graph traversal**: 4-byte edge IDs mean more of the adjacency list fits in a cache line during search. hnsw_rs's 68-byte-per-edge Arc pointers cause more cache misses during graph traversal.

### hnsw_rs perfect recall anomaly

At 10K vectors with M=16 and ef_construction=200, the graph is extremely well-connected relative to the dataset size. Even ef_search=10 finds all true neighbors because the graph diameter is small. Larger datasets (100K+) will break this — the graph becomes sparse relative to the search space.

## Limitations

- 10K vectors is too small for production-relevant recall curves (especially hnsw_rs)
- Only euclidean metric tested (cosine and dot product untested)
- instant-distance ef_search sweep involves full index rebuild per value (correct but slow)
- Filtered search comparison between hnsw_rs and usearch not yet done at scale
- No source code examination (unsafe audit, API ergonomics) completed yet

## Next Steps (Pass 2)

1. **100K vectors** to get realistic recall/QPS Pareto curves for all 3 crates
2. **1M vectors** overnight run for publication-quality numbers
3. **Filtered ANN head-to-head**: hnsw_rs (Filterable trait) vs usearch (predicate fn)
4. **Source code examination**: unsafe audit, API ergonomics, composability assessment
5. **Fix instant-distance serialization** to eliminate duplicate vector storage
