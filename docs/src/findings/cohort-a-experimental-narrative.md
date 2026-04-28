# Cohort A: Experimental Narrative — From Hypothesis to Bug Discovery

> **Date range**: 2026-04-26 to 2026-04-27
> **Purpose**: Complete epistemic record of the Cohort A benchmark experiment for blog post drafting
> **Audience context**: Rust ML practitioners evaluating ANN libraries (hnsw_rs, usearch, instant-distance)

---

## 1. Driving Force: Why This Research Exists

This experiment is part of a larger Vector Data Structures & Algorithms initiative (research-cfj.2) that began as an open question: "What does the Rust ANN ecosystem actually look like, and which implementations should a Rust project use?"

The initiative started with three iterative Research & Exploration (R&E) runs spanning several days. Run 1 surveyed the broad ecosystem — 21 crates across 16 algorithm families with 45+ references. Run 2 refined the taxonomy, filled gaps, and expanded the scope to include filtered ANN, quantization methods (RaBitQ, TurboQuant), and dimensionality-dependent behavior that the original scope hadn't anticipated. Run 3 achieved convergence on the taxonomy and research questions.

The output of R&E was a structured taxonomy of Rust ANN crates organized by algorithm family, maturity tier, and evaluation dimensions. This taxonomy naturally suggested a cohort-based evaluation strategy: group crates by algorithm family so that within-family comparisons use the same metrics and benchmarks.

## 2. What We Wanted

The core research question for Cohort A was: **"Which HNSW implementation should a Rust project use, and what are the real trade-offs?"**

Sub-questions that emerged during scoping:
- How does FFI (C++ via usearch) compare to pure Rust (hnsw_rs, instant-distance) on throughput, memory, and recall?
- Is the FFI overhead meaningful, or do C++ SIMD optimizations dominate?
- What does "memory overhead" actually mean at the implementation level — is it the algorithm or the language?
- Does the answer change with vector dimensionality (128d embeddings vs 1536d LLM embeddings)?
- Could a hypothetical pure Rust implementation match usearch's C++ performance?

## 3. How We Chose What to Compare

### Cohort definitions (from R&E taxonomy)

The R&E phase identified 5 cohorts based on algorithm family:

| Cohort | Family | Tier 1 Crates | Primary Trade-off |
|--------|--------|--------------|-------------------|
| A | NSW/HNSW (graph) | hnsw_rs, usearch | Pure Rust vs FFI; filtered ANN |
| B | DiskANN (SSD-resident graph) | diskann | Memory per vector; SSD I/O |
| C | Tree-based (RPT, KD, VP) | arroy, kiddo | Dimensionality degradation |
| D | Quantization (PQ, RaBitQ) | faiss | Compression vs recall |
| X | Distance utility | simsimd | SIMD throughput; composability |

Cohort A was chosen as the first evaluation target because HNSW is the dominant ANN algorithm in both literature and production. The three selected crates represent the three main implementation strategies available in the Rust ecosystem:

1. **hnsw_rs** (v0.3.4) — Pure Rust, most popular pure-Rust HNSW. 332K downloads. Filterable trait for predicate-based search. Generic over data types via the `Distance<T>` trait from the `anndists` crate.

2. **usearch** (v2.25.1) — FFI wrapper around C++ USearch by Unum Cloud. 221K downloads. Single-file C++ header compiled via CXX bridge. Supports multi-precision storage (f32/f16/i8), filtered search, and runtime metric switching. Backed by SimSIMD for hand-tuned SIMD distance kernels.

3. **instant-distance** (v0.6.1) — Pure Rust, clean API. 126K downloads. M=32 hardcoded (not configurable). Focused, minimal implementation. Possibly "complete/stable" rather than abandoned — the last release was functional with no outstanding issues.

### Why these three and not others

