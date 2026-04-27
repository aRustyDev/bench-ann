# Cohort A: Rust Memory Patterns for High-Performance HNSW

> **Date**: 2026-04-27
> **Context**: Analysis of what it would take to implement a pure Rust HNSW matching usearch's memory density and QPS
> **Related**: [cohort-a-usearch-vs-hnsw_rs-architecture.md](cohort-a-usearch-vs-hnsw_rs-architecture.md)

## The Problem

hnsw_rs uses idiomatic Rust patterns (Arc, Vec<Vec<T>>, RwLock) that optimize for safety and flexibility at the cost of memory density and cache locality. usearch's C++ tape-based layout achieves 149B graph overhead/vec vs hnsw_rs's ~2000B. A pure Rust HNSW could close this gap by applying arena and pool patterns, but each pattern has trade-offs with the borrow checker.

## Applicable Rust Memory Patterns

### 1. Bump Allocator (Arena Pattern)

**Mechanism**: A pointer that simply "bumps" forward in a contiguous block for each allocation. All allocations are freed at once when the arena is dropped.

**Crate**: `bumpalo` — fast, contiguous bump allocator.

**HNSW application**: Allocate all `Point` structs and neighbor metadata in a single arena during index build. Since HNSW indexes are typically build-once-query-many, the "free everything at once" semantics are a natural fit.

**What it solves**:
- Eliminates per-point `Arc` allocation overhead (~56 bytes per point)
- Eliminates per-`Vec<f32>` allocation header (24 bytes per vector)
- Guarantees spatial locality for points allocated close in time

**What it doesn't solve**:
- Doesn't ensure vectors are contiguous (points are allocated in insertion order, interleaved with neighbor metadata)
- Doesn't help with incremental delete (bumpalo can't free individual items)
- References into the arena are lifetime-bound (`&'arena T`), which complicates APIs that return owned data

**Applicability to HNSW**:
- Build-then-query workloads: excellent fit
- Incremental insert/delete: poor fit (need a different pattern)
- Thread safety: bumpalo arenas are not `Sync` — need one per thread during build, merged after

### 2. Slot-Based Object Pools (Generational Arena)

**Mechanism**: A `Vec<T>` acts as the pool. Deleted items leave "holes" tracked in a free list. Stable "generational IDs" (index + generation counter) replace pointers, preventing use-after-free and ABA problems.

**Crate**: `generational-arena` — industry standard for this pattern.

**HNSW application**: Store all graph nodes in a `generational-arena`. Neighbor lists reference nodes by `Index` (a generational ID) instead of `Arc<Point>`. This replaces:
```
Arc<PointWithOrder<T>> (68 bytes/edge)
```
with:
```
(Index, f32) ≈ (8 bytes + 4 bytes) = 12 bytes/edge
```
Or if distances are computed on-the-fly (usearch style):
```
Index ≈ 8 bytes/edge
```

**What it solves**:
- Eliminates Arc reference counting overhead entirely
- Supports individual item deletion (generational IDs detect stale references)
- Memory stays contiguous even after deletions (holes are reused)
- Safe — no raw pointers, no lifetime gymnastics

**What it doesn't solve**:
- Slight overhead from generation checks on every access (~1-2ns)
- Items within the arena are not necessarily ordered (fragmentation after deletes)
- Pool may have unused capacity (holes between active items)

**Applicability to HNSW**:
- Excellent for replacing Arc-based edge storage
- Supports incremental insert/delete (important for production HNSW)
- 12 bytes/edge is 5.7x better than hnsw_rs's 68 bytes, but still 3x worse than usearch's 4 bytes
- To match usearch: store only u32 neighbor IDs (4 bytes), not generational indices (8 bytes)

### 3. Fixed-Size Typed Pools

**Mechanism**: Fixed-size buffer of typed slots. Objects are stored in contiguous memory with O(1) alloc/free via free-list.

**Crate**: `axiom_mem` — typed object pool with contiguous storage.

**HNSW application**: Pre-allocate a pool of `Node` structs with fixed-size neighbor arrays:
```rust
struct Node {
    vector_offset: u32,       // index into flat vector buffer
    neighbors_l0: [u32; 2*M], // fixed-size, no Vec overhead
    neighbors_upper: [u32; M],
    level: u8,
}
```

**What it solves**:
- Zero per-node allocation overhead (all nodes in one contiguous block)
- Fixed-size neighbor arrays eliminate Vec<Arc<...>> overhead entirely
- Matches usearch's tape layout conceptually
- Cache-friendly: nodes can be iterated sequentially

**What it doesn't solve**:
- Fixed-size arrays waste space for nodes with fewer neighbors than max
- M must be known at compile time (or use a runtime-configurable max)
- Not in std — adds a dependency

**Applicability to HNSW**:
- Best match for usearch's memory model
- With `[u32; 2*M]` neighbor arrays: 128 bytes at M=16, matching usearch's ~149B
- Combined with a flat vector buffer: matches usearch's memory profile exactly

### 4. Flat Contiguous Vector Storage

