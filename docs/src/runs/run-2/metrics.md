# Evaluation Metrics — Operationalized (Run 2)

> **Bead**: `research-cfj.2.14.2.4`
> **Date**: 2026-04-24

## Purpose

Operationalize the evaluation dimensions identified in Run 1. Define specific metrics, measurement protocols, and performance tier thresholds to enable consistent comparison across ANN crates in later phases.

## Test Dimensions

All benchmarks should be run at these dimensionalities to capture how algorithms degrade with increasing dimension:

| Dimension | Represents | Dataset |
|-----------|-----------|---------|
| 128d | Low-dimensional features (SIFT, traditional CV) | SIFT1M (standard) |
| 384d | Small embedding models (all-MiniLM-L6-v2) | Synthetic: i.i.d. Gaussian or uniformly sampled on unit sphere |
| 768d | Medium embedding models (BERT, BGE-base) | Synthetic or GloVe-derived |
| 1536d | Large embedding models (OpenAI ada-002) | Synthetic |

**Synthetic dataset protocol**: For each dimensionality, generate N vectors sampled uniformly from the unit sphere (to test with cosine/inner product) and N vectors from a multivariate Gaussian (to test with L2). Use fixed random seed for reproducibility.

## Dataset Sizes

| Scale | N (vectors) | Target Use Case |
|-------|-------------|-----------------|
| Small | 10,000 | Unit tests, sanity checks |
| Medium | 100,000 | Development benchmarks |
| Standard | 1,000,000 | Primary benchmark (comparable to ann-benchmarks) |
| Large | 10,000,000 | Scale test for production readiness |

**Primary benchmark**: 1M vectors. Report all metrics at this scale.
**Query set**: 10,000 queries (same distribution as base vectors).

## Distance Metrics to Test

| Metric | Priority | Notes |
|--------|----------|-------|
| Cosine similarity | P0 | Most common for embeddings. Test via pre-normalized + inner product where libraries support it. |
| Euclidean (L2) | P0 | Standard. Use squared L2 if library supports it. |
| Dot product (inner product) | P1 | For MIPS problems. Distinct from cosine only on non-normalized vectors. |

## Core Metrics

### 1. Recall@10

**What**: Fraction of true 10-nearest neighbors found by the ANN algorithm.
**Protocol**: For each of the 10,000 queries, compute exact 10-NN (brute force). Then run the ANN algorithm and compute |ANN_10 ∩ exact_10| / 10. Report the mean across all queries.
**Why k=10**: Balances between single-result (k=1, too noisy) and large result sets (k=100, inflates recall for methods with strong top-1 but weak tail).

| Tier | Recall@10 |
|------|-----------|
| Unacceptable | < 0.90 |
| Acceptable | 0.90 – 0.95 |
| Good | 0.95 – 0.99 |
| Excellent | >= 0.99 |

### 2. Queries Per Second (QPS)

**What**: Throughput at a fixed recall level.
**Protocol**: Measure single-threaded QPS at recall@10 >= 0.95 (the "good" threshold). Report median over 3 runs after 1 warmup run. Exclude index load time.
**Parameterization**: Tune the algorithm's recall knob (ef_search for HNSW, nprobe for IVF, etc.) until recall@10 >= 0.95, then measure QPS.

| Tier | QPS at recall@10 >= 0.95 (1M vectors, 128d) |
|------|---------------------------------------------|
| Unacceptable | < 100 |
| Acceptable | 100 – 1,000 |
| Good | 1,000 – 10,000 |
| Excellent | > 10,000 |

**Scaling note**: QPS thresholds are for 128d. For higher dimensions, expect 2-10x reduction. Report raw numbers at each dimensionality; do not adjust thresholds.

### 3. Build Time

**What**: Wall-clock time to construct the index from raw vectors.
**Protocol**: Single-threaded build time for 1M vectors. Measure wall-clock seconds. Include any training (k-means for IVF, codebook learning for PQ). Exclude data loading I/O.

| Tier | Build time (1M vectors, 128d, single-thread) |
|------|----------------------------------------------|
| Unacceptable | > 600s (10 min) |
| Acceptable | 60s – 600s |
| Good | 10s – 60s |
| Excellent | < 10s |

### 4. Memory Per Vector

**What**: Additional memory consumed by the index per indexed vector, beyond the raw vector data.
**Protocol**: Measure RSS (Resident Set Size) after building the index on 1M vectors. Subtract the raw data size (N * d * sizeof(f32)). Divide by N.
**Formula**: memory_per_vector = (RSS_with_index - raw_data_bytes) / N

