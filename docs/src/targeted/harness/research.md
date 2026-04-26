# Research: Benchmark Harness Patterns for ANN Crate Evaluation

> **Bead**: `research-ola.1`
> **Date**: 2026-04-25
> **Search matrix**: `targeted/harness/search-matrix.md`
> **Tier reached**: 1 (all acceptance and success criteria met)

## Purpose

Survey existing Rust benchmark infrastructure and ANN evaluation methodology to determine what to reuse vs build from scratch for a shared cross-cohort benchmark harness.

## Summary: Reuse vs Build

| Component | Recommendation | Rationale |
|-----------|---------------|-----------|
| Timing framework | **Build custom** + optional criterion `iter_custom` | criterion can't measure recall@k; divan has no JSON export; iai measures instructions not wall-clock. Custom harness needed for the three-metric problem (recall + QPS + memory). |
| Dataset loading (fvecs) | **Build from scratch** (~20 lines) | fvecs format is trivial (4-byte dim header + f32 data). Existing `fvecs_readers` crate is undocumented. No dependency needed. |
| Dataset loading (HDF5) | **Reuse** `hdf5` crate | ann-benchmarks datasets are HDF5. hnsw_rs already uses it. Requires libhdf5 native dep. Optional — fvecs alone is sufficient for SIFT1M. |
| Ground truth computation | **Build from scratch** | Brute-force exact kNN is ~30 lines. Cache to disk as ivecs (ann-benchmarks pattern). |
| Recall computation | **Build from scratch** (follow ann-benchmarks) | Use distance-threshold recall (not set-intersection). Handles near-ties at the k-th boundary. ~20 lines. |
| QPS measurement | **Build from scratch** | `std::time::Instant` for wall-clock. Min-over-runs (ann-benchmarks pattern). Single-threaded, sequential queries. |
| Memory measurement | **Reuse** `memory-stats` crate | Cross-platform RSS. Only approach that captures mmap'd memory (critical for arroy/LMDB, kiddo/mmap). |
| Parameter sweeping | **Build custom** (config-driven) | ann-benchmarks uses YAML configs per algorithm. Rust equivalent: serde config files or CLI args. |
| Result output | **Build custom** JSON + optional criterion HTML | Structured JSON for Pareto frontier generation and cross-cohort comparison. Criterion HTML for timing drill-down. |
| Pareto frontier | **Build from scratch** | Monotone filter on (recall, QPS) sorted tuples. ~20 lines. ann-benchmarks pattern. |

## Detailed Findings

### 1. Rust Benchmark Frameworks

**criterion.rs**: The established Rust benchmark framework. Provides statistical rigor (95% CI, outlier detection, warm-up), HTML reports, JSON output. Has `iter_custom` escape hatch that lets you control exactly what's measured and return a raw `Duration`. **Cannot** measure non-timing metrics like recall@k — the `Measurement` trait is explicitly timing-only. Best used for the QPS dimension only, not as the overall harness.

**divan**: Simpler API, clean parameterized syntax (`#[bench(args = [...])]`). Has `AllocProfiler` for heap allocation counting. **No JSON export**, no custom measurement API. Insufficient for ANN harness needs.

**iai-callgrind**: Instruction-count based (deterministic, not wall-clock). Useful for CI regression detection but irrelevant for QPS measurement. Cannot measure recall.

**Decision**: Custom harness that handles the three-metric problem (recall + QPS + memory) as a unified pipeline. Optionally wrap the QPS measurement in criterion's `iter_custom` for statistical rigor on timing. The harness orchestrates; criterion measures one dimension.

### 2. ann-benchmarks Methodology

The reference standard for ANN evaluation. Key patterns to adopt:

**Pipeline**: Dataset load → index build (timed separately) → query sweep (vary search params at fixed build params) → compute recall + QPS per parameter setting → plot Pareto frontier.

**Recall computation**: Distance-threshold recall, NOT set-intersection.
```
threshold = ground_truth_distances[i][k-1] + epsilon  (epsilon = 1e-3)
count matches where ANN_distance <= threshold
recall@k = mean(matches) / k
```
This handles near-ties at the k-th boundary gracefully. More robust than `|ANN_k ∩ exact_k| / k`.

