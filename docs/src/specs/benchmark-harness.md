# SPEC: Shared Benchmark Harness — Vector DS&A

> **Bead**: `research-ola.2`
> **Status**: approved
> **Author**: aRustyDev + Claude
> **Reviewer**: aRustyDev
> **Date**: 2026-04-25
> **Approved**: 2026-04-25
> **Research**: `targeted/harness/research.md` (research-ola.1)

## Purpose

Specify the shared benchmark harness for cross-cohort ANN crate evaluation. The harness is a Rust workspace that provides datasets, ground truth, measurement infrastructure, and a common trait interface so that all 15 crates across 5 cohorts produce directly comparable results.

This spec defines WHAT the harness does and what interfaces it exposes. The implementation plan (research-ola.3) defines HOW to build it.

## Scope

### In Scope

- `AnnIndex` trait definition and adapter contract
- Dataset loading (fvecs format) and synthetic generation
- Ground truth computation and caching
- Measurement protocols for all 6 primary metrics + 4 secondary metrics (recall@1/@100 computed alongside primary recall; dimensionality sensitivity, filtered ANN, incremental updates as dedicated protocols)
- Result output schema (JSON)
- Pareto frontier computation
- CLI interface for running benchmarks
- Tier classification against metrics.md thresholds

### Out of Scope

- Adapter implementations for specific crates (those are built per cohort)
- Benchmark result analysis or visualization beyond Pareto frontier data
- CI integration or automated regression detection
- HDF5 dataset loading (deferred to cargo feature if needed later)
- Multi-threaded benchmark modes (single-threaded is primary; multi-threaded is future work)

### Assumptions

- Target hardware: Apple M-series or AMD Zen 4, single core, >=16 GB RAM, NVMe SSD
- Primary benchmark scale: 1M vectors, 10K queries
- All adapters run in the same process (no Docker isolation — unlike ann-benchmarks)
- fvecs format is sufficient for SIFT1M; HDF5 not needed initially

## Specification

### 1. Workspace Structure

```
benchmarks/vector-dsa/
  Cargo.toml              # workspace root
  ann-bench-core/         # trait, types, measurement, Pareto, tier classification
  ann-bench-datasets/     # fvecs loader, synthetic generator, ground truth
  ann-bench-harness/      # runner pipeline, result aggregation
  ann-bench-cli/          # binary: CLI for running benchmarks
  adapters/               # per-crate adapter crates (added incrementally)
    ann-bench-hnsw-rs/
    ann-bench-usearch/
    ann-bench-kiddo/
    ann-bench-arroy/
    ann-bench-diskann/
    ...
```

Each adapter is an independent crate in the workspace so that native dependencies (libhdf5, C++ FFI) are isolated as cargo features.

### 2. Core Trait: `AnnIndex`

