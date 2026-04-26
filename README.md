# Vector DS&A Benchmark Harness

Shared benchmark harness for cross-cohort ANN crate evaluation. Produces directly comparable results across all adapter crates using a common trait interface, measurement protocols, and output schema.

## Build

```bash
cd benchmarks/vector-dsa
cargo build --workspace
cargo test --workspace
```

## Download SIFT1M

```bash
# Primary source
wget ftp://ftp.irisa.fr/local/texmex/corpus/sift.tar.gz
tar xzf sift.tar.gz

# Fallback: HuggingFace mirror
# https://huggingface.co/datasets/qbo-odp/sift1m
```

Extract to a directory (e.g., `data/sift/`). Files needed:
- `sift_base.fvecs` (1M vectors, 128d)
- `sift_query.fvecs` (10K queries, 128d)

## Run First Benchmark

```bash
# 1. Compute ground truth
cargo run -p ann-bench-cli -- ground-truth \
  --dataset sift-128 \
  --data-dir data/sift \
  --metric euclidean \
  --k 100

# 2. List available adapters
cargo run -p ann-bench-cli -- list

# 3. Run hnsw_rs adapter tests (end-to-end validation)
cargo test -p ann-bench-hnsw-rs -- --nocapture
```

## Workspace Structure

```
ann-bench-core/        Core trait (AnnIndex), types, tier classification, Pareto frontier
ann-bench-datasets/    fvecs loader, synthetic generator, ground truth computation
ann-bench-harness/     Runner pipeline, measurement, recall, memory, filtered/incremental
ann-bench-cli/         CLI binary: run, ground-truth, pareto, tier, list commands
adapters/
  ann-bench-hnsw-rs/   First adapter (hnsw_rs) for end-to-end validation
```

## Adding Adapters

Each adapter is a separate workspace crate under `adapters/`. See `adapters/ann-bench-hnsw-rs/` for the reference implementation. An adapter must:

1. Implement the `AnnIndex` trait from `ann-bench-core`
2. Declare a default parameter sweep
3. Declare supported features via `supports_*` methods
4. Be added to the workspace `Cargo.toml` members list
