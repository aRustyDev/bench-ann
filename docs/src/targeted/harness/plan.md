# PLAN: Benchmark Harness Implementation

> **Bead**: `research-ola.3`
> **Status**: draft
> **Author**: aRustyDev + Claude
> **Date**: 2026-04-25
> **Spec**: `specs/benchmark-harness.md` (research-ola.2, approved)

## Prerequisites

- [x] Harness research complete (research-ola.1)
- [x] Harness spec approved (research-ola.2 + research-4wn)
- [ ] Rust toolchain installed (stable)
- [ ] SIFT1M dataset downloaded (`ftp://ftp.irisa.fr/local/texmex/corpus/sift.tar.gz`)

## Build Sequence

Implementation is ordered by dependency — each phase builds on the previous.

### Phase 1: Core Types & Trait (`ann-bench-core`)

**What**: Define the `AnnIndex` trait, types (`DistanceMetric`, `QueryResult`, `BuildConfig`, `QueryConfig`), and result structures.

**Files**:
```
ann-bench-core/
  Cargo.toml          # deps: serde, serde_json, anyhow
  src/
    lib.rs            # re-exports
    trait.rs          # AnnIndex trait + associated trait bounds
    types.rs          # DistanceMetric, QueryResult, BuildConfig, QueryConfig traits
    results.rs        # BenchmarkResult, QuerySweepResult, TierClassification structs
    tiers.rs          # tier_classify() — maps metric values to Unacceptable/Acceptable/Good/Excellent
    pareto.rs         # compute_pareto_frontier() — monotone filter on (recall, QPS)
```

**Checkpoint**: `cargo check` passes. Types are `Serialize`/`Deserialize`. Tier thresholds match metrics.md.

### Phase 2: Datasets (`ann-bench-datasets`)

**What**: fvecs loader, synthetic generator, ground truth computation with rayon.

**Files**:
```
ann-bench-datasets/
  Cargo.toml          # deps: ann-bench-core, rayon, rand, rand_distr, anyhow
  src/
    lib.rs
    fvecs.rs          # load_fvecs(), load_ivecs(), save_fvecs(), save_ivecs()
    synthetic.rs      # generate_synthetic() — unit sphere + Gaussian, fixed seeds
    ground_truth.rs   # compute_ground_truth() (rayon-parallel), save/load cache
    distance.rs       # euclidean(), cosine(), dot_product() — used by ground truth
```

**Checkpoint**: Load SIFT1M base vectors (1M × 128d). Generate 1M × 384d synthetic. Compute ground truth for 100 queries against 10K vectors — verify against manual brute force.

### Phase 3: Harness Runner (`ann-bench-harness`)

**What**: The measurement pipeline — build timing, QPS measurement, recall computation, memory measurement, latency distribution, result aggregation.

**Files**:
```
ann-bench-harness/
  Cargo.toml          # deps: ann-bench-core, ann-bench-datasets, memory-stats, anyhow, serde_json
  src/
    lib.rs
    runner.rs         # run_benchmark() — full pipeline: build → memory → sweep → filtered → incremental → pareto → output
    measurement.rs    # measure_qps(), measure_latency() — timing with warmup + repetitions
    recall.rs         # compute_recall() — distance-threshold method, recall@1/@10/@100
    memory.rs         # snapshot_rss() — wrapper around memory-stats
    filtered.rs       # run_filtered_benchmark() — filtered ANN pass (only if adapter supports it)
    incremental.rs    # run_incremental_benchmark() — insert/delete pass (only if adapter supports it)
    output.rs         # write_results_json() — serialize BenchmarkResult to JSON file, including inline pareto_frontier
```

**Checkpoint**: Wire up with a dummy adapter that returns random results. Verify: JSON output is valid and includes inline `pareto_frontier` array, tier classification works, memory snapshots produce sane values. `filtered.rs` and `incremental.rs` are testable once the dummy adapter declares `supports_filtered_search() = true`.

### Phase 4: CLI (`ann-bench-cli`)

**What**: Binary crate with clap-based CLI.

