# ANN Benchmark Harness — Reproducible Experiments
#
# Usage:
#   just build          Build harness in release mode
#   just test           Run all tests
#   just sweep-10k      Quick validation (10K vectors, ~5 min)
#   just sweep-100k     Research-grade (100K vectors, ~1-2 hr)
#   just sweep-1m       Publication-grade (1M vectors, ~6-12 hr)
#   just summary DIR    Print comparison table from result JSON files
#
# Prerequisites:
#   - Rust toolchain (stable)
#   - just (cargo install just)

# Default: show available recipes
default:
    @just --list

# ─── Build & Test ─────────────────────────────────────────────

# Build the full workspace in release mode
build:
    cargo build --release --workspace

# Run all workspace tests
test:
    cargo test --workspace

# List available adapters and datasets
list:
    cargo run --release -p ann-bench-cli -- list

# ─── Individual Benchmark Runs ────────────────────────────────

# Run a single benchmark
# Example: just run hnsw-rs synthetic-128 10000 1000 16 results
run adapter dataset n_vectors="10000" n_queries="1000" m="16" output_dir="results":
    cargo run --release -p ann-bench-cli -- run \
        --adapter {{adapter}} \
        --dataset {{dataset}} \
        --n-vectors {{n_vectors}} \
        --n-queries {{n_queries}} \
        --runs 3 \
        -M {{m}} \
        --output-dir {{output_dir}} \
        --gt-dir ground_truth

# ─── Sweep Recipes ────────────────────────────────────────────
# Each sweep runs all 3 Cohort A adapters across 4 dimensionalities.
# Ground truth is cached and shared across adapters.
# Sweeps skip runs whose output JSON already exists (resumable).

adapters := "hnsw-rs usearch instant-distance"
dims := "128 384 768 1536"

# Quick validation: 10K vectors, 1K queries
# ~5 min, ~0.6 GB peak RAM, 12 runs
sweep-10k confirm="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{confirm}}" != "yes" ]; then
        echo ""
        echo "  sweep-10k: Quick validation sweep"
        echo "  ─────────────────────────────────"
        echo "  Vectors:    10,000"
        echo "  Queries:    1,000"
        echo "  Adapters:   hnsw-rs, usearch, instant-distance"
        echo "  Dimensions: 128, 384, 768, 1536"
        echo "  Runs:       12 (3 adapters × 4 dims)"
        echo "  Est. time:  ~5 minutes"
        echo "  Peak RAM:   ~0.6 GB"
        echo "  Disk:       ~5 MB results"
        echo ""
        echo "  Run with: just sweep-10k yes"
        echo ""
        exit 0
    fi
    echo "═══ 10K Sweep: 3 adapters × 4 dims ═══"
    just _sweep 10000 1000 results/10k ground_truth/10k
    echo "═══ 10K Sweep Complete ═══"

# Research-grade: 100K vectors, 1K queries
# ~1-2 hours, ~9 GB peak RAM, 12 runs
sweep-100k confirm="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{confirm}}" != "yes" ]; then
        echo ""
        echo "  sweep-100k: Research-grade sweep"
        echo "  ────────────────────────────────"
        echo "  Vectors:    100,000"
        echo "  Queries:    1,000"
        echo "  Adapters:   hnsw-rs, usearch, instant-distance"
        echo "  Dimensions: 128, 384, 768, 1536"
        echo "  Runs:       12 (3 adapters × 4 dims)"
        echo "  Est. time:  ~1-2 hours"
        echo "  Peak RAM:   ~9 GB (at 1536d)"
        echo "  Disk:       ~50 MB results + ground truth"
        echo ""
        echo "  Run with: just sweep-100k yes"
        echo ""
        exit 0
    fi
    echo "═══ 100K Sweep: 3 adapters × 4 dims ═══"
    just _sweep 100000 1000 results/100k ground_truth/100k
    echo "═══ 100K Sweep Complete ═══"

# Publication-grade: 1M vectors, 1K queries
# ~6-12 hours, ~15 GB peak RAM, 12 runs
# Uses 1K queries (not 10K) to stay within 32GB RAM at 1536d.
sweep-1m confirm="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{confirm}}" != "yes" ]; then
        echo ""
        echo "  sweep-1m: Publication-grade sweep"
        echo "  ──────────────────────────────────"
        echo "  Vectors:    1,000,000"
        echo "  Queries:    1,000"
        echo "  Adapters:   hnsw-rs, usearch, instant-distance"
        echo "  Dimensions: 128, 384, 768, 1536"
        echo "  Runs:       12 (3 adapters × 4 dims)"
        echo "  Est. time:  ~6-12 hours (overnight recommended)"
        echo "  Peak RAM:   ~15 GB (at 1536d)"
        echo "  Disk:       ~500 MB results + ground truth"
        echo "  Min RAM:    32 GB required"
        echo ""
        echo "  Recommended: nohup just sweep-1m yes > sweep-1m.log 2>&1 &"
        echo "  Monitor:     grep -c Done sweep-1m.log  # out of 12"
        echo ""
        echo "  Run with: just sweep-1m yes"
        echo ""
        exit 0
    fi
    echo "═══ 1M Sweep: 3 adapters × 4 dims ═══"
    just _sweep 1000000 1000 results/1m ground_truth/1m
    echo "═══ 1M Sweep Complete ═══"

