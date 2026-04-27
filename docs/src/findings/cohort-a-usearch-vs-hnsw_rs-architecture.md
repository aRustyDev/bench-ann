# Cohort A: usearch vs hnsw_rs Architectural Analysis

> **Date**: 2026-04-26
> **Phase**: Targeted Research, Cohort A, Pass 1 source examination
> **Question**: Could a pure Rust HNSW match usearch's performance? What would it take?

## The Measured Gap

At M=32, 128d, 10K vectors (all else equal):

| Metric | usearch (C++/FFI) | hnsw_rs (pure Rust) | Ratio |
|--------|-------------------|---------------------|-------|
| Best QPS | 34,493 | 22,474 | **1.53x** |
| Mem overhead/vec | 521 B | 3,033 B | **5.8x** |
| Disk graph/vec | 149 B | 638 B | **4.3x** |
| Build time | 2.6s | 3.1s | 1.2x |

The QPS ratio is consistent at 1.53x across M=16 and M=32. The memory ratio is even larger.

## Root Cause Decomposition

The gap decomposes into four independent causes. Each contributes to both QPS and memory. Fixing any one improves both metrics.

### 1. Edge representation: 4-5 bytes vs ~68 bytes (biggest memory factor)

**usearch**: Uses a **tape-based node layout**: `[key (8B)] [level (2B)] [neighbors_count (4B)] [slot_0] [slot_1] ...`. Neighbor IDs stored as `compressed_slot_t` — either `uint32_t` (4B, datasets <4B vectors) or custom `uint40_t` (5B, up to 1 trillion vectors). Distances are **not stored** — computed on-the-fly during traversal.

**hnsw_rs**: Each edge is `Arc<PointWithOrder<T>>`:
- Arc control block: ~56 bytes (strong count, weak count, data pointer, allocator)
- Pointer to Point: 8 bytes  
- Distance (f32): 4 bytes
- **Total: ~68 bytes per edge** — 14-17x usearch

With M=32, layer 0 has up to 64 neighbors. That's:
- usearch: 64 * 4 = 256 bytes per node
- hnsw_rs: 64 * 68 = 4,352 bytes per node

**Why hnsw_rs does this**: Arc enables thread-safe shared ownership. Multiple threads can hold references to the same point during concurrent search. This is a design choice — safety over density.

**Could Rust match usearch?** Yes. Use indices (u32) into a pre-allocated vector/arena instead of Arc pointers. This eliminates the reference counting overhead entirely. The trade-off: lose automatic lifetime management; require a "graph owns everything" model.

### 2. Vector storage: contiguous pool vs Vec<Vec<f32>> (biggest QPS factor)

**usearch**: All vectors stored in a single contiguous allocation (C++ `aligned_alloc` or equivalent). Vector i is at offset `i * dim * sizeof(float)`. Zero per-vector overhead.

**hnsw_rs**: Each vector is `Vec<f32>` — a separate heap allocation per vector. Per-vector overhead: 24 bytes (ptr + len + cap). More importantly: **vectors are scattered across the heap**, causing cache misses during distance computation.

During search, HNSW computes distances between the query and ~ef candidate neighbors. With contiguous storage, these vectors may share cache lines. With scattered allocations, each distance computation likely causes an L2/L3 cache miss.

**Could Rust match usearch?** Yes. Allocate a single `Vec<f32>` of size `n * dim` (a flat buffer). Access vector i as `&buffer[i*dim..(i+1)*dim]`. This is the exact same pattern as usearch. No unsafe code needed — just index arithmetic.

