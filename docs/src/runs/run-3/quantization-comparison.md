# Q1.g: Modern Quantization Landscape

> **Run**: 3
> **Date**: 2026-04-24
> **Task**: Q1.g -- Quantization method comparison (PQ through RaBitQ/TurboQuant)

## Paper Sources

| ID | Paper | Venue | Year |
|----|-------|-------|------|
| [RaBitQ] | Gao & Long, "RaBitQ: Quantizing High-Dimensional Vectors with a Theoretical Error Bound for ANN Search" (arXiv:2405.12497) | SIGMOD 2024 | 2024 |
| [ExtRaBitQ] | Gao et al., "Practical and Asymptotically Optimal Quantization of High-Dimensional Vectors" (arXiv:2409.09913) | SIGMOD 2025 | 2024 |
| [RvsTQ] | Gao et al., "Revisiting RaBitQ and TurboQuant" (arXiv:2604.19528) | Technical report | 2026 |
| [GPU-RaBitQ] | Shi et al., "GPU-Native ANN Search with IVF-RaBitQ" (arXiv:2602.23999) | Technical report | 2026 |
| [TurboQuant] | Zandieh et al., "TurboQuant: Online Vector Quantization with Near-optimal Distortion Rate" (arXiv:2504.19874) | ICLR 2026 | 2025 |
| [EDEN-note] | Ben-Basat et al., "A Note on TurboQuant and the Earlier DRIVE/EDEN Line of Work" (arXiv:2604.18555) | Technical report | 2026 |
| [QJL] | Zandieh et al., "QJL: 1-Bit Quantized JL Transform for KV Cache Quantization with Zero Overhead" (arXiv:2406.03482) | ICML 2024 | 2024 |
| [PolarQuant] | Han et al., "PolarQuant: Quantizing KV Caches with Polar Transformation" (arXiv:2502.02617) | AISTATS 2026 | 2025 |
| [DRIVE] | Vargaftik et al., "DRIVE: One-bit Distributed Mean Estimation" | NeurIPS 2021 | 2021 |
| [EDEN] | Vargaftik et al., "EDEN: Communication-Efficient and Robust Distributed Mean Estimation" | ICML 2022 | 2022 |

---

## 1. Method-by-Method Analysis

### 1.1 Product Quantization (PQ)

**Mechanism**: Decomposes D-dimensional vectors into M sub-vectors of D/M dimensions each. Each sub-vector is independently quantized via k-means to one of K centroids (typically K=256). Distance estimated by table lookups against pre-computed centroid distances (ADC -- Asymmetric Distance Computation). [inferred]

**Compression**: At K=256 (8-bit codes per sub-vector) with M sub-vectors: D*32 bits -> M*8 bits. For D=768, M=96: 32x compression. Compression ratio is configurable via M and K. [inferred]

**Training**: Requires k-means clustering per sub-vector space. Training cost is O(K * D/M * N * iterations) where N is the dataset size. Codebook must be re-trained if data distribution shifts. [inferred]

**Theoretical guarantees**: None. PQ has no formal error bound. [RaBitQ] observes that PQ "fail[s] disastrously on some real-world datasets" and notes maximum relative error can reach ~100% on MSong and Word2Vec datasets. [cited: RaBitQ]

**Recall at 32x compression**: Varies by dataset; on favorable datasets (SIFT) PQ achieves competitive recall, but on others (MSong) PQ achieves "<60% recall even with re-ranking". [cited: RaBitQ]

**Distance estimation**: ADC with precomputed lookup tables. SIMD-friendly via FastScan (FAISS). [inferred]

**Rust availability**: No pure-Rust IVF-PQ crate exists (confirmed absent, Run 2). FAISS bindings via `faiss-rs` provide indirect access. [cited: Run 2 report]

**Production adoption**: Ubiquitous. FAISS, Milvus, Qdrant, Weaviate, Pinecone, Vespa all support PQ or variants. [inferred]

### 1.2 Optimized Product Quantization (OPQ)