**Files**:
```
ann-bench-cli/
  Cargo.toml          # deps: ann-bench-core, ann-bench-harness, ann-bench-datasets, clap, anyhow
  src/
    main.rs           # clap command dispatch
    commands/
      mod.rs
      run.rs          # `ann-bench run` — run benchmarks for specified adapters
      ground_truth.rs # `ann-bench ground-truth` — precompute and cache
      pareto.rs       # `ann-bench pareto` — post-process result JSONs
      tier.rs         # `ann-bench tier` — classify results against thresholds
      list.rs         # `ann-bench list` — list adapters and datasets
```

**Checkpoint**: `ann-bench list` shows available adapters. `ann-bench ground-truth --dataset sift-128` computes and caches ground truth.

### Phase 5: First Adapter — hnsw_rs (`ann-bench-hnsw-rs`)

**What**: Validate the harness end-to-end with a real crate. hnsw_rs is chosen because it's pure Rust (no FFI complexity), Tier 1, supports filtered search, and has known benchmark results to sanity-check against.

**Files**:
```
adapters/ann-bench-hnsw-rs/
  Cargo.toml          # deps: ann-bench-core, hnsw_rs, anyhow, serde
  src/
    lib.rs            # AnnIndex impl for hnsw_rs
    config.rs         # HnswBuildConfig { M, ef_construction }, HnswQueryConfig { ef_search }
    sweep.rs          # default_sweep() — ef_search = [10, 20, 50, 100, 200, 500]
  README.md           # crate version, build prereqs, known limitations
```

**Implementation notes for hnsw_rs adapter**:
- `build()`: Create `Hnsw::new(M, max_elements, 16, ef_construction, dist_fn)`. Insert vectors via `parallel_insert` or sequential `insert`.
- `query()`: `search(vector, k, ef_search)`. Returns `Vec<Neighbour>` — extract (index, distance).
- `filtered_query()`: hnsw_rs supports a `Filterable` trait. Implement via `search_filter()` with predicate.
- `save()`/`load()`: hnsw_rs has `file_dump`/`load_hnsw` with `bincode` serialization.
- `prepare_query()`: No-op — hnsw_rs doesn't pre-allocate per-ef_search.

**Checkpoint**: `ann-bench run --adapter hnsw-rs --dataset sift-128 --metric euclidean` produces valid JSON. Recall@10 at ef_search=200 should be >0.95 (sanity check against known HNSW behavior on SIFT1M).

### Phase 6: Workspace Setup

**What**: Cargo workspace root tying it all together.

**File**: `benchmarks/vector-dsa/Cargo.toml`
```toml
[workspace]
resolver = "2"
members = [
    "ann-bench-core",
    "ann-bench-datasets",
    "ann-bench-harness",
    "ann-bench-cli",
    "adapters/ann-bench-hnsw-rs",
]
```