```rust
use std::path::Path;

/// Distance metric for vector similarity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistanceMetric {
    Euclidean,       // L2
    Cosine,          // cosine similarity (pre-normalize + inner product)
    DotProduct,      // inner product (MIPS)
}

/// Opaque build-time parameters, serialized as JSON for output.
/// Each adapter defines its own concrete type that implements this.
pub trait BuildConfig: Serialize + std::fmt::Debug {
    fn name(&self) -> &str;  // e.g., "M=16,ef_c=200"
}

/// Opaque query-time parameters, serialized as JSON for output.
/// Each adapter defines its own concrete type that implements this.
pub trait QueryConfig: Serialize + std::fmt::Debug {
    fn name(&self) -> &str;  // e.g., "ef_search=50"
}

/// Query result: (vector_index, distance).
pub type QueryResult = (usize, f32);

/// Core trait that every crate adapter must implement.
pub trait AnnIndex: Sized {
    type Build: BuildConfig;
    type Query: QueryConfig;

    /// Build an index from contiguous vector data.
    /// `vectors` is a flat buffer of n*dim f32 values, row-major.
    fn build(
        vectors: &[f32],
        n: usize,
        dim: usize,
        metric: DistanceMetric,
        config: &Self::Build,
    ) -> anyhow::Result<Self>;

    /// Optional: pre-allocate internal buffers for the given query config.
    /// Called once before a query pass. Default: no-op.
    /// Adapters that maintain state based on query params (e.g., visited-set
    /// buffers sized to ef_search) should implement this.
    fn prepare_query(&mut self, _config: &Self::Query) {}

    /// Query the index for the k nearest neighbors.
    /// Returns results sorted by distance (ascending).
    fn query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
    ) -> anyhow::Result<Vec<QueryResult>>;

    /// Query with a filter predicate. Only vectors where `filter(id)` returns
    /// true are eligible results. Returns an error if filtered search is
    /// not supported.
    fn filtered_query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
        filter: &dyn Fn(usize) -> bool,
    ) -> anyhow::Result<Vec<QueryResult>> {
        // Default: not supported
        Err(anyhow::anyhow!("filtered search not supported"))
    }

    /// Serialize the index to disk. Returns the total bytes written.
    fn save(&self, path: &Path) -> anyhow::Result<u64>;

    /// Load a serialized index from disk.
    fn load(path: &Path, metric: DistanceMetric) -> anyhow::Result<Self>;

    /// Human-readable crate name (e.g., "hnsw_rs").
    fn crate_name(&self) -> &str;

    /// Whether this adapter supports filtered queries.
    fn supports_filtered_search(&self) -> bool { false }

    /// Whether this adapter supports incremental insert/delete.
    fn supports_incremental_updates(&self) -> bool { false }

    /// Insert vectors into an existing index (if supported).
    /// `vectors` is a flat buffer of n*dim f32 values.
    fn insert(&mut self, _vectors: &[f32], _n: usize, _dim: usize, _start_id: usize) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("incremental insert not supported"))
    }

    /// Delete vectors from an existing index by ID (if supported).
    fn delete(&mut self, _ids: &[usize]) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("incremental delete not supported"))
    }
}
```

**Design decisions**:
- Associated types `Build` and `Query` allow each adapter to define its own parameter types with full type safety, while still serializing to JSON for the output schema.
- `filtered_query` and `insert`/`delete` have default "not supported" implementations so adapters only implement what their crate supports.
- `vectors: &[f32]` with `n` and `dim` parameters — contiguous flat buffer is what most ANN crates want internally (SIMD-friendly). The runner converts from `Vec<Vec<f32>>` once; adapters don't need intermediate allocations.
- `prepare_query` with default no-op — adapters that pre-allocate per-config state (e.g., visited-set buffers) can implement this, called once per query-param setting before the query pass.
- `anyhow::Result` for error handling — benchmarks should report errors, not panic.
- **Adapter lifecycle**: Adapters carry their environment in `Self` (LMDB env handle, temp dir path, etc.). `BuildConfig` may include infrastructure paths (e.g., `lmdb_path`, `ssd_dir`) alongside algorithm params. Cleanup happens via `Drop`. This keeps the trait simple at the cost of mixing infra and algo params in the config — acceptable for a benchmark harness where configs are defined per-adapter, not shared.

### 3. Dataset Specification

#### 3.1 fvecs Loader

```rust
/// Load vectors from an fvecs file.
/// Format: per vector, [dim: i32 LE] [v[0]: f32 LE] ... [v[dim-1]: f32 LE]
/// No global header. Dimension is repeated per vector.
pub fn load_fvecs(path: &Path) -> anyhow::Result<(usize, Vec<Vec<f32>>)>;
// Returns (dimension, vectors)

/// Load ground truth indices from an ivecs file.
pub fn load_ivecs(path: &Path) -> anyhow::Result<Vec<Vec<i32>>>;
```

Validation: first vector's dimension must match all subsequent vectors. Error on dimension mismatch.

#### 3.2 Synthetic Generator

```rust
/// Generate synthetic vectors for benchmarking.
pub struct SyntheticDataset {
    pub vectors: Vec<Vec<f32>>,
    pub queries: Vec<Vec<f32>>,
    pub dimension: usize,
    pub seed: u64,
}

/// Distribution for synthetic vector generation.
pub enum Distribution {
    /// Uniform on unit sphere (for cosine/inner product benchmarks).
    UnitSphere,
    /// Multivariate Gaussian, identity covariance (for L2 benchmarks).
    Gaussian,
}

pub fn generate_synthetic(
    n_vectors: usize,
    n_queries: usize,
    dimension: usize,
    distribution: Distribution,
    seed: u64,
) -> SyntheticDataset;
```

