use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct RunArgs {
    /// Adapter to benchmark (hnsw-rs, usearch, etc.)
    #[arg(long)]
    pub adapter: String,

    /// Dataset name (sift-128, synthetic-128, etc.)
    #[arg(long)]
    pub dataset: String,

    /// Distance metric
    #[arg(long, default_value = "euclidean")]
    pub metric: String,

    /// Path to dataset directory (for fvecs datasets)
    #[arg(long)]
    pub data_dir: Option<PathBuf>,

    /// Number of vectors (for synthetic datasets)
    #[arg(long, default_value_t = 1_000_000)]
    pub n_vectors: usize,

    /// Number of queries (for synthetic datasets)
    #[arg(long, default_value_t = 10_000)]
    pub n_queries: usize,

    /// Number of neighbors to retrieve
    #[arg(long, default_value_t = 10)]
    pub k: usize,

    /// Number of measurement runs
    #[arg(long, default_value_t = 3)]
    pub runs: usize,

    /// Output directory for result JSON files
    #[arg(long, default_value = "results")]
    pub output_dir: PathBuf,

    /// Ground truth cache directory
    #[arg(long, default_value = "ground_truth")]
    pub gt_dir: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn execute(args: RunArgs) -> anyhow::Result<()> {
    // Adapter dispatch: each adapter crate registers itself here.
    // For now, adapters are run via their own integration tests since
    // linking all adapters into the CLI binary requires feature flags
    // that will be set up per-cohort.
    let known = ["hnsw-rs"];

    if known.contains(&args.adapter.as_str()) {
        println!("Adapter '{}' is registered.", args.adapter);
        println!();
        println!("To run benchmarks, use the adapter's integration test:");
        println!(
            "  cargo test -p ann-bench-{} --test integration -- --nocapture",
            args.adapter
        );
        println!();
        println!("Direct CLI dispatch will be available once adapter feature");
        println!("flags are configured for the target cohort.");
    } else {
        anyhow::bail!(
            "unknown adapter: {}. Run `ann-bench list --adapters` to see available adapters.",
            args.adapter
        );
    }

    Ok(())
}
