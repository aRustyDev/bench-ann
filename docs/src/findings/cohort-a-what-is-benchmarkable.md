# Cohort A: What's Benchmarkable vs Rewrite-Only

> **Date**: 2026-04-27
> **Context**: Separating what we can test within the current harness from what requires a new implementation

## Benchmarkable Within Current Harness

These are changes to feature flags, adapter code, or configurations — no fork of upstream crates needed.

### 1. hnsw_rs with SIMD enabled

hnsw_rs has optional SIMD via the `simdeez_f` feature (x86_64) or `stdsimd` (nightly). We can:
- Rebuild `ann-bench-hnsw-rs` with `features = ["simdeez_f"]`
- Re-run the same benchmarks
- Measure the QPS delta — this isolates the SIMD contribution to the gap

**Expected impact**: +10-20% QPS, narrowing the usearch gap especially at high dimensions where distance computation dominates.

### 2. Fix instant-distance serialization

Our adapter stores vectors twice on disk (Hnsw internal copy + rebuild cache). We can:
- Reconstruct `points` from `hnsw.iter()` on load instead of serializing the clone
- Re-measure disk overhead — should drop to ~700-1000B/vec (from 860-6492B)

**Expected impact**: Disk overhead becomes constant across dimensions, matching hnsw_rs's ~630B profile.

### 3. usearch with higher ef_search values

At 100K, usearch maxes at recall 0.76 with ef=500. We can:
- Add ef_search=1000, 2000, 5000 to the sweep
- See if usearch can reach 0.95+ recall and at what QPS cost

**Expected impact**: Reveals whether usearch's recall ceiling is a fundamental graph quality issue (M=16 connectivity) or just insufficient ef_search budget.

### 4. usearch with ScalarKind::F16

usearch supports f16 quantized storage natively. We can:
- Add a build config variant with `quantization: ScalarKind::F16`
- Benchmark memory reduction vs recall impact

**Expected impact**: ~50% memory reduction for vector storage, some recall loss.

## NOT Benchmarkable — Requires New Implementation

These are architectural changes that would require forking hnsw_rs or writing a new HNSW from scratch.

### Flat vector buffer (Pattern #4)

Replacing hnsw_rs's `Vec<Vec<f32>>` with a flat `Vec<f32>` requires changing its internal `PointData` enum and insert/search code paths. This is a core library change, not an adapter change.

### Index-based edges (Pattern #2/3)

Replacing `Arc<PointWithOrder>` with `u32` indices requires rewriting hnsw_rs's entire graph representation, neighbor selection, and search loop. This is effectively a new HNSW implementation.

### Thread-local search context

Adding reusable per-thread search buffers (visited sets, candidate heaps) requires modifying hnsw_rs's search internals. Not achievable from the adapter layer.

### Prefetch instructions

Adding `_mm_prefetch` to the search loop requires modifying hnsw_rs's graph traversal code.

## Summary

| Change | Benchmarkable? | Expected QPS Impact | Expected Memory Impact |
|--------|---------------|--------------------|-----------------------|
| hnsw_rs + simdeez_f SIMD | **Yes** | +10-20% | None |
| Fix instant-distance serialization | **Yes** | None | -50-80% disk |
| usearch higher ef_search | **Yes** | Maps recall ceiling | None |
| usearch f16 quantization | **Yes** | ~same QPS | -50% vector memory |
| Flat vector buffer | No (fork) | +15-25% | Eliminates dim-scaling |
| Index-based edges | No (rewrite) | +5-10% | -80% graph memory |
| Thread-local context | No (fork) | +2-5% | None |
| Prefetch instructions | No (fork) | +3-5% | None |

**Recommendation**: Benchmark items 1-3 in Pass 2. They quantify the SIMD and recall-ceiling components of the gap with minimal effort. The architectural changes (flat buffer, index edges) are documented as the blueprint for a future "usearch-in-Rust" implementation.