**Fixed seeds for reproducibility**:

| Configuration | Seed (u64) |
|--------------|------------|
| 128d unit sphere | `42_000_128` |
| 384d unit sphere | `42_000_384` |
| 768d unit sphere | `42_000_768` |
| 1536d unit sphere | `42_001_536` |
| 128d Gaussian | `43_000_128` |
| 384d Gaussian | `43_000_384` |
| 768d Gaussian | `43_000_768` |
| 1536d Gaussian | `43_001_536` |

Pattern: `42_xxx_xxx` for unit sphere, `43_xxx_xxx` for Gaussian. The last digits encode the dimension.

#### 3.3 Ground Truth Computation

```rust
/// Compute exact k nearest neighbors via brute force.
/// Parallelized with rayon (one query per thread). Deterministic output.
/// Returns: for each query, a Vec of (neighbor_index, distance) sorted by distance.
pub fn compute_ground_truth(
    base: &[f32],       // contiguous, n_base * dim
    n_base: usize,
    queries: &[f32],    // contiguous, n_queries * dim
    n_queries: usize,
    dim: usize,
    k: usize,
    metric: DistanceMetric,
) -> Vec<Vec<(usize, f32)>>;

/// Save ground truth to disk (ivecs for indices, fvecs for distances).
pub fn save_ground_truth(gt: &[Vec<(usize, f32)>], dir: &Path) -> anyhow::Result<()>;

/// Load cached ground truth from disk.
pub fn load_ground_truth(dir: &Path) -> anyhow::Result<Vec<Vec<(usize, f32)>>>;
```

Ground truth is computed with k=100 (like ann-benchmarks) so it can serve recall@1, recall@10, and recall@100 without recomputation. Cached to disk alongside the dataset.

**SIFT1M note**: SIFT1M ground truth includes neighbor indices but NOT distances. The harness must compute distances from indices + base vectors on first load, then cache the (index, distance) pairs.

### 4. Measurement Protocols

#### 4.1 Recall@k (Primary)

**Method**: Distance-threshold recall (ann-benchmarks pattern).

```
For each query i:
  threshold = ground_truth_distances[i][k-1] + EPSILON
  matches = count(ann_results where distance <= threshold)
  recall_i = matches / k

recall@k = mean(recall_i) over all queries
```

Where `EPSILON = 1e-3`.

**Rationale**: Distance-threshold handles near-ties at the k-th boundary better than set-intersection (`|ANN_k ∩ exact_k| / k`). Two vectors at nearly identical distances may swap ordering between exact and approximate computation without indicating a real quality difference.

