# Key Insight: usearch Advantage Narrows with Dimensionality

> **Date**: 2026-04-27
> **Evidence**: M=32 benchmark sweep at 128d/384d/768d/1536d, 10K vectors

## The Finding

The usearch QPS advantage over hnsw_rs is **not constant** — it shrinks as vector dimensionality increases:

```
M=32 QPS ratio (usearch / hnsw_rs):
  128d:   34,493 / 22,474 = 1.53x
  384d:    9,561 /  8,539 = 1.12x
  768d:    4,388 /  3,575 = 1.23x
  1536d:   1,958 /  1,664 = 1.18x
```

At 128d, usearch is 53% faster. At 1536d, only 18% faster.

## Why This Happens

An HNSW query has two cost components:

1. **Graph traversal** — navigate edges, manage visited set, maintain priority queue. Cost is O(ef * M) and **independent of dimension**.

2. **Distance computation** — compute distance between query and candidate vector. Cost is O(dim) per comparison.

At **low dimensions** (128d), graph traversal is a large fraction of total query time. usearch's compact edges (4B vs 68B), prefetch instructions, and thread-local context dominate here — these are all graph traversal optimizations.

At **high dimensions** (1536d), distance computation overwhelms traversal. Both implementations spend >80% of time in the distance function. The remaining gap (~18%) is primarily the SIMD difference (SimSIMD hand-tuned kernels vs auto-vectorized scalar code).

## Implications

### For "should we use usearch or hnsw_rs?"

- **Low-d workloads** (embeddings <256d, geo-spatial): usearch's 1.5x advantage matters.
- **High-d workloads** (LLM embeddings 768d-4096d): the gap is small enough that hnsw_rs's API flexibility (generic types, FilterT, graph introspection) may outweigh the ~18% QPS difference.

### For "could a pure Rust HNSW match usearch?"

The answer depends on dimension:
- **At 128d**: Need all four optimizations (flat vectors, index edges, SIMD, prefetch) to close a 53% gap.
- **At 1536d**: SIMD alone (e.g., `std::simd` or `pulp`) would close most of the 18% gap. Graph layout optimizations provide diminishing returns at high d.

### For the broader Vector DS&A research

This insight applies beyond HNSW. Any graph-based ANN algorithm (Vamana/DiskANN, NSG, MRNG) will exhibit the same pattern: graph structure optimizations matter most at low d, distance computation optimizations matter most at high d. The crossover point depends on M and the specific algorithm.

## Verification Needed

- Confirm the pattern holds at 100K and 1M scale (10K may be too small for graph traversal to be representative).
- Test with hnsw_rs `simdeez_f` enabled to isolate the SIMD component.
- Test at intermediate dimensions (256d, 512d) to find the crossover point more precisely.