**Mechanism**: Pre-rotates vectors with a learned orthogonal matrix before applying standard PQ, minimizing quantization distortion. The rotation is optimized to align sub-vector boundaries with the data's principal variance directions. [inferred]

**Compression**: Same as PQ (configurable via M and K). [inferred]

**Training**: PQ training cost plus iterative rotation optimization. Alternates between codebook update and rotation refinement. More expensive than plain PQ. [inferred]

**Theoretical guarantees**: None beyond PQ. Empirically better than PQ, especially when variance is not aligned with sub-vector boundaries. [inferred]

**Recall at equivalent compression**: Strictly >= PQ. Improvement is dataset-dependent (largest when data has skewed variance across dimensions). [inferred]

**Rust availability**: No known standalone crate. Available through FAISS bindings. [inferred]

**Production adoption**: FAISS "OPQ" transform widely used. Many vector databases support it as a preprocessing option. [inferred]

### 1.3 RaBitQ (1-bit)

**Mechanism**: Three-step process [cited: RaBitQ]:
1. **Normalize**: Center vector by dataset centroid, normalize to unit sphere: `o := (o_raw - c) / ||o_raw - c||`
2. **Random rotation**: Apply a shared random orthogonal matrix P to the normalized vector
3. **Binary encoding**: Quantize to nearest vertex of the D-dimensional hypercube {+/- 1/sqrt(D)}^D, producing a D-bit string

The codebook is the set of 2^D vertices of the scaled hypercube -- deterministic after rotation. Each data vector maps to its nearest codebook vertex (by sign of each rotated coordinate).

**Distance estimation** [cited: RaBitQ]: The estimator for inner product <o, q> is:

```
<o_bar, q> / <o_bar, o>
```