**Deviation from metrics.md**: metrics.md (section 1) specifies set-intersection recall. This spec adopts distance-threshold recall following ann-benchmarks methodology. Distance-threshold is more robust at boundary conditions and is the industry standard. metrics.md should be amended to align — this is a refinement (Rule #10), not an expansion.

Report recall@1, recall@10 (primary), and recall@100 for each parameter setting.

#### 4.1b Recall@1 and Recall@100 (Secondary)

Computed alongside recall@10 using the same distance-threshold method. No separate measurement pass needed — the ground truth is computed at k=100, so recall@1 and recall@100 are free to compute during the same query result evaluation. Reported in the same JSON output alongside recall@10.

- **Recall@1**: Top-1 accuracy. Noisy (single result) but important for re-ranking pipelines.
- **Recall@100**: Broad retrieval quality. Higher than recall@10 for all algorithms.

#### 4.2 QPS — Queries Per Second (Primary)

**Protocol**:
1. Set query parameters (e.g., `ef_search`)
2. Run all 10K queries sequentially, single-threaded
3. Time the full pass with `std::time::Instant` (wall-clock)
4. Repeat 3 times (measurement runs) after 1 warmup run (discarded)
5. Take the **minimum** total time across measurement runs
6. QPS = `n_queries / min_total_time_seconds`

**Why minimum**: Eliminates OS scheduling noise, GC pauses, background process interference. ann-benchmarks uses the same approach.

**Deviation from metrics.md**: metrics.md (section 2) specifies reporting the **median** over 3 runs. This spec adopts **minimum** following ann-benchmarks methodology. The minimum better represents achievable performance by removing OS interference. All 3 run times are recorded in the JSON output (`run_times_s`), so median can also be computed from the raw data. metrics.md should be amended to align — this is a refinement (Rule #10), not an expansion.

#### 4.3 Build Time (Primary)

**Protocol**:
1. Load dataset into memory (do NOT include load time)
2. Start timer (`Instant::now()`)
3. Call `AnnIndex::build()`
4. Stop timer
5. Single run (no repetitions — build is deterministic for most algorithms)
6. Report wall-clock seconds

#### 4.4 Memory Per Vector (Primary)

**Protocol**:
1. Load dataset into memory
2. Snapshot RSS: `rss_before = memory_stats().physical_mem`
3. Build index
4. Snapshot RSS: `rss_after = memory_stats().physical_mem`
5. `raw_data_bytes = n_vectors * dimension * 4` (f32)
6. `memory_per_vector = (rss_after - rss_before - raw_data_bytes) / n_vectors`

**Tool**: `memory-stats` crate (cross-platform RSS, captures mmap'd memory).

**Note**: If `memory_per_vector` is negative (compressed representation), report as negative — this is valid and meaningful (e.g., PQ compression below raw size).

#### 4.5 Index Size on Disk (Primary)

**Protocol**:
1. Build index
2. Call `AnnIndex::save(path)`
3. `disk_per_vector = file_size_bytes / n_vectors`

#### 4.6 Latency Distribution (Primary)

**Protocol**: Measured during the QPS query pass.
1. Record per-query wall-clock duration for each of the 10K queries in the best run
2. Sort durations
3. Report p50 (median) and p99

#### 4.7 Dimensionality Sensitivity (Secondary)

**Protocol**: Run the full benchmark at 128d, 384d, 768d, 1536d.
Report `qps_ratio = QPS_at_1536d / QPS_at_128d` (at recall@10 >= 0.95 for both).

#### 4.8 Filtered ANN Performance (Secondary)

**Only for**: Adapters where `supports_filtered_search() == true`.

**Protocol**:
1. Assign each vector a categorical attribute with cardinality C
2. Test C = 10, 100, 1000
3. Apply filter selecting ~10% of dataset
4. Run `filtered_query` with same parameters as unfiltered baseline
5. Report filtered recall@10, filtered QPS, and delta from unfiltered

**Attribute assignment**: Deterministic based on vector index: `category = index % C`. Filter predicate: `|id| id % C == 0` (selects ~10% when C=10).

#### 4.9 Incremental Update Performance (Secondary)

**Only for**: Adapters where `supports_incremental_updates() == true`.

**Protocol**:
1. Build index on first 900K vectors
2. Insert next 100K vectors via `insert()`
3. Delete first 100K vectors via `delete()`
4. Measure recall@10 and QPS on the 900K-vector post-update index
5. Compare against a fresh build on the same 900K vectors (vectors 100K..1M)

### 5. Result Output Schema

All results are written as JSON. One file per (crate, dataset, build_config) combination.

```json
{
  "schema_version": "1.0",
  "harness_version": "0.1.0",
  "timestamp": "2026-04-25T12:00:00Z",
  "hardware": {
    "cpu": "Apple M4 Pro",
    "cores_used": 1,
    "ram_gb": 36,
    "os": "macOS 15.4",
    "storage": "NVMe SSD"
  },
  "crate_name": "hnsw_rs",
  "crate_version": "0.3.0",
  "dataset": {
    "name": "sift-128-euclidean",
    "source": "fvecs",
    "n_vectors": 1000000,
    "n_queries": 10000,
    "dimension": 128,
    "metric": "euclidean"
  },
  "build": {
    "config": {"M": 16, "ef_construction": 200},
    "time_s": 45.2,
    "memory_per_vector_bytes": 312,
    "rss_before_bytes": 536870912,
    "rss_after_bytes": 849346560
  },
  "index_size": {
    "disk_bytes": 312000000,
    "disk_per_vector_bytes": 312
  },
  "query_sweeps": [
    {
      "config": {"ef_search": 10},
      "recall_at_1": 0.72,
      "recall_at_10": 0.85,
      "recall_at_100": 0.91,
      "qps": 15230,
      "latency_p50_us": 62,
      "latency_p99_us": 180,
      "run_times_s": [0.657, 0.661, 0.659],
      "best_run_s": 0.657
    },
    {
      "config": {"ef_search": 50},
      "recall_at_1": 0.95,
      "recall_at_10": 0.96,
      "recall_at_100": 0.98,
      "qps": 5100,
      "latency_p50_us": 185,
      "latency_p99_us": 520,
      "run_times_s": [1.961, 1.965, 1.970],
      "best_run_s": 1.961
    }
  ],
  "filtered": null,
  "incremental": null,
  "tier_classification": {
    "recall_at_10": "good",
    "qps_at_0_95_recall": "good",
    "build_time": "acceptable",
    "memory_per_vector": "good",
    "disk_per_vector": "acceptable",
    "latency_p99": "good"
  }
}
```

**Tier classification** follows the thresholds defined in `runs/run-2/metrics.md`. The harness computes these automatically:

| Metric | Unacceptable | Acceptable | Good | Excellent |
|--------|-------------|------------|------|-----------|
| recall@10 | <0.90 | 0.90–0.95 | 0.95–0.99 | >=0.99 |
| QPS (at recall>=0.95, 128d 1M) | <100 | 100–1K | 1K–10K | >10K |
| Build time (1M 128d single-thread) | >600s | 60–600s | 10–60s | <10s |
| Memory/vector (128d f32) | >2KB | 512B–2KB | 128–512B | <128B |
| Disk/vector (128d f32) | >4KB | 1–4KB | 256B–1KB | <256B |
| Latency p99 (1M 128d) | >100ms | 10–100ms | 1–10ms | <1ms |

#### Pareto Frontier

Computed per (crate, dataset, build_config):
1. Collect all (recall@10, QPS) points from the query sweep
2. Sort by recall descending; break ties by QPS descending
3. Initialize `max_qps = 0`. Walk the sorted list: keep a point if and only if `point.qps > max_qps`, then update `max_qps = point.qps`
4. The resulting frontier has monotonically non-decreasing QPS as recall decreases
5. Output as `pareto_frontier` array in the JSON:
```json
"pareto_frontier": [
  {"recall_at_10": 0.99, "qps": 1200},
  {"recall_at_10": 0.96, "qps": 5100},
  {"recall_at_10": 0.85, "qps": 15230}
]
```

### 6. CLI Interface

```
ann-bench [OPTIONS] <COMMAND>

COMMANDS:
  run          Run benchmarks for specified adapters
  ground-truth Compute and cache ground truth for a dataset
  pareto       Compute Pareto frontiers from result JSON files
  tier         Classify results against metric tier thresholds
  list         List available adapters and datasets

OPTIONS:
  --dataset <NAME>       Dataset name (sift-128, synthetic-384, etc.)
  --adapter <NAME>       Adapter to benchmark (hnsw-rs, usearch, etc.)
  --dimension <D>        Dimension for synthetic datasets
  --n-vectors <N>        Number of vectors (default: 1000000)
  --n-queries <N>        Number of queries (default: 10000)
  --metric <METRIC>      Distance metric: euclidean, cosine, dot (default: euclidean)
  --output-dir <PATH>    Output directory for result JSON files
  --runs <N>             Number of measurement runs (default: 3)
  --k <K>                Number of neighbors to retrieve (default: 10)
  -v, --verbose          Verbose output
```

Example usage:
```bash
# Compute ground truth for SIFT1M
ann-bench ground-truth --dataset sift-128 --metric euclidean --k 100

# Run benchmark for hnsw_rs at all dimensionalities
ann-bench run --adapter hnsw-rs --dataset synthetic-128 synthetic-384 synthetic-768 synthetic-1536 --metric cosine

# Run full evaluation for a cohort
ann-bench run --adapter hnsw-rs usearch instant-distance --dataset sift-128 --metric euclidean cosine
```

### 7. Adapter Contract

Each adapter crate must:

1. **Implement `AnnIndex`** with concrete `Build` and `Query` associated types
2. **Declare a default parameter sweep** — at minimum 5 query-param settings that span the recall range (e.g., ef_search = [10, 20, 50, 100, 200, 500] for HNSW)
3. **Declare supported features** via the `supports_*` methods
4. **Handle errors gracefully** — return `Err`, never panic
5. **Be a separate workspace crate** — native dependencies isolated behind cargo features
6. **Include a `README.md`** noting: crate version tested, any build prerequisites (e.g., `apt install libhdf5-dev`), known limitations

Adapters do NOT need to implement `filtered_query` or `insert`/`delete` unless their crate supports these operations.

## Success Criteria

- [ ] `AnnIndex` trait is implementable for all 15 target crates without trait workarounds or hacks
- [ ] At least one adapter (hnsw_rs) builds, runs, and produces valid JSON output
- [ ] Recall@10 computation matches brute-force verification (±0.001)
- [ ] QPS measurement is reproducible (±5% across runs on quiescent hardware)
- [ ] Memory measurement captures mmap'd memory (verified with arroy/LMDB or kiddo/mmap adapter)
- [ ] SIFT1M fvecs loading produces vectors matching known dimensions (128d, 1M vectors)
- [ ] Pareto frontier computation produces monotonically non-decreasing QPS for decreasing recall
- [ ] Tier classification matches manual classification against metrics.md thresholds
- [ ] CLI supports all documented commands and options

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `AnnIndex` trait too restrictive for some crates (e.g., arroy needs LMDB env, diskann needs disk paths) | Medium | High | Allow adapters to carry setup state in `Self`; `build()` receives opaque config that can include paths/env handles |
| RSS measurement noisy on systems with background processes | Medium | Medium | Document isolation requirements; take before/after delta; report all 3 run values not just min |
| FFI adapter build failures on different platforms | Medium | Medium | Isolate each adapter as a cargo feature; CI tests each adapter independently |
| SIFT1M FTP server unavailable | Low | Low | Mirror on HuggingFace (qbo-odp/sift1m); document fallback |
| Ground truth computation slow for 1M×10K at high dimensions | Low | Low | Parallelized with rayon; cached to disk. Compute once per (dataset, k, metric) triple. |

## Open Questions

*(none remaining)*

### Resolved

- ~~**Adapter lifecycle**~~: Adapters carry their environment in `Self` (LMDB env handle, temp dir path, etc.). `BuildConfig` may include infrastructure paths alongside algorithm params. Cleanup via `Drop`. This keeps the trait simple. Documented in Design Decisions section. *(resolved 2026-04-25)*
- ~~**metrics.md deviations**~~: Two refinements adopted from ann-benchmarks methodology: (a) distance-threshold recall instead of set-intersection, (b) min-over-runs QPS instead of median. Both documented in measurement protocol sections with rationale. Raw data for both methods is retained in JSON output. metrics.md amendment tracked separately. *(resolved 2026-04-25)*
- ~~**Rayon for ground truth**~~: Yes — use rayon for brute-force ground truth computation. This is a one-time preprocessing step that runs before any measurement. Output is deterministic (same neighbors, same distances regardless of thread count). Rayon dependency is isolated to `ann-bench-datasets` crate, does not affect the measurement path or adapter builds. No experimental complexity introduced — parallelism is only in preprocessing, never during measurement. *(resolved 2026-04-25)*

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-25 | Initial draft — formalized from research-ola.1 findings | aRustyDev + Claude |
| 2026-04-25 | Self-review fixes: resolved adapter lifecycle (Self+Drop), acknowledged metrics.md deviations (recall method, QPS aggregation), added prepare_query(), changed vectors to contiguous &[f32], added recall@1/@100 section, fixed Pareto algorithm description, added pareto_frontier to JSON schema, fixed seed naming | Claude |
| 2026-04-25 | Resolved rayon for ground truth (yes — preprocessing only, no experimental impact). All open questions now resolved. | Claude |