**The critical pattern.** Do NOT use `Vec<Vec<f32>>` for 2D array-like data.

**Standard way**: Single flat `Vec<f32>` with index arithmetic: `vector_i = &buffer[i * dim..(i+1) * dim]`

**Crate alternative**: `ndarray` for n-dimensional contiguous arrays with BLAS integration.

**HNSW application**: All vectors in a single `Vec<f32>` of size `n * dim`. Nodes reference vectors by index (u32), not by pointer.

**What it solves**:
- Eliminates per-vector `Vec` overhead (24 bytes each)
- Sequential distance computations hit L1/L2 cache (vectors are adjacent in memory)
- SIMD-friendly: aligned, contiguous data
- This is the **single most impactful change** for QPS — cache locality during search

**What it doesn't solve**:
- Must know `dim` at index construction time (or store it as metadata)
- Adding vectors requires either pre-allocation or realloc (which invalidates all slices)
- Cannot store mixed-dimension vectors in one buffer

**Applicability to HNSW**:
- Essential for matching usearch QPS
- hnsw_rs already accepts `&[T]` slices — the adapter copies into `Vec<Vec<T>>` because hnsw_rs borrows from owned data. A flat buffer owned by the index would eliminate this copy.

### 5. Thread-Safe Pools

**Crate**: `shared-arena` — lock-free, thread-safe memory pool.

**Alternative**: `Arc<Mutex<Pool>>` for simpler but slower shared access.

**HNSW application**: During concurrent index build, multiple threads insert nodes. A thread-safe pool avoids per-allocation mutex contention.

**Applicability to HNSW**:
- Relevant for parallel build only (query is read-only, no allocation)
- `shared-arena` provides per-thread allocation with shared ownership
- Alternative: per-thread `bumpalo` arenas during build, merged into a single pool after build completes (no runtime synchronization during queries)

### Borrow Checker Constraints

A fundamental tension exists between contiguous pools and Rust's borrow checker:

**The reallocation problem**: If the pool (a `Vec`) reallocates to grow, all references into it become dangling. Raw pointers would be invalidated. This is why C++ can use `std::vector<Node>` with pointer-to-elements while Rust cannot safely hold `&Node` across a push that might realloc.

**Solutions (ranked by safety)**:
1. **Pre-allocate to capacity** — `Vec::with_capacity(max_n)`. No realloc ever happens. References are stable. Requires knowing max size upfront. (usearch does this via `reserve()`)
2. **Index-based access** — Never hold references; always access by `pool[index]`. Slight overhead per access but fully safe.
3. **Generational indices** — `generational-arena` validates that an index hasn't been recycled. Adds ~1ns per access.
4. **Pin + raw pointers** — `Pin<Box<[Node]>>` guarantees the buffer won't move. References are stable. Requires unsafe for raw pointer dereference.
5. **UnsafeCell** — Interior mutability for concurrent mutation. Maximum flexibility, maximum risk.

**For HNSW specifically**: Option 1 (pre-allocate) is the right choice. HNSW indexes know their capacity at build time (`reserve(n)` is standard). The graph is immutable during queries, so no reallocation risk.

## Synthesis: Blueprint for a usearch-Competitive Pure Rust HNSW

```
┌─────────────────────────────────────────────────────┐
│                  Index Struct                         │
│                                                       │
│  vectors: Vec<f32>  (flat buffer, n * dim)           │  ← Pattern #4
│  nodes: Vec<Node>   (pre-allocated to capacity)      │  ← Pattern #3
│  dim: usize                                           │
│  m: usize                                             │
│  entry_point: u32                                     │
│                                                       │
│  struct Node {                                        │
│      vector_idx: u32,          // into vectors buf    │
│      level: u8,                                       │
│      neighbors: [u32; 2*M],    // layer 0            │  ← Pattern #3
│      n_neighbors: [u8; MAX_LEVEL],                    │
│  }                                                    │
│                                                       │
│  // Query-time: no locks, no allocation               │
│  // Build-time: per-thread bumpalo for temporaries    │  ← Pattern #1
│  // Distances: computed on-the-fly, not stored        │
│  // SIMD: std::simd or pulp crate                     │
└─────────────────────────────────────────────────────┘

Estimated memory per vector (M=16, 128d):
  Vector data:     512 bytes (128 * 4)
  Node metadata:   4 + 1 = 5 bytes (vector_idx + level)
  Layer 0 edges:   128 bytes (32 * u32)
  Upper edges:     ~64 bytes (amortized across levels)
  Total overhead:  ~197 bytes/vec  (vs usearch 149B, hnsw_rs 2000B+)
```

This is achievable with **zero unsafe code** if using pre-allocated `Vec`s and index-based access. The ~48B gap vs usearch (197 vs 149) comes from Rust's `Vec` metadata and the slightly larger `u32` (4B) vs usearch's compressed addressing. Matching usearch exactly would require packed structs or custom allocation, which means unsafe.

## What's Benchmarkable vs What's a Rewrite

See [companion analysis](cohort-a-what-is-benchmarkable.md) for which of these patterns can be tested within our current benchmark harness vs which require a new implementation.