# Pre-compute ground truth for all dimensions at a given scale.
# Runs sequentially (one dim at a time) to manage peak memory.
# GT is cached — subsequent benchmark runs skip computation.
precompute-gt n_vectors="1000000" n_queries="1000" gt_dir="ground_truth/1m":
    #!/usr/bin/env bash
    set -euo pipefail
    echo "═══ Pre-computing ground truth ({{n_vectors}} vectors, {{n_queries}} queries) ═══"
    for dim in {{dims}}; do
        cache="{{gt_dir}}/synthetic-${dim}_euclidean_k100"
        if [ -d "$cache" ]; then
            echo "--- SKIP ${dim}d GT (cached at $cache) ---"
        else
            echo "--- Computing GT: ${dim}d ({{n_vectors}} vectors, {{n_queries}} queries) ---"
            cargo run --release -p ann-bench-cli -- ground-truth \
                --dataset "synthetic-$dim" \
                --n-vectors {{n_vectors}} \
                --n-queries {{n_queries}} \
                --metric euclidean \
                --k 100 \
                --output-dir {{gt_dir}}
        fi
    done
    echo "═══ Ground truth pre-computation complete ═══"

# Internal: run a sweep with skip-if-exists checkpointing.
# Skips runs whose output JSON already exists. Commits after each dimension.
_sweep n_vectors n_queries output_dir gt_dir:
    @for dim in {{dims}}; do \
        for adapter in {{adapters}}; do \
            outfile="{{output_dir}}/$( \
                echo "$adapter" | sed 's/-/_/g; s/instant_distance/instant-distance/' \
            )_synthetic-${dim}_M=*.json"; \
            if ls $outfile 1>/dev/null 2>&1; then \
                echo "--- SKIP $adapter @ ${dim}d (exists) ---"; \
            else \
                echo "--- $adapter @ ${dim}d ({{n_vectors}} vecs) ---"; \
                cargo run --release -p ann-bench-cli -- run \
                    --adapter "$adapter" \
                    --dataset "synthetic-$dim" \
                    --n-vectors {{n_vectors}} \
                    --n-queries {{n_queries}} \
                    --runs 3 \
                    --output-dir {{output_dir}} \
                    --gt-dir {{gt_dir}}; \
            fi; \
        done; \
        echo "=== Checkpoint: ${dim}d complete ==="; \
    done

# Fair M=32 comparison (instant-distance's hardcoded M)
sweep-m32 n_vectors="10000" n_queries="1000" output_dir="results/m32" confirm="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{confirm}}" != "yes" ]; then
        echo ""
        echo "  sweep-m32: Fair M=32 comparison"
        echo "  ───────────────────────────────"
        echo "  Only hnsw-rs + usearch (instant-distance is always M=32)"
        echo "  Vectors:    {{n_vectors}}"
        echo "  Queries:    {{n_queries}}"
        echo "  Runs:       8 (2 adapters × 4 dims)"
        echo ""
        echo "  Run with: just sweep-m32 yes"
        echo ""
        exit 0
    fi
    echo "═══ M=32 Sweep: hnsw-rs + usearch × 4 dims ═══"
    for dim in {{dims}}; do
        for adapter in hnsw-rs usearch; do
            echo "--- $adapter M=32 @ ${dim}d ---"
            cargo run --release -p ann-bench-cli -- run \
                --adapter "$adapter" \
                --dataset "synthetic-$dim" \
                --n-vectors {{n_vectors}} \
                --n-queries {{n_queries}} \
                --runs 3 \
                -M 32 \
                --output-dir {{output_dir}} \
                --gt-dir ground_truth
        done
    done
    echo "═══ M=32 Sweep Complete ═══"

# Remove all results and ground truth (fresh start)
clean confirm="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ "{{confirm}}" != "yes" ]; then
        echo ""
        echo "  clean: Remove ALL results and ground truth caches"
        echo "  ──────────────────────────────────────────────────"
        echo "  This deletes:"
        echo "    results/     — all benchmark JSON files"
        echo "    ground_truth/ — all cached brute-force results"
        echo ""
        echo "  Run with: just clean yes"
        echo ""
        exit 0
    fi
    rm -rf results ground_truth
    echo "Cleaned all results and ground truth caches."

# ─── Analysis ─────────────────────────────────────────────────

# Print a comparison summary table from a results directory
summary dir="results":
    python3 scripts/summary.py {{dir}}

# Detailed Pareto frontier comparison
pareto dir="results":
    python3 scripts/pareto.py {{dir}}

# ─── Toolchain Info ───────────────────────────────────────────

# Print environment info for reproducibility
env:
    @echo "Rust toolchain:"
    @rustc --version
    @cargo --version
    @echo ""
    @echo "Platform:"
    @uname -srm
    @echo ""
    @echo "CPU:"
    @sysctl -n machdep.cpu.brand_string 2>/dev/null || cat /proc/cpuinfo 2>/dev/null | grep "model name" | head -1 || echo "unknown"
    @echo ""
    @echo "RAM:"
    @python3 -c "import os; print(f'{os.sysconf(\"SC_PAGE_SIZE\") * os.sysconf(\"SC_PHYS_PAGES\") / (1024**3):.0f} GB')" 2>/dev/null || echo "unknown"