**QPS**: Single-threaded sequential queries. Time the full query pass. Repeat `run_count` times, take **minimum** (best case, not mean — avoids OS scheduling noise). QPS = num_queries / best_total_time.

**Parameter sweeping**: Build index once with build-time params. Then loop over query-time params (e.g., ef_search), calling `set_query_arguments()` before each query pass. Each setting produces one (recall, QPS) point on the scatter plot.

**Pareto frontier**: Sort all (recall, QPS) points. Walk in recall-descending order, keeping a point only if it improves the best QPS seen so far. The frontier is the "up and to the right" boundary.

**Datasets**: HDF5 format with `train`, `test`, `neighbors` (int, k=100), `distances` (float) arrays. Canonical SIFT1M sourced from IRISA fvecs, converted to HDF5.

### 3. Existing Benchmark Code in Crate Repos

| Repo | Framework | End-to-end (recall+QPS)? | Dataset | Reusable? |
|------|-----------|-------------------------|---------|-----------|
| hnsw_rs | Custom (examples/) | Yes — recall@k + QPS | HDF5 (ann-benchmarks) | **Yes**: `annhdf5::AnnBenchmarkData` loader, recall computation |
| usearch | C++ custom | Yes, but C++ only | fvecs (C++), synthetic (Python) | No Rust code |
| kiddo | criterion (benches/) | QPS only (exact kNN, no recall) | Synthetic | **Partial**: `Throughput::Elements` idiom, parameterized sweep macro |
| arroy | None (examples/) | No — manual timing, no recall | Synthetic (4K vectors) | No |
| diskann | criterion (benches/) | Primitives only (distance, queue, k-means) | Synthetic + SIFT small (test data) | **Partial**: criterion config patterns, SIFT test fixture |

**Key reusable patterns**:
- hnsw_rs `annhdf5` module: HDF5 loader for ann-benchmarks format (train/test/ground-truth). Clean struct-based API. Worth studying, but we should implement our own to avoid taking a dependency on hnsw_rs for the harness.
- kiddo `batch_benches!` macro: Expands (type, size, dimension) combinations into criterion benchmark groups. Pattern transferable to our parameter sweep.
- kiddo `Throughput::Elements`: Standard criterion idiom for reporting QPS.

### 4. SIFT1M Dataset Format

**fvecs format** (simple binary, no library needed):
```
Per vector: [dim: i32 LE] [v[0]: f32 LE] ... [v[dim-1]: f32 LE]
No global file header. Dimension repeated per vector.
```

| Variant | Component type | Bytes per vector |
|---------|---------------|-----------------|
| fvecs | f32 | 4 + dim*4 |
| ivecs | i32 | 4 + dim*4 |
| bvecs | u8 | 4 + dim |

**SIFT1M contents** (from `ftp://ftp.irisa.fr/local/texmex/corpus/sift.tar.gz`):
- `sift_base.fvecs`: 1M base vectors, 128-dim (~500 MB)
- `sift_query.fvecs`: 10K query vectors, 128-dim (~5 MB)
- `sift_groundtruth.ivecs`: Top-100 neighbor IDs per query (~4 MB)
- `sift_learn.fvecs`: 100K training vectors (~50 MB)

**Note**: Ground truth contains neighbor indices only, not distances. For distance-threshold recall (ann-benchmarks style), we need to compute distances ourselves from the ground truth indices + base vectors.

**Implementation**: ~20 lines of Rust. Read i32 dim header, then `dim` f32 values per vector. Little-endian. No existing crate worth depending on.

### 5. Memory Measurement

| Approach | mmap coverage | Precision | Cross-platform | Dependency |
|----------|--------------|-----------|----------------|------------|
| `memory-stats` (RSS) | **Yes** | OS-level | macOS + Linux + Windows | `memory-stats = "1"` |
| `cap` (allocator tracking) | No | Exact heap bytes | Yes | `cap = "0.1"` |
| jemalloc `allocated` | No | Exact jemalloc heap | Linux + macOS | `tikv-jemallocator` + `tikv-jemalloc-ctl` |
| jemalloc `resident` | Partial | jemalloc's RSS view | Linux + macOS | Same |

**Critical for our harness**: arroy uses LMDB (mmap-based), kiddo supports mmap persistence. Allocator-tracking approaches will **significantly undercount** memory for these crates because mmap bypasses `GlobalAlloc`.