The R&E taxonomy identified ~8 HNSW-family crates in the Rust ecosystem. Tier 1 (hnsw_rs, usearch) were selected based on download count, maintenance activity, and feature completeness. instant-distance was selected as Tier 2 because it represents a different design philosophy (simplicity over configurability) and its hardcoded M=32 provides a natural control variable.

Other HNSW crates (hnswlib-rs, swarc, granne, small-world-rs) were classified as Tier 3 references — examined for architectural patterns but not deep-dived.

## 4. Research Infrastructure: The Benchmark Harness

Before any crate could be benchmarked, we needed a shared measurement framework. This was tracked as a separate epic (research-ola: Shared Benchmark Harness) with its own spec, plan, implementation, and review cycle.

### Design decisions

The harness was designed around a core trait:

```rust
pub trait AnnIndex: Sized {
    type Build: BuildConfig;
    type Query: QueryConfig;

    fn build(vectors: &[f32], n: usize, dim: usize, metric: DistanceMetric, config: &Self::Build) -> Result<Self>;
    fn query(&self, vector: &[f32], k: usize, config: &Self::Query) -> Result<Vec<QueryResult>>;
    fn filtered_query(&self, vector: &[f32], k: usize, config: &Self::Query, filter: &dyn Fn(usize) -> bool) -> Result<Vec<QueryResult>>;
    fn save(&self, path: &Path) -> Result<u64>;
    fn load(path: &Path, metric: DistanceMetric) -> Result<Self>;
    // ...
}
```

Key design choices:
- **Flat vector buffer input** (`&[f32]`, not `Vec<Vec<f32>>`) — adapters convert to their internal format
- **Separate build and query configs** — enables parameter sweeps at query time without rebuilding
- **`prepare_query(&mut self, config)` hook** — called once before measurement, outside timing. This was critical for instant-distance (see below).
- **Query result type** is `(usize, f32)` — vector index + distance. The distance value is in whatever space the adapter uses. **This assumption would later cause the measurement bug.**

### Measurement protocol

The harness measures QPS using a multi-run best-of-N protocol:
1. Call `prepare_query()` (outside timing)
2. One warmup run (discarded)
3. N measurement runs, each timing all queries
4. Take the minimum total time (best run)
5. QPS = n_queries / min_time

Recall is computed using the ann-benchmarks distance-threshold method:
```
threshold = ground_truth[k-1].distance + EPSILON  (where EPSILON = 1e-3)
match = (ann_distance <= threshold)
recall@k = count(matches) / k
```

Ground truth is computed via brute-force (parallelized with rayon) and cached to disk.

### Workspace structure

```
ann-bench-core/        Core trait, types, tier classification
ann-bench-datasets/    Synthetic generator, fvecs loader, ground truth
ann-bench-harness/     Runner pipeline, measurement, recall, memory
ann-bench-cli/         CLI: run, ground-truth, pareto, tier, list
adapters/
  ann-bench-hnsw-rs/   hnsw_rs adapter
  ann-bench-usearch/   usearch adapter
  ann-bench-instant-distance/  instant-distance adapter
```

## 5. Building the Adapters

### hnsw_rs adapter

Straightforward implementation. hnsw_rs's API requires `Vec<Vec<f32>>` for vector ownership — the library borrows from owned data via lifetime annotations (`&'b [T]`). Our adapter copies the flat input buffer into per-vector Vecs:

```rust
let data: Vec<Vec<f32>> = (0..n)
    .map(|i| vectors[i * dim..(i + 1) * dim].to_vec())
    .collect();