where `o_bar` is the quantized codeword, and `<o_bar, o>` is pre-computed at index time. The query is inversely rotated (q' = P^-1 q), and `<o_bar, q'>` is computed via bitwise popcount (XOR + POPCNT) on the D-bit code and quantized query. SIMD batch processing uses lookup tables for groups of 4 bits.

**Compression**: D-dimensional float32 vector -> D bits. This is exactly 32x compression. [cited: RaBitQ]

**Training**: **Data-oblivious**. Only requires sampling a random orthogonal matrix P (one-time, shared across dataset). No k-means, no iterative optimization. [cited: RaBitQ]

**Theoretical guarantees** [cited: RaBitQ]:
- **Unbiasedness**: E[estimator] = true inner product
- **Error bound**: O(1/sqrt(D)) with high probability
- **Failure probability**: 2*exp(-c_0 * epsilon_0^2) -- exponential concentration
- Matches the theoretical lower bound of Alon & Klartag (2017)

**Recall at 32x compression**: Outperforms PQ "by a clear margin" at equivalent compression. Where PQ gives <60% recall on MSong, RaBitQ maintains stable high recall. Bitwise operations are ~3x faster than PQ per vector for single-vector computation. [cited: RaBitQ]

**Rust availability**: `rabitq-rs` (IVF+RaBitQ + MSTG, 13 versions, active development). Critical ARM64 bugs noted. [cited: Run 2 report, web search]

**Production adoption**: Integrated into NVIDIA cuVS library. LanceDB has RaBitQ support. Weaviate exploring integration (8-bit rotational quantization based on similar principles). [cited: GPU-RaBitQ, web search]

### 1.4 Extended RaBitQ (variable-rate)

**Mechanism** [cited: ExtRaBitQ]: Generalizes RaBitQ from 1-bit to B-bit per dimension:
1. Construct a codebook of 2^(B*D) vectors using B-bit unsigned integer grids in D dimensions
2. Normalize each grid vector, apply shared random rotation P
3. Quantize each data vector to its nearest codebook vector via a critical-value enumeration algorithm

The key insight is that B-bit grids provide finer granularity than repeating 1-bit encoding B times. The codebook construction ensures uniform coverage of the unit hypersphere at each rate.

**Compression rates** [cited: ExtRaBitQ]:
- B=1: 32x (original RaBitQ)
- B=2: 16x
- B=4: 8x
- B=8: 4x

**Training**: **Data-oblivious** (same as RaBitQ -- just the shared random rotation). Quantization code finding uses critical value enumeration with time complexity O(2^B * D * log D). For B=5, a million 3072-dim vectors can be quantized in "a few minutes." [cited: ExtRaBitQ]

**Theoretical guarantees** [cited: ExtRaBitQ]:
- **Asymptotically optimal**: Proven to achieve the information-theoretic optimal trade-off between space and error bound (Theorem 3.2)
- **Required bits**: B = Theta(log(1/D * 1/epsilon^2 * log(1/delta))) for error epsilon with probability 1-delta
- **Empirical formula** (>99.9% probability): epsilon < 2^(-B) * 5.75 / sqrt(D) -- exponential decay with B

**Distance estimation** [cited: ExtRaBitQ]: Same estimator structure as RaBitQ. Supports two-stage distance computation with IVF:
- Stage 1: Use most significant bits (equivalent to original RaBitQ) with SIMD FastScan
- Stage 2: If stage 1 insufficient for pruning, access remaining bits for incremental refinement

**Recall at various rates** [cited: ExtRaBitQ]:
- 95% recall: ~6.4x compression
- 99% recall: ~4.5x compression
- Consistently outperforms PQ, SQ, and OPQ at equivalent bit-widths

**Rust availability**: Available through `rabitq-rs` (supports variable rates). [cited: web search]

**Production adoption**: Same as RaBitQ 1-bit; the variable-rate extension is newer (SIGMOD 2025). [inferred]

### 1.5 TurboQuant

**Mechanism** [cited: TurboQuant]:
1. **Random rotation**: Apply random orthogonal matrix (or Randomized Hadamard Transform) to input vector, inducing a concentrated Beta distribution on coordinates
2. **Scalar quantization**: Apply optimal Lloyd-Max scalar quantizers independently per coordinate, exploiting near-independence of coordinates in high dimensions
3. **Inner product variant (TurboQuant_prod)**: Two-stage approach -- (B-1)-bit MSE quantizer followed by 1-bit QJL transform on the residual, yielding an unbiased inner product estimator

**Compression**: Configurable to any bit-width B per coordinate. At B=1: 32x, B=2: 16x, B=4: 8x, etc. [cited: TurboQuant]

**Training**: **Data-oblivious**. No codebook learning, no k-means. The scalar quantizer boundaries are analytically derived from the Beta distribution (which depends only on the dimensionality, not the data). Indexing time is "virtually zero" compared to PQ. [cited: TurboQuant]

**Theoretical guarantees** [cited: TurboQuant]:
- Near-optimal distortion rate within a constant factor (~2.7x) of the information-theoretic lower bound
- Formal proof of the lower bound provided
- Applies across all bit-widths and dimensions

**Recall**: Claims to outperform PQ in ANN recall while reducing indexing time to virtually zero. [cited: TurboQuant]

**Contested claims** [cited: RvsTQ, EDEN-note]:
- [RvsTQ] finds "TurboQuant does not provide a consistent improvement over RaBitQ in directly comparable settings; in many tested configurations, it performs worse than RaBitQ."
- [RvsTQ] reports RaBitQ "consistently achieves higher recall" across 3 datasets at 2-4 bits. Advantage most pronounced at small k and 2-bit width.
- [RvsTQ] found TurboQuant quantization times "up to approximately two orders of magnitude slower" than reported when using the released implementation.
- [EDEN-note] argues TurboQuant_mse is a special case of EDEN (ICML 2022) with suboptimal scale parameter S=1, and TurboQuant_prod is suboptimal in three specific ways. Claims "2-bit EDEN beats 3-bit TurboQuant_prod."

**Component papers**:
- **QJL** (ICML 2024) [cited: QJL]: 1-bit quantized Johnson-Lindenstrauss transform. Eliminates memory overhead of storing quantization constants. Asymmetric estimator: apply QJL to one vector, standard JL to the other, get unbiased inner product. Used as the residual correction step in TurboQuant_prod.
- **PolarQuant** (AISTATS 2026) [cited: PolarQuant]: Transforms vectors to polar coordinates via random preconditioning + recursive polar transformation. Angles have tightly bounded, analytically computable distributions after preconditioning. Eliminates per-block normalization constants (zero overhead). Achieves >4.2x KV cache compression. Provides the MSE quantization foundation for TurboQuant.

**Rust availability**: `turbo-quant` crate (implements TurboQuant, PolarQuant, and QJL). [cited: Run 2 report, web search]

**Production adoption**: Early stage. Google Research origin. No major vector database integration announced yet. [inferred]

### 1.6 Scalar Quantization (SQ / int8)

**Mechanism**: Linearly maps each floating-point coordinate to an integer range (typically 0-255 for int8). Per-dimension or per-vector min/max scaling. [inferred]

**Compression**: float32 -> int8 = 4x compression. float32 -> int4 = 8x. [inferred]

**Training**: Trivial -- compute min/max per dimension (single pass). No iterative optimization. [inferred]

**Theoretical guarantees**: Uniform quantization error bounded by (max-min)/(2*levels) per dimension. No concentration guarantees for aggregate distance estimation. [inferred]

**Recall at 4x compression**: Very high recall (typically >0.99) at int8 / 4x compression. Degrades significantly at int4 / 8x. [inferred]

**Distance estimation**: Standard integer arithmetic. SIMD-friendly (VNNI on x86, dot product instructions on ARM). [inferred]

**Rust availability**: Widely available. Most vector libraries support SQ natively. [inferred]

**Production adoption**: Universal. Every major vector database supports int8 SQ. Qdrant, pgvector, and others use it as default quantization. [inferred]

### 1.7 Binary Quantization (naive sign-bit)

**Mechanism**: Each float coordinate is mapped to 1 bit by its sign (or by thresholding at 0/median). Hamming distance approximates cosine similarity for normalized vectors. [inferred]

**Compression**: float32 -> 1 bit = 32x compression. [inferred]

**Training**: None. Purely coordinate-wise thresholding. [inferred]

**Theoretical guarantees**: Sign-bit hashing preserves angular distance with known bounds (Charikar 2002 / simhash). The relationship Hamming(sign(x), sign(y)) approximates arccos(cos_sim(x,y)) / pi. Guarantees degrade with non-centered distributions. [inferred]

**Recall at 32x compression**: Poor in general. Highly dataset-dependent. For high-dimensional embeddings with well-distributed coordinates, can achieve 0.7-0.8 recall for re-ranking use; for raw search without re-ranking, typically much lower. [inferred]

**Distance estimation**: Hamming distance via XOR + POPCNT. Extremely fast. [inferred]

**Rust availability**: Trivial to implement. Available in most libraries as a building block. [inferred]

**Production adoption**: Used as a coarse pre-filter (re-ranking required). Qdrant "binary quantization" feature; Weaviate BQ; pgvector bit type. [inferred]

---

## 2. Comparison Table

| Dimension | PQ | OPQ | RaBitQ (1-bit) | Extended RaBitQ | TurboQuant | SQ (int8) | Binary (naive) |
|-----------|-----|------|-----------------|-----------------|------------|-----------|----------------|
| **Mechanism** | Sub-vector k-means + ADC | Learned rotation + PQ | Random rotation + hypercube vertex encoding | Random rotation + B-bit grid quantization | Random rotation + per-coord scalar quantizer (+ QJL residual for IP) | Linear per-coord scaling | Sign-bit thresholding |
| **Typical compression** | 8x-64x (configurable) | 8x-64x (configurable) | 32x (fixed) | 4x-32x (configurable via B) | 4x-32x (configurable via B) | 4x (int8) or 8x (int4) | 32x (fixed) |
| **Training required?** | Yes (k-means) | Yes (k-means + rotation) | No (data-oblivious) | No (data-oblivious) | No (data-oblivious) | Minimal (min/max scan) | No |
| **Training cost** | O(K*D/M*N*iters), minutes to hours | Higher than PQ (+ rotation optimization) | One random matrix sample (negligible) | One random matrix sample (negligible) | Analytically derived boundaries (negligible) | Single pass O(N*D) | None |
| **Theoretical guarantees** | None [cited: RaBitQ] | None | Sharp O(1/sqrt(D)) error bound, matches lower bound [cited: RaBitQ] | Asymptotically optimal space-error trade-off [cited: ExtRaBitQ] | Near-optimal within ~2.7x of info-theoretic lower bound [cited: TurboQuant]. Contested: may be suboptimal vs EDEN [cited: EDEN-note] | Uniform quantization bound per coordinate | Angular preservation (simhash) |
| **Recall at ~32x compr.** | Dataset-dependent; <60% on hard datasets [cited: RaBitQ] | Better than PQ, still no guarantees | Significantly better than PQ [cited: RaBitQ] | At 32x = 1-bit, same as RaBitQ | Claimed > PQ [cited: TurboQuant]; but <= RaBitQ on 3 datasets [cited: RvsTQ] | N/A (4x is typical) | 0.5-0.8 (requires re-ranking) |
| **Distance estimation** | ADC lookup tables (SIMD FastScan) | Same as PQ | Bitwise XOR+POPCNT or SIMD LUT [cited: RaBitQ] | Two-stage: MSB via FastScan, refinement via remaining bits [cited: ExtRaBitQ] | Scalar dequant + dot; or QJL for IP [cited: TurboQuant] | Integer dot product (VNNI/NEON) | Hamming (XOR+POPCNT) |
| **Rust availability** | None standalone; via `faiss-rs` bindings | Via `faiss-rs` | `rabitq-rs` (active, ARM64 bugs) [cited: web search] | `rabitq-rs` (variable-rate support) | `turbo-quant` crate [cited: Run 2 report] | Built into most vector libs | Trivial; widespread |
| **Production adoption** | Ubiquitous (FAISS, Milvus, Qdrant, Weaviate, Pinecone) | Wide (FAISS OPQ) | NVIDIA cuVS, LanceDB [cited: GPU-RaBitQ] | Early (SIGMOD 2025) | Early (ICLR 2026, Google Research) | Universal | Common as pre-filter |

---

## 3. Key Findings

### 3.1 RaBitQ dominates at 32x compression

At the standard 1-bit (32x) compression rate, RaBitQ provides the best accuracy-efficiency trade-off among all methods surveyed. It achieves this through a theoretically grounded mechanism (random rotation to hypercube vertices) that guarantees O(1/sqrt(D)) error with exponential concentration -- matching information-theoretic lower bounds. PQ has no such guarantees and empirically fails on datasets where sub-vector correlations violate independence assumptions (MSong, Word2Vec). [cited: RaBitQ]

### 3.2 Extended RaBitQ achieves asymptotic optimality at variable rates

For applications requiring recall >95%, the Extended RaBitQ method provides a principled way to trade compression ratio for accuracy. The empirical formula `epsilon < 2^(-B) * 5.75 / sqrt(D)` gives practitioners a direct way to size B for a target error budget. The two-stage IVF integration (MSB for coarse filtering, LSBs for refinement) preserves SIMD efficiency. [cited: ExtRaBitQ]

### 3.3 TurboQuant's position is contested

TurboQuant (ICLR 2026, Google Research) claims near-optimal distortion rates via data-oblivious scalar quantization. However, two independent critiques challenge these claims:

1. **RaBitQ team** [cited: RvsTQ]: Found TurboQuant performs worse than RaBitQ in "many tested configurations" at 2-4 bits across 3 datasets. Reported reproducibility issues with TurboQuant's published numbers (quantization times ~100x slower than claimed).

2. **EDEN team** [cited: EDEN-note]: Argues TurboQuant_mse is a special case of EDEN (ICML 2022) with a fixed scale parameter S=1 that is suboptimal except asymptotically. Demonstrates "2-bit EDEN beats 3-bit TurboQuant_prod" experimentally.

The shared insight across RaBitQ, TurboQuant, and EDEN is identical: random rotation concentrates coordinates toward a Beta distribution, enabling effective per-coordinate quantization. The genuine difference is in codebook design and reconstruction scaling. [cited: RvsTQ, EDEN-note]

### 3.4 GPU IVF-RaBitQ is a production-ready high-performance option

IVF-RaBitQ on GPU (integrated into NVIDIA cuVS) demonstrates compelling performance [cited: GPU-RaBitQ]:
- **2.2x higher QPS** than CAGRA (state-of-the-art graph method) at recall ~0.95
- **7.7x faster index build** than CAGRA
- **2.7x higher throughput** than IVF-PQ without re-ranking
- **<25% storage** of raw-vector methods
- No codebook training required
- Tested on datasets from 512 to 3072 dimensions, 1M to 10M vectors

This makes IVF-RaBitQ the first quantization-based method to consistently outperform graph-based GPU methods at high recall on modern embedding dimensions.

### 3.5 The data-oblivious property matters for production

RaBitQ, Extended RaBitQ, and TurboQuant are all data-oblivious: they require no training phase, no codebook learning, and no data-dependent optimization. This has significant production implications:
- **Zero indexing overhead**: Quantization is a simple per-vector operation (rotation + rounding)
- **No distribution drift**: The quantization is valid for any vector distribution
- **Streaming-friendly**: New vectors can be quantized independently without re-training
- **Deterministic**: Same input always produces same output (given fixed rotation matrix)

PQ and OPQ, by contrast, require periodic codebook re-training as data distributions evolve. [cited: RaBitQ, TurboQuant]

### 3.6 EDEN/DRIVE lineage predates TurboQuant

The DRIVE (NeurIPS 2021) and EDEN (ICML 2022) line of work established the random-rotation + optimal-scalar-quantizer paradigm for distributed gradient compression before TurboQuant applied it to ANN search. The EDEN team's note [cited: EDEN-note] demonstrates that EDEN's optimized scale parameter yields strictly better MSE and inner-product estimation than TurboQuant's fixed S=1 choice. Both exploit the same Beta-distribution concentration after rotation, the same Lloyd-Max quantizers, and the same Hadamard transform acceleration. The ANN search application is novel to TurboQuant; the quantization mechanism is not.

---

## 4. Lineage Diagram

```
Shannon Source Coding Theory
    |
    +-- Product Quantization (Jegou 2010)
    |       +-- OPQ (Ge 2013)
    |       +-- IVF-PQ (FAISS 2017)
    |
    +-- Scalar Quantization (classical)
    |       +-- int8/int4 variants
    |
    +-- Random Rotation + Hypercube Quantization
    |       +-- RaBitQ (Gao & Long, SIGMOD 2024)
    |       |       +-- Extended RaBitQ (Gao et al., SIGMOD 2025)
    |       |       +-- GPU IVF-RaBitQ (Shi et al., 2026) --> NVIDIA cuVS
    |       |
    |       +-- DRIVE (Vargaftik et al., NeurIPS 2021) [gradient compression]
    |               +-- EDEN (Vargaftik et al., ICML 2022) [multi-bit]
    |
    +-- Random Rotation + Scalar Quantization (per-coordinate)
    |       +-- PolarQuant (Han et al., AISTATS 2026) [polar coords]
    |       +-- QJL (Zandieh et al., ICML 2024) [1-bit JL]
    |       +-- TurboQuant (Zandieh et al., ICLR 2026) [PolarQuant + QJL]
    |
    +-- Binary / Sign-bit (Charikar 2002 / simhash)
```

---

## 5. Open Questions

1. **RaBitQ vs TurboQuant resolution**: The reproducibility dispute is unresolved as of April 2026. Independent benchmarks by third parties would clarify the true performance ordering.
2. **EDEN in ANN search**: EDEN's scalar quantization has not been formally evaluated for ANN search (only gradient compression). If EDEN's optimized S parameter truly gives "one free bit" over TurboQuant, an ANN-search evaluation would be valuable.
3. **ARM64 correctness**: `rabitq-rs` has documented critical bugs on ARM64/AArch64. This blocks Apple Silicon and Graviton deployments.
4. **Matryoshka + quantization**: How RaBitQ/TurboQuant interact with dimensionality-reduced embeddings (Matryoshka) is unexplored.