**Decision**: `memory-stats` (RSS) as primary measurement. The protocol from metrics.md is sound:
```
memory_per_vector = (RSS_after_build - RSS_before_build - raw_data_bytes) / N
```

Optionally supplement with jemalloc `allocated` as a secondary signal to separate heap fragmentation from mmap overhead. The delta between RSS and heap-allocated is itself useful diagnostic data.

## Harness Architecture Recommendation

Based on these findings, the harness should be structured as:

```
benchmarks/vector-dsa/
  core/          # AnnIndex trait, measurement framework, result types
  datasets/      # fvecs loader, synthetic generator, ground truth computation
  harness/       # Runner: build → sweep → measure → output
  adapters/      # Per-crate thin wrappers implementing AnnIndex
    hnsw-rs/
    usearch/
    kiddo/
    arroy/
    diskann/
    ...
  cli/           # CLI binary for running benchmarks
```

**Core trait** (sketch):
```rust
trait AnnIndex {
    fn build(vectors: &[Vec<f32>], params: &BuildParams) -> Self;
    fn query(&self, vector: &[f32], k: usize, params: &QueryParams) -> Vec<(usize, f32)>;
    fn filtered_query(&self, vector: &[f32], k: usize, params: &QueryParams, filter: &dyn Fn(usize) -> bool) -> Vec<(usize, f32)>;
    fn set_query_params(&mut self, params: &QueryParams);
    fn serialize(&self, path: &Path) -> Result<()>;
    fn name(&self) -> &str;
}
```

**Runner pipeline** (per algorithm, per parameter set):
1. Load dataset (fvecs or synthetic)
2. Compute ground truth (brute-force, cache to disk)
3. Snapshot RSS before build
4. Build index (time it)
5. Snapshot RSS after build → compute memory_per_vector
6. For each query-param setting in sweep:
   a. Run query pass (10K queries, sequential, single-threaded)
   b. Repeat `run_count` times (3), take min total time
   c. Compute recall@k (distance-threshold method)
   d. Compute QPS = num_queries / best_total_time
   e. Record latency distribution (p50, p99)
7. Serialize index → measure disk size
8. Output: JSON with all metrics per (build_params, query_params) point
9. Post-process: compute Pareto frontier

**Output JSON** (sketch):
```json
{
  "crate": "hnsw_rs",
  "dataset": "sift-128-euclidean",
  "dimension": 128,
  "n_vectors": 1000000,
  "build_params": {"M": 16, "ef_construction": 200},
  "build_time_s": 45.2,
  "memory_per_vector_bytes": 312,
  "index_size_bytes": 312000000,
  "results": [
    {"query_params": {"ef_search": 10}, "recall_at_10": 0.85, "qps": 15230, "p50_us": 62, "p99_us": 180},
    {"query_params": {"ef_search": 50}, "recall_at_10": 0.96, "qps": 5100, "p50_us": 185, "p99_us": 520},
    ...
  ]
}
```

## Dependencies (minimal)

| Crate | Purpose | Required? |
|-------|---------|-----------|
| `memory-stats` | RSS measurement | Yes |
| `serde` + `serde_json` | JSON output | Yes |
| `rand` + `rand_distr` | Synthetic dataset generation | Yes |
| `hdf5` | ann-benchmarks HDF5 loading | Optional (fvecs sufficient for SIFT1M) |
| `criterion` | Statistical QPS timing (via `iter_custom`) | Optional |
| `clap` | CLI argument parsing | Yes |
| `tikv-jemallocator` | Supplementary memory stats | Optional |

## Risks Identified

1. **HDF5 native dependency**: The `hdf5` crate requires libhdf5 system library. May complicate builds. Mitigate by making HDF5 support a cargo feature, defaulting to fvecs-only.
2. **RSS measurement noise**: RSS includes OS page cache, shared libraries, etc. Mitigate by taking before/after snapshots (delta) and running on a quiescent system.
3. **SIFT1M ground truth lacks distances**: Need to compute distances from indices + base vectors for distance-threshold recall. Small upfront cost, cache alongside ground truth.
4. **Adapter complexity varies**: Some crates (faiss FFI, usearch FFI) may need complex build configurations. Mitigate by making adapters optional cargo features.

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-25 | Initial research findings — all Tier 1 queries successful | Claude |