```

This copy is necessary and would later be identified as a source of memory overhead (each Vec adds 24 bytes of ptr/len/cap metadata plus a separate heap allocation per vector).

The adapter used `DistL2` from the `anndists` crate for euclidean distance. **We did not inspect what `DistL2` actually computes.** This was the first oversight.

### usearch adapter

usearch's Rust bindings use CXX to bridge to a C++ header-only library. The API is opaque but functional:

```rust
let options = IndexOptions {
    dimensions: dim,
    metric: MetricKind::L2sq,  // Squared L2
    quantization: ScalarKind::F32,
    connectivity: config.m,
    expansion_add: config.ef_construction,
    expansion_search: 200,
    multi: false,
};
let index = Index::new(&options)?;
```

Key implementation detail: `change_expansion_search()` takes `&self` (interior mutability via C++ side), so ef_search can be tuned per-query without rebuilding. This maps cleanly to the harness's query config sweep pattern.

Filtered search uses a closure: `index.filtered_search(query, k, |key: Key| filter(key as usize))`.

### instant-distance adapter

instant-distance presented two challenges:

**Challenge 1: Distance function is baked into the Point type.** The `Point` trait requires `fn distance(&self, other: &Self) -> f32`, with no external metric parameter. We solved this by embedding the metric in the point struct:

```rust
#[derive(Clone, Serialize, Deserialize)]
struct MetricPoint {
    data: Vec<f32>,
    id: usize,         // original vector index for result mapping
    metric: DistanceMetric,
}

impl Point for MetricPoint {
    fn distance(&self, other: &Self) -> f32 {
        match self.metric {
            DistanceMetric::Euclidean => /* squared L2 */,
            DistanceMetric::Cosine => /* 1 - cosine_similarity */,
            DistanceMetric::DotProduct => /* -dot(a,b) */,
        }
    }
}
```

We explicitly implemented squared L2 (`sum((a-b)²)`) to match our ground truth distance function. **This was correct.** The fact that we were conscious of the distance function here but not in the hnsw_rs adapter is notable in retrospect.

**Challenge 2: ef_search is a build-time parameter.** instant-distance's `Builder::ef_search()` sets the search expansion at index construction time. There is no query-time equivalent. To sweep different ef_search values within the harness's build-once-sweep-many interface, we used `prepare_query()` to rebuild the entire index:

```rust
fn prepare_query(&mut self, config: &Self::Query) {
    if config.ef_search != self.current_ef_search {
        let (hnsw, _ids) = Builder::default()
            .ef_construction(self.ef_construction)
            .ef_search(config.ef_search)
            .build_hnsw(self.points.clone());
        self.hnsw = hnsw;
        self.current_ef_search = config.ef_search;
    }
}
```

This requires keeping a full clone of the points vector for rebuilding. **This clone is the source of instant-distance's inflated memory and disk overhead** — vectors are stored twice in memory (once in the Hnsw, once in the rebuild cache) and three times on disk (the Hnsw serialization includes its internal copy, plus the separate clone).

**Challenge 3: serde feature flag.** instant-distance v0.6.1 has a bug where enabling the `serde` feature without `serde-big-array` causes a compilation error (`BigArray` undeclared). The correct feature flag is `with-serde`, which enables both dependencies. This was discovered during the first build and fixed immediately.

### CLI integration

Each adapter was registered in the CLI's run command dispatch and list command. A `--m` and `--ef-construction` CLI argument was added later to enable fair M=32 comparisons.

## 6. Initial Experiment Planning

### Parameter choices

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| n_vectors | 10,000 | Quick iteration during development |
| n_queries | 1,000 | Statistically adequate (~3% SE on recall) |
| dimensions | 128, 384, 768, 1536 | Covers traditional (128) through LLM (1536) embedding sizes |
| M | 16 (default) | Standard HNSW default; matches most literature |
| ef_construction | 200 | Standard; high enough for good graph quality |
| ef_search sweep | [10, 20, 50, 100, 200, 500] | Covers low-recall/high-QPS through high-recall/low-QPS |
| metric | euclidean | Most common for benchmarking; cosine deferred to Pass 2 |
| n_runs | 3 | Adequate for QPS stability |

### Initial experiment structure

The plan was a three-phase approach:
1. **10K vectors** — validate adapters work, establish rough performance profiles (~5 min)
2. **100K vectors** — research-grade results, real recall differentiation (~1-2 hrs)
3. **1M vectors** — publication-quality Pareto frontiers (~6-12 hrs overnight)

Ground truth would be computed once per (dataset, metric, k) tuple and cached.

## 7. First Run Results (10K, M=16)

### The data

All 12 runs (3 adapters × 4 dimensions) completed successfully.

```
10K vectors, M=16, euclidean (ORIGINAL — with hnsw_rs distance bug)
Crate                 Dim  BestR   BestQ   R@100   Q@100
hnsw_rs               128 1.0000   39782  1.0000    6068   ← "perfect recall"
hnsw_rs               384 1.0000   15169  1.0000    2690
hnsw_rs               768 1.0000    6704  1.0000    1274
hnsw_rs              1536 1.0000    2908  1.0000     633
usearch               128 0.9942   60831  0.8909    9862
usearch               384 0.9828   18432  0.8331    3473
usearch               768 0.9744    7949  0.7843    1534
usearch              1536 0.9476    3586  0.7558     718
instant-distance      128 1.0000   22999  0.9806    3636
instant-distance      384 0.9988    7379  0.9496    1498
instant-distance      768 0.9980    3028  0.9306     633
instant-distance     1536 0.9970    1416  0.9129     315
```

### Initial interpretation (later proven partially wrong)

At the time, we interpreted these results as:

1. **hnsw_rs has superior graph quality** — perfect recall at all ef values and dimensions suggested exceptionally well-constructed graphs. We noted this was "suspicious" at 10K but attributed it to the small dataset size making the graph well-connected relative to the search space.

2. **usearch has a clear QPS advantage** — 1.5-2x faster than hnsw_rs, attributed to C++ SIMD optimization and compact memory layout.

3. **instant-distance trades QPS for recall** — M=32 (vs M=16) produces better recall but slower queries due to denser graph traversal.

4. **Memory analysis revealed structural differences** — usearch at 149 B/vec graph overhead (constant across dimensions), hnsw_rs at 630 B/vec, instant-distance growing with dimension (860-6492 B/vec).

### What we got right

- The QPS comparison between usearch and hnsw_rs was valid (QPS measurement is independent of recall computation)
- The memory analysis was correct and led to genuine architectural insights
- The instant-distance disk overhead growing with dimension correctly identified the serialization duplication bug
- usearch's constant 149 B/vec graph overhead correctly characterized its tape-based memory model

### What we got wrong

- **hnsw_rs recall was entirely artificial.** Every recall number was 1.0000 because the measurement was broken, not because the graph was perfect.
- **The narrative of "hnsw_rs trades QPS for better recall" was backwards.** When measured correctly, hnsw_rs has the *lowest* recall of the three.

## 8. M=32 Fair Comparison

We ran hnsw_rs and usearch at M=32 to compare fairly with instant-distance (hardcoded M=32). This produced an important secondary finding:

```
M=32 QPS ratio (usearch / hnsw_rs):
  128d:  1.53x
  384d:  1.12x
  768d:  1.23x
  1536d: 1.18x