The challenge: hnsw_rs's API borrows from owned data (`&'b [T]`), requiring the caller to keep the data alive. A contiguous buffer owned by the index struct solves this cleanly.

### 3. SIMD distance computation (moderate QPS factor)

**usearch**: Delegates to **SimSIMD**, a dedicated SIMD kernel library with 200+ hand-tuned routines:
- x86_64: SSE, AVX2, AVX-512 (FP16 handling)
- ARM64: NEON, SVE, SVE2
- Key optimizations: masked loads (eliminate tail loops), Horner's method for polynomial approximation (119x speedup vs GCC 12 in some paths), bit-hack sqrt replacement
- Explicit `_mm_prefetch()` calls in graph traversal loops

**hnsw_rs**: Delegates to `anndists` crate. Default: scalar loops (compiler auto-vectorization). Optional: `simdeez_f` feature (x86_64 only), `stdsimd` feature (nightly Rust only). Neither is enabled by default.

**Could Rust match usearch?** Mostly. Rust's `std::simd` (nightly) or crates like `pulp`, `simdeez`, or `wide` provide explicit SIMD. The `simsimd` crate (Cohort X) wraps the same C library that usearch uses internally. A pure Rust HNSW with `std::simd` or `pulp` would be within ~10-20% of C++ SIMD on the same ISA.

The gap: C++ can more easily dispatch across ISA variants at runtime (single binary, CPUID check). Rust's `std::simd` does this but is nightly-only. Stable Rust requires feature detection + function pointer dispatch, which adds ~5% overhead.

### 4. Graph traversal and search infrastructure (minor-moderate factor)

**usearch**: Several micro-optimizations compound:
- **Visited set**: Hash-set with linear probing (power-of-two capacity, sentinel initialization). O(1) average. Not a bitset — trades memory for speed.
- **Top candidates**: Sorted buffer with binary search insertion (O(log K)).
- **Thread-local context**: Reusable heap/visited-set per thread. Zero allocation during query.
- **Prefetch**: Explicit `_mm_prefetch()` in traversal loops — prefetches neighbor vectors before computing distance.

**hnsw_rs**: Standard Rust collections:
- `BinaryHeap<T>` for candidates (standard library)
- `Arc<RwLock<...>>` per node — RwLock adds ~48-56 bytes per node
- No thread-local context caching — allocates per query
- No explicit prefetch instructions

**Could Rust match usearch?** Yes, for the single-threaded case. Use `UnsafeCell` for single-threaded builds, or `parking_lot::RwLock` (smaller than std) for concurrent. If graph is immutable after build, no locking needed at query time. Thread-local context reuse is straightforward in Rust (`thread_local!` macro). Prefetch requires `core::arch` intrinsics but is easy to add.

## Can a Pure Rust HNSW Match usearch?

### Short answer: Yes, within ~10-20% on QPS, and matching on memory.

### What it would take:

```
Contiguous vector storage:      +15-25% QPS (cache locality)
u32 index-based edges:          +5-10% QPS, -80% memory (no Arc)
Explicit SIMD (std::simd/pulp): +10-15% QPS (vs auto-vectorize)
No per-node RwLock at query:    +2-5% QPS
────────────────────────────────────────────────────────
Combined estimate:               ~1.4-1.6x current hnsw_rs
usearch advantage:               1.53x
```

A pure Rust HNSW with all four optimizations would be roughly at parity with usearch. The remaining ~5-10% gap would come from:
- C++ compiler auto-vectorization is slightly better than Rust's in some patterns
- C++ can inline across the FFI boundary (usearch is header-only)
- Runtime ISA dispatch is more mature in C++

### Would eliminating the FFI barrier help?

The FFI boundary between Rust and C++ (via CXX) adds negligible overhead per call (~1-2ns). For the benchmark workload (thousands of searches per second), FFI cost is <0.1% of total time. Eliminating it wouldn't measurably help.

However, a pure Rust implementation has other advantages:
- **Composability**: Can be used as a library component in other Rust data structures
- **No C++ build dependency**: Simpler CI/CD, cross-compilation, WASM target
- **Trait-based extensibility**: Custom distance functions, custom point types, filtering
- **Safety guarantees**: No unsound C++ code to audit

### Would Arenas/Typed Pools close the gap?

**Yes, for memory.** An arena-allocated HNSW would match usearch's memory profile:
- `bumpalo` or `typed-arena` for point storage: eliminates per-vector Vec overhead
- Fixed-size neighbor arrays in the arena: eliminates per-edge Arc overhead
- Graph overhead would be ~150-250B/vec (same ballpark as usearch's 149B)

**Partially, for QPS.** Arenas improve allocation patterns but don't automatically give you:
- Contiguous vector layout (need to design this explicitly)
- SIMD distance computation (orthogonal concern)
- Cache-optimal graph traversal order (usearch likely reorders nodes by locality)

The key insight: **arenas are necessary but not sufficient**. You also need contiguous vector storage and explicit SIMD to match usearch's QPS.

## usearch Limitations vs hnsw_rs

### What you lose with usearch:

| Capability | hnsw_rs | usearch |
|-----------|---------|---------|
| **Generic data types** | `T: Clone + Send + Sync + Serialize` | f32, f64, f16, i8, u8 only |
| **Custom distance fn** | Via `Distance<T>` trait (anndists) | Via raw pointer callback (`*const T, *const T -> f32`) — unsafe |
| **Graph introspection** | Full layer access, point lookup by ID, iteration | Opaque — no access to graph structure |
| **Filtered search API** | `FilterT` trait (Vec, closure, custom impl) | Closure `Fn(Key) -> bool` only |
| **Parallel insert** | `parallel_insert_slice()` via rayon | Requires manual thread management |
| **Memory mapping** | Optional mmap for vectors (graph in RAM) | `view()` mmaps entire index (less granular) |
| **Graph flattening** | `FlatNeighborhood` for compact read-only graphs | Not available |
| **Incremental updates** | Delete + reinsert pattern | `remove(key)` native support |
| **Multi-vector keys** | Not supported | `multi: true` allows multiple vectors per key |

### What you gain with usearch:

| Capability | usearch | hnsw_rs |
|-----------|---------|---------|
| **Multi-precision** | f32/f16/i8/u8 (quantized storage) | f32 only (via adapter) |
| **Built-in quantization** | ScalarKind enum at index creation | Must be done externally |
| **Query-time metric switch** | `change_metric_kind()` | Must rebuild index |
| **Memory efficiency** | 149B graph overhead/vec | ~2000B graph overhead/vec |
| **Raw throughput** | 1.5x QPS advantage | — |
| **Save/load simplicity** | `save(path)` / `load(path)` | Custom dump format, manual layer handling |

### The real trade-off

usearch is a **sealed, optimized appliance**. hnsw_rs is an **open, flexible toolkit**. If you need to:
- Plug in custom distance functions for exotic types → hnsw_rs
- Introspect graph structure for research → hnsw_rs
- Build a filtered search product → both work (different APIs)
- Maximize throughput on standard types → usearch
- Minimize memory footprint → usearch
- Embed in a larger Rust system without C++ deps → hnsw_rs

## Open Questions for Pass 2

1. Does hnsw_rs with `simdeez_f` feature enabled close the QPS gap? (Quantify SIMD contribution)
2. At 100K-1M scale, does usearch's cache advantage grow or shrink?
3. How do filtered search implementations compare at scale? (hnsw_rs Filterable trait vs usearch predicate fn)
4. What is instant-distance's actual graph overhead after fixing the serialization duplication?