**Note**: Phase 6 is listed last but should be created first (empty workspace, then add members as they're built). The build sequence above describes the logical dependency order.

## Per-Adapter Notes (Remaining 14)

These are NOT built during the harness implementation phase. They are built per-cohort during Targeted Research. Notes here guide future adapter authors.

### Cohort A: NSW/HNSW

| Adapter | Key Considerations |
|---------|-------------------|
| **hnsw_rs** | Phase 5 (above). Pure Rust. Filtered via `Filterable` trait. |
| **usearch** | FFI to C++ via `usearch` crate. Build may need C++ compiler. Multi-precision (f32/f16/i8) — test f32 first. `filtered_search` uses predicate fn. |
| **instant-distance** | Pure Rust. Simple API. May lack filtered search. Check if `save`/`load` exists. |

### Cohort B: DiskANN

| Adapter | Key Considerations |
|---------|-------------------|
| **diskann** | Pure Rust. Needs temp directory for SSD files — `BuildConfig` must include `ssd_path: PathBuf`. Memory measurement tricky: PQ codes in RAM, graph on disk. RSS captures RAM portion only; disk portion measured via `save()` size. |
| **diskann-rs** | Alternative impl. Same considerations as diskann. Compare API surface. |
| **rust-diskann** | By jianshu93. Uses mmap. Similar to diskann. |

### Cohort C: Tree-based

| Adapter | Key Considerations |
|---------|-------------------|
| **arroy** | Needs LMDB environment — `BuildConfig` must include `lmdb_path: PathBuf` and `map_size: usize`. Filtered via RoaringBitmap. Writer/Reader lifecycle: build with `Writer`, query with `Reader`. `save()` is implicit (LMDB persistence). RSS includes mmap'd LMDB pages. |
| **kiddo** | Pure Rust. Exact kNN (not ANN) — recall will be 1.0 at all settings. Benchmark for QPS and dimensionality degradation. Low-d (<20d) is the sweet spot. Uses `NearestNeighbour` iterator. |
| **kd-tree** | Pure Rust. Simpler API than kiddo. Exact kNN. Same notes as kiddo. |
| **vpsearch** | Pure Rust. VP-tree with custom metric support. Interface may differ from others — check API. |

### Cohort D: Quantization

| Adapter | Key Considerations |
|---------|-------------------|
| **faiss** | FFI to C++ FAISS. Heavy build dependency (libfaiss, BLAS, LAPACK). Multiple index types — need to decide which to benchmark (IVFFlat, IVFPQ, HNSW at minimum). `BuildConfig` selects index type + params. |
| **rabitq-rs** | Pure Rust. x86_64 only — ARM64 has critical bugs. Skip on Apple Silicon. IVF+RaBitQ. Build config: nlist, nprobe. |
| **turbo-quant** | Algorithm library, not an index. Adapter wraps quantization + brute-force search on compressed vectors. Different from other adapters. |
| **turbovec** | Vector index built on TurboQuant. Very new — API may be unstable. |

### Cohort X: Utility

| Adapter | Key Considerations |
|---------|-------------------|
| **simsimd** | NOT an AnnIndex adapter. Separate benchmark: raw distance computation throughput at various precisions. Doesn't implement `AnnIndex`. Needs its own mini-harness or a separate CLI command. |

## Testing Strategy

1. **Unit tests in each crate**: fvecs round-trip, distance computation, recall computation, tier classification, Pareto frontier.
2. **Integration test with dummy adapter**: Full pipeline on synthetic 1K-vector dataset. Verify JSON output schema.
3. **Validation with hnsw_rs**: End-to-end on SIFT1M. Sanity-check recall@10 against known HNSW behavior (~0.95-0.99 at ef_search=100-500 on SIFT1M 128d).
4. **Memory measurement validation**: Build hnsw_rs index, verify RSS delta is positive and plausible (expect ~300-500 bytes/vector overhead for M=16).

## Checkpoints

| # | Milestone | Verification |
|---|-----------|-------------|
| 1 | Workspace compiles | `cargo check --workspace` |
| 2 | SIFT1M loads correctly | 1M vectors, 128d, first vector matches known values |
| 3 | Ground truth computes and caches | 10K queries × 1M base, k=100, cached to disk |
| 4 | Dummy adapter produces valid JSON | Schema validates, tiers computed, Pareto computed |
| 5 | hnsw_rs adapter runs end-to-end | recall@10 > 0.95 at ef_search=200 on SIFT1M |
| 6 | CLI commands all functional | `list`, `ground-truth`, `run`, `pareto`, `tier` |

## Risk Register

| Risk | Mitigation | Checkpoint |
|------|-----------|------------|
| hnsw_rs API changed since research | Pin version in Cargo.toml; check docs.rs before implementing | 5 |
| SIFT1M download fails | Fallback to HuggingFace mirror; document both sources | 2 |
| Ground truth computation too slow | rayon parallelism approved; cache aggressively | 3 |
| Memory measurement unreliable on CI | Primary validation on local hardware; document isolation requirements | 4 |

## Definition of Done

- [ ] All 6 checkpoints pass
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace` clean (no warnings)
- [ ] At least one complete benchmark run (hnsw_rs on SIFT1M) produces valid, plausible results
- [ ] JSON output is parseable and contains all fields from the spec schema
- [ ] README in workspace root documents: how to build, how to download SIFT1M, how to run first benchmark

## Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-04-25 | Initial plan — 6 build phases, per-adapter notes, testing strategy, checkpoints | aRustyDev + Claude |
| 2026-04-25 | Self-review fixes: added filtered.rs + incremental.rs to Phase 3, clarified Pareto is inline in JSON output, updated spec approval status | Claude |