```

**The usearch QPS advantage narrows with dimensionality.** At 128d, usearch is 53% faster. At 1536d, only 18% faster. This is because at high dimensions, distance computation (O(dim)) dominates over graph traversal, and both implementations spend most time in the distance function. usearch's compact edges, prefetch instructions, and thread-local context — all graph traversal optimizations — matter most at low d.

This finding was valid and unaffected by the recall bug.

## 9. Architectural Deep Dive

Two research agents were dispatched in parallel to examine the usearch C++ source code and hnsw_rs Rust source code. Key findings:

### hnsw_rs architecture

- **Edges**: Each edge is `Arc<PointWithOrder<T>>` — approximately 68 bytes per edge (Arc control block ~56B, pointer 8B, distance 4B). This is 17x usearch's 4 bytes per edge.
- **Vectors**: Stored as `Vec<Vec<f32>>` — each vector is a separate heap allocation with 24 bytes of Vec metadata. Vectors are scattered across the heap, causing cache misses during distance computation.
- **Synchronization**: `Arc<RwLock<Vec<Vec<Arc<PointWithOrder>>>>>` per node. RwLock adds ~48-56 bytes per node even when single-threaded.
- **SIMD**: None by default. Delegated to `anndists` crate with optional `simdeez_f` (x86_64) and `stdsimd` (nightly) features, neither enabled by default.
- **API**: Rich — generic types, FilterT trait, layer access, parallel insert, graph flattening.

### usearch architecture

- **Edges**: Stored as packed `compressed_slot_t` arrays — either `uint32_t` (4B) or custom `uint40_t` (5B). Distances are NOT stored; computed on-the-fly.
- **Vectors**: Contiguous memory pool (C++ aligned allocation). Zero per-vector overhead.
- **Traversal**: Hash-set visited list with linear probing, sorted buffer for top-K, explicit `_mm_prefetch()` in traversal loops, thread-local reusable search context.
- **SIMD**: Delegates to SimSIMD — 200+ hand-tuned kernels across SSE/AVX2/AVX-512/NEON/SVE. Includes Horner's method approximations and bit-hack sqrt replacement.
- **API**: Sealed — no graph introspection, limited custom distance functions (raw pointer callbacks), but multi-precision quantization and multi-vector-per-key.

### Memory analysis synthesis

The memory overhead difference decomposes into:

| Factor | usearch | hnsw_rs | Ratio |
|--------|---------|---------|-------|
| Per edge | 4-5 B (ID only) | ~68 B (Arc + distance) | 14-17x |
| Per vector (overhead) | 0 B (contiguous pool) | 24 B (Vec metadata) + copy | — |
| Per node (sync) | 0 B (C++ atomics) | ~50 B (RwLock) | — |
| Graph overhead/vec | 149 B (constant) | ~630 B (constant) | 4.2x |
| Total overhead/vec | 305 B (constant) | 2446 B (grows with dim) | 8x |

The dimension-dependent growth in hnsw_rs overhead is entirely explained by the `Vec<Vec<f32>>` copy — each vector's data is duplicated because hnsw_rs borrows from owned data.

## 10. 100K Results and the First Doubt

We ran 100K vectors with 1K queries. The results showed:

```
100K, M=16, euclidean (ORIGINAL — with hnsw_rs distance bug)
hnsw_rs  128d: recall=1.0000, QPS=27364
usearch  128d: recall=0.8796, QPS=38029
instant  128d: recall=0.9764, QPS=13731
```

hnsw_rs was STILL showing perfect recall at 100K. We noted this was "interesting" and suggested that even larger datasets might be needed. But the persistence of exactly 1.0000 across ALL ef_search values, ALL dimensions, and now ALL scales began to feel anomalous.

## 11. 1M Run and OOM Discovery

We attempted a 1M sweep with 10K queries. Every single run was OOM-killed by macOS during ground truth computation. The brute-force ground truth at 1M/1536d requires ~6.1 GB for vectors alone, plus rayon thread buffers, exceeding the 32 GB system RAM when combined with other processes.

**Resolution**: Reduced n_queries from 10K to 1K. Ground truth at 1M/1536d with 1K queries peaks at ~6.3 GB (vs ~21 GB with 10K queries). This fit within the 26 GB available.

The 1M run with 1K queries produced:

```
1M, M=16, euclidean (with hnsw_rs distance bug, partial results)
hnsw_rs  128d: recall=1.0000, QPS=12174
usearch  128d: recall=0.5831, QPS=20113
instant  128d: recall=0.7889, QPS=6479
```

usearch's recall dropped to 0.58 at 1M — dramatically lower than 100K. instant-distance dropped to 0.79. But hnsw_rs remained at exactly 1.0000.

## 12. What Made Us Double-Check

The user asked a critical question: **"Can you reason about why usearch and instant-distance are underperforming hnsw_rs so much?"**

This forced a closer examination. The reasoning chain:

1. hnsw_rs achieves recall=1.0000 at ef_search=10 with 1M vectors. ef=10 means the algorithm explores only 10 candidates. For recall@10 to be perfect, the 10 nearest neighbors must ALWAYS be among the first 10 candidates in the priority queue. At 1M vectors, this is statistically near-impossible unless the graph has essentially perfect connectivity.

2. The recall is not 0.9999 or 0.9998 — it is EXACTLY 1.0000 at every ef value, every dimension, every scale. A real HNSW implementation would show at least marginal recall loss at low ef values.

3. If hnsw_rs had genuinely superior graph quality, we would expect a gap, but not perfection. usearch implements the same HNSW algorithm with the same M and ef_construction parameters.

4. The only way to get artificially perfect recall is if the distance comparison always passes the threshold check, regardless of whether the correct neighbors were found.

## 13. Root Cause Analysis

### Hypothesis

The recall metric computes: `ann_distance <= ground_truth_distance + EPSILON`. If the ANN returns distances in a smaller scale than ground truth, the threshold check trivially passes.

Ground truth uses `euclidean_sq`: `sum((a-b)²)` — squared L2, no square root.

What does hnsw_rs's `DistL2` return?

### Investigation

We examined the `anndists` crate source code at `~/.cargo/registry/src/*/anndists-0.1.5/src/dist/distances.rs`:

```rust
fn scalar_l2_f32(va: &[f32], vb: &[f32]) -> f32 {
    let norm: f32 = va.iter().zip(vb.iter())
        .map(|t| (*t.0 - *t.1) * (*t.0 - *t.1))
        .sum();
    assert!(norm >= 0.);
    norm.sqrt()   // ← HERE
}
```

**`DistL2` returns L2 distance (with sqrt), not squared L2 (without sqrt).**

### The bug mechanism

```
Ground truth distance (squared L2):     sum((a-b)²)      → e.g., 150.0
hnsw_rs returned distance (L2):        sqrt(sum((a-b)²)) → e.g., 12.25

Recall threshold:  ground_truth + EPSILON = 150.001
Check:             12.25 <= 150.001  →  TRUE  (always, for any distance > 1)
```

For any distance greater than 1.0, `sqrt(x) < x`, so the hnsw_rs distance is always less than the ground truth threshold. Every result trivially passes. Recall is always 1.0000.

### Why it wasn't caught earlier

1. **The adapter tests use `< 0.01` for exact matches**, which passes for both L2 (0.0) and L2sq (0.0). The test didn't distinguish between distance scales.

2. **At small scales (10K), high recall is plausible.** We attributed perfect recall to "small dataset, well-connected graph" rather than investigating the distance function.

3. **usearch explicitly uses `MetricKind::L2sq`** (squared L2), which matched our ground truth. instant-distance uses our custom `sum((a-b)²)` implementation, also matching. Only hnsw_rs used a library function (`DistL2`) that we didn't inspect.

4. **The name `DistL2` is ambiguous.** "L2" can mean either the L2 norm (with sqrt) or the L2 distance (ambiguous between squared and non-squared). Different libraries use different conventions. FAISS uses squared L2 for `METRIC_L2`. hnsw_rs/anndists uses non-squared.

5. **No cross-validation step.** We never compared raw distance values between adapters for the same query/vector pair. If we had, the mismatch would have been immediately visible.

### Why it matters

- **All hnsw_rs recall numbers in every benchmark run were invalid.** Recall was always 1.0000 regardless of actual search quality.
- **The narrative was inverted.** We claimed hnsw_rs "trades QPS for better recall." In reality, hnsw_rs has the lowest recall of the three at the same M and ef_construction.
- **Architectural conclusions about graph quality were unfounded.** We speculated that hnsw_rs's (id, distance) edge storage enabled better neighbor selection. This may or may not be true, but the recall data couldn't support the claim.

### What was NOT affected

- **QPS measurements** — timing is independent of recall computation
- **Memory analysis** — RSS and disk measurements are independent of distance values
- **Architectural analysis** — the code examination of internal data structures is factual
- **usearch and instant-distance results** — their distance functions matched ground truth from the start

## 14. The Fix

### Code change

In `adapters/ann-bench-hnsw-rs/src/lib.rs`, both `query_inner` and `filtered_query_inner`:

```rust
// Before (buggy):
.map(|n| (n.d_id, n.distance))

// After (fixed):
.map(|n| (n.d_id, n.distance * n.distance))
```

Squaring the L2 distance converts it to L2sq, matching ground truth's distance space.

### Corrected results (10K, M=16)

```
10K, M=16, euclidean (CORRECTED)
Crate                 Dim  BestR   BestQ   R@100   Q@100
hnsw_rs               128 0.9879   41673  0.8945    5999   ← was falsely 1.0000
hnsw_rs               384 0.9870   14770  0.8573    2687
hnsw_rs               768 0.9734    6636  0.8060    1269
hnsw_rs              1536 0.9812    3098  0.8018     623
usearch               128 0.9998   55977  0.8909    9167   ← unchanged
usearch               384 0.9996   17466  0.8331    3301
usearch               768 0.9946    7800  0.7843    1491
usearch              1536 0.9727    3327  0.7558     694
instant-distance      128 1.0000   22464  0.9808    3629   ← unchanged
instant-distance      384 1.0000    7262  0.9550    1455
instant-distance      768 0.9998    3115  0.9287     611
instant-distance     1536 0.9998    1416  0.9114     325
```

### The real ranking

At maximum ef (2000), best recall:
- **instant-distance: 1.000** (128d) → 0.9998 (1536d) — best recall, lowest QPS
- **usearch: 0.9998** (128d) → 0.973 (1536d) — middle recall, highest QPS
- **hnsw_rs: 0.988** (128d) → 0.981 (1536d) — lowest recall, middle QPS

The ordering is completely inverted from what the buggy data suggested.

## 15. Additional Improvements Made During the Fix

### Extended ef_search sweep

The original sweep `[10, 20, 50, 100, 200, 500]` was insufficient at larger scales. Extended to `[10, 20, 50, 100, 200, 500, 1000, 2000]` for all three adapters. This provides better coverage of the high-recall region of the Pareto curve, especially at 1M where ef=500 only reached 0.58 recall for usearch.

### Reproducibility infrastructure

- **justfile** with sweep recipes (`sweep-10k`, `sweep-100k`, `sweep-1m`) including confirm gates that display computational requirements before running
- **Skip-if-exists checkpointing** — sweeps skip runs whose output JSON already exists, making interrupted sweeps resumable
- **`clean` recipe** for fresh-start re-runs
- **`summary` and `pareto` recipes** calling Python analysis scripts
- **`env` recipe** printing Rust toolchain, platform, CPU, and RAM for reproducibility metadata

### Standalone repository

The benchmark harness was extracted into a standalone repository (`bench-ann`) using `git-filter-repo` to preserve commit history. Two source paths were remapped:
- `benchmarks/vector-dsa/*` → repo root
- `docs/src/vector-dsa/*` → `docs/src/`

The repo is published at github.com/aRustyDev/bench-ann.

## 16. Lessons Learned

### Lesson 1: Verify distance function semantics across library boundaries

**What happened**: We used `DistL2` from `anndists` without verifying whether it returns L2 (with sqrt) or L2sq (without sqrt). The name is ambiguous and different libraries use different conventions.

**Prevention**: When integrating a distance function from an external library into a measurement framework, always:
1. Read the source code of the distance function
2. Compute a known distance by hand and compare
3. Cross-validate distance values between adapters for the same input

### Lesson 2: Exact 1.0000 recall is a red flag, not a green flag

**What happened**: We attributed perfect recall to excellent graph quality rather than investigating whether the measurement was broken.

**Prevention**: Any result that is exactly at the theoretical maximum across all parameter settings should trigger skepticism. Real algorithms have imperfections. If recall@10 is 1.0000 at ef_search=10 with 100K vectors, something is wrong with the measurement, not right with the algorithm.

### Lesson 3: The recall threshold method is sensitive to distance scale

**What happened**: The ann-benchmarks-style threshold method (`dist <= gt_dist + epsilon`) assumes both distances are in the same scale. A scale mismatch (L2 vs L2sq) causes systematic bias.

**Prevention**: Either:
- Normalize all distances to a common scale before comparison
- Use set-intersection recall (check if the correct IDs appear, ignoring distances)
- Add a sanity check: compare raw distance values between ANN results and ground truth for exact-match queries

### Lesson 4: Test adapters against known answers, not just "does it run"

**What happened**: Our adapter tests verified that build/query/save/load worked and that exact-match distance was "close to zero." They did not verify that distance values matched a known expected value.

**Prevention**: Add a test that queries for a known vector and asserts the distance is within a tight tolerance of the hand-computed value. For example: query [1,0,0,0] against [0,1,0,0] should return distance=2.0 (squared L2) not 1.414 (L2).

### Lesson 5: OOM on brute-force ground truth scales quadratically with n_queries

**What happened**: 1M vectors × 10K queries × rayon parallelism exceeded 32 GB RAM. The fix was reducing to 1K queries.

**Prevention**: Estimate memory budget before running: `n_vectors × dim × 4` (vectors) + `n_threads × n_vectors × 12` (per-thread distance buffers) + index build memory. For 32 GB systems, 1K queries at 1M vectors is safe up to 1536d.

### Lesson 6: The `prepare_query` rebuild pattern works but has hidden costs

**What happened**: instant-distance's ef_search is a build-time parameter. We used `prepare_query()` to rebuild the index for each sweep point. This is correct (rebuild is outside timing) but stores vectors twice in memory and three times on disk.

**Prevention**: Document the memory implications. Consider reconstructing the rebuild cache from `hnsw.iter()` on load rather than serializing it.

### Lesson 7: Separate the harness build from the analysis interpretation

**What happened**: We wrote the architectural analysis (usearch vs hnsw_rs trade-offs) before verifying that the recall data was correct. The analysis was partly based on the false premise that hnsw_rs had superior recall.

**Prevention**: Run a full validation suite (including cross-adapter distance checks) before writing interpretive analysis. Data first, narrative second.

## 17. Current State (as of 2026-04-27 evening)

### Completed

- All three adapters built, tested (42 tests passing), and registered in CLI
- 10K sweep complete with corrected distances and extended ef sweep [10..2000]
- 100K sweep running (7 of 12 complete as of last check)
- 1M sweep queued to run overnight after 100K completes
- Architectural analysis documents written (5 findings documents)
- Standalone bench-ann repository published with reproducible justfile
- Session memory updated

### In progress

- 100K corrected sweep (~5 runs remaining)
- 1M corrected sweep (queued, overnight)

### Outstanding for Pass 2

- M=32 corrected comparisons (need re-run with distance fix)
- hnsw_rs with `simdeez_f` SIMD feature (isolate SIMD contribution to QPS gap)
- usearch with higher ef_search [up to 5000] at 1M (can it reach 0.95+ recall?)
- Fix instant-distance serialization duplication
- Filtered ANN comparison: hnsw_rs Filterable trait vs usearch predicate fn
- Cosine metric evaluation
- Source code quality assessment (unsafe audit, API ergonomics)

### Open research questions

1. Does hnsw_rs's lower recall (vs usearch at same M) indicate a weaker neighbor selection heuristic, or is there another factor?
2. Would enabling hnsw_rs's SIMD features close the QPS gap meaningfully?
3. At 1M scale, what ef_search does each adapter need to reach 0.95 recall? What's the QPS at that operating point?
4. Is the dimensionality-narrowing of the usearch QPS advantage (1.53x at 128d → 1.18x at 1536d) consistent at larger scales?
