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

adapters := "hnsw-rs usearch instant-distance"
dims := "128 384 768 1536"

# Quick validation sweep: 10K vectors, 1K queries (~5 min total)
sweep-10k:
    @echo "═══ 10K Sweep: 3 adapters × 4 dims ═══"
    @just _sweep 10000 1000 results/10k ground_truth/10k
    @echo "═══ 10K Sweep Complete ═══"

# Research-grade sweep: 100K vectors, 1K queries (~1-2 hr total)
sweep-100k:
    @echo "═══ 100K Sweep: 3 adapters × 4 dims ═══"
    @just _sweep 100000 1000 results/100k ground_truth/100k
    @echo "═══ 100K Sweep Complete ═══"

# Publication-grade sweep: 1M vectors, 1K queries (~6-12 hr total)
# Uses 1K queries (not 10K) to stay within 32GB RAM at 1536d.
sweep-1m:
    @echo "═══ 1M Sweep: 3 adapters × 4 dims ═══"
    @just _sweep 1000000 1000 results/1m ground_truth/1m
    @echo "═══ 1M Sweep Complete ═══"

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
sweep-m32 n_vectors="10000" n_queries="1000" output_dir="results/m32":
    @echo "═══ M=32 Sweep: hnsw-rs + usearch × 4 dims ═══"
    @for dim in {{dims}}; do \
        for adapter in hnsw-rs usearch; do \
            echo "--- $adapter M=32 @ ${dim}d ---"; \
            cargo run --release -p ann-bench-cli -- run \
                --adapter "$adapter" \
                --dataset "synthetic-$dim" \
                --n-vectors {{n_vectors}} \
                --n-queries {{n_queries}} \
                --runs 3 \
                -M 32 \
                --output-dir {{output_dir}} \
                --gt-dir ground_truth; \
        done; \
    done
    @echo "═══ M=32 Sweep Complete ═══"

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