| Tier | Overhead per vector (128d, f32) |
|------|---------------------------------|
| Unacceptable | > 2 KB (>4x raw size of 512 bytes) |
| Acceptable | 512 B – 2 KB (1-4x raw) |
| Good | 128 B – 512 B (0.25-1x raw) |
| Excellent | < 128 B (<0.25x raw, i.e., compressed below raw size) |

### 5. Index Size on Disk

**What**: Serialized index footprint.
**Protocol**: Build the index, serialize to disk, measure file size. Report bytes per vector.

| Tier | Disk per vector (128d, f32) |
|------|-----------------------------|
| Unacceptable | > 4 KB (>8x raw) |
| Acceptable | 1 KB – 4 KB |
| Good | 256 B – 1 KB |
| Excellent | < 256 B (compressed) |

### 6. Latency Distribution

**What**: Query response time at p50 and p99.
**Protocol**: Single-threaded, single query at a time. Report p50 and p99 over 10,000 queries at recall@10 >= 0.95.

| Tier | p99 latency (1M vectors, 128d) |
|------|-------------------------------|
| Unacceptable | > 100 ms |
| Acceptable | 10 ms – 100 ms |
| Good | 1 ms – 10 ms |
| Excellent | < 1 ms |

## Secondary Metrics

These are important for characterization but do not have pass/fail thresholds:

### 7. Dimensionality Sensitivity Profile

**What**: How performance degrades as dimension increases from 128d to 1536d.
**Protocol**: Run the primary benchmark at all four dimensionalities. Plot recall@10 vs. QPS Pareto frontier at each dimension. Report the "QPS@0.95recall" ratio between 1536d and 128d.
**Use**: Identify algorithms that are robust vs. sensitive to high dimensions.

### 8. Recall@1 and Recall@100

**What**: Quality at the extremes.
**Protocol**: Same as recall@10 but for k=1 (top-1 accuracy) and k=100 (broader retrieval). Report alongside recall@10 for each algorithm/parameter setting.

### 9. Filtered ANN Performance

**What**: How recall and QPS change when scalar filters are applied.
**Protocol**: Add a categorical attribute with cardinality C (test C=10, 100, 1000). Apply a filter that selects 10% of the dataset. Compare filtered recall@10 and QPS against unfiltered baseline.
**Only for**: Crates that support filtered search.

### 10. Incremental Insert/Delete

**What**: Can the index be updated without full rebuild?
**Protocol**: Build index on 900k vectors. Insert 100k, delete 100k. Measure recall@10 and QPS after updates vs. a fresh build on the same 900k final vectors.
**Only for**: Crates that support incremental updates.

## Benchmark Hardware Specification

To ensure reproducibility, define a reference hardware profile:

| Component | Specification |
|-----------|--------------|
| CPU | Apple M-series (M2/M3/M4) or AMD Zen 4 — single core for primary benchmarks |
| RAM | >= 16 GB |
| Storage | NVMe SSD (for disk-based algorithms) |
| OS | macOS or Linux |

**Comparability note**: ann-benchmarks uses AWS r6i.16xlarge (Intel Xeon, 512GB RAM). Our results will differ in absolute numbers but relative rankings should be similar. Always report the hardware used.

## Benchmark Methodology

1. **Warmup**: 1 run discarded before measurement
2. **Repetitions**: 3 measurement runs, report median
3. **Parameter sweep**: For each algorithm, sweep the primary recall knob over at least 5 settings to generate a recall vs. QPS curve
4. **Ground truth**: Exact k-NN computed via brute force (or a verified exact algorithm)
5. **Isolation**: No other CPU-intensive processes during benchmarking
6. **Reporting**: Pareto frontier plots (recall@10 vs. QPS) are the primary visualization. Tables supplement with specific operating points.

## Metric Summary Table

| Metric | Unit | Primary/Secondary | Threshold Basis |
|--------|------|-------------------|-----------------|
| Recall@10 | 0.0–1.0 | Primary | 4 tiers: <0.90, 0.90, 0.95, 0.99 |
| QPS | queries/sec | Primary | 4 tiers at recall >= 0.95 |
| Build time | seconds | Primary | 4 tiers for 1M vectors |
| Memory per vector | bytes | Primary | 4 tiers relative to raw size |
| Index size on disk | bytes/vector | Primary | 4 tiers relative to raw size |
| Latency (p50, p99) | milliseconds | Primary | 4 tiers for p99 |
| Dimensionality sensitivity | ratio | Secondary | Characterization only |
| Recall@1, Recall@100 | 0.0–1.0 | Secondary | Reported alongside @10 |
| Filtered performance | delta | Secondary | Only for filter-capable crates |
| Incremental update | delta | Secondary | Only for update-capable crates |
