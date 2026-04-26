use std::path::PathBuf;

use clap::Args;

use ann_bench_core::DistanceMetric;
use ann_bench_datasets::{fvecs, ground_truth, synthetic};

#[derive(Args)]
pub struct GroundTruthArgs {
    /// Dataset name (sift-128, synthetic-128, etc.)
    #[arg(long)]
    pub dataset: String,

    /// Distance metric
    #[arg(long, default_value = "euclidean")]
    pub metric: String,

    /// Number of neighbors for ground truth
    #[arg(long, default_value_t = 100)]
    pub k: usize,

    /// Path to dataset directory (for fvecs datasets)
    #[arg(long)]
    pub data_dir: Option<PathBuf>,

    /// Number of vectors (for synthetic datasets)
    #[arg(long, default_value_t = 1_000_000)]
    pub n_vectors: usize,

    /// Number of queries (for synthetic datasets)
    #[arg(long, default_value_t = 10_000)]
    pub n_queries: usize,

    /// Output directory for cached ground truth
    #[arg(long, default_value = "ground_truth")]
    pub output_dir: PathBuf,
}

pub fn execute(args: GroundTruthArgs) -> anyhow::Result<()> {
    let metric = parse_metric(&args.metric)?;

    let (dim, base, n_base, queries, n_queries) = load_dataset(&args)?;

    eprintln!(
        "Computing ground truth: {} queries × {} base vectors, dim={}, k={}, metric={}",
        n_queries, n_base, dim, args.k, metric
    );

    let gt = ground_truth::compute_ground_truth(
        &base, n_base, &queries, n_queries, dim, args.k, metric,
    );

    let cache_dir = args.output_dir.join(format!(
        "{}_{}_k{}",
        args.dataset, args.metric, args.k
    ));
    ground_truth::save_ground_truth(&gt, &cache_dir)?;
    eprintln!("Ground truth saved to {}", cache_dir.display());

    Ok(())
}

fn parse_metric(s: &str) -> anyhow::Result<DistanceMetric> {
    match s.to_lowercase().as_str() {
        "euclidean" | "l2" => Ok(DistanceMetric::Euclidean),
        "cosine" => Ok(DistanceMetric::Cosine),
        "dot" | "dotproduct" | "ip" => Ok(DistanceMetric::DotProduct),
        other => anyhow::bail!("unknown metric: {other}"),
    }
}

/// Returns (dim, base_vectors, n_base, queries, n_queries).
#[allow(clippy::type_complexity)]
fn load_dataset(args: &GroundTruthArgs) -> anyhow::Result<(usize, Vec<f32>, usize, Vec<f32>, usize)> {
    if args.dataset.starts_with("sift") {
        let data_dir = args.data_dir.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--data-dir required for fvecs datasets"))?;

        let (dim, base) = fvecs::load_fvecs(&data_dir.join("sift_base.fvecs"))?;
        let n_base = base.len() / dim;
        let (qd, queries) = fvecs::load_fvecs(&data_dir.join("sift_query.fvecs"))?;
        anyhow::ensure!(dim == qd, "base dim ({dim}) != query dim ({qd})");
        let n_queries = queries.len() / dim;

        Ok((dim, base, n_base, queries, n_queries))
    } else if args.dataset.starts_with("synthetic-") {
        let dim: usize = args.dataset
            .strip_prefix("synthetic-")
            .unwrap()
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid synthetic dimension in dataset name"))?;

        let seed = match dim {
            128 => synthetic::seeds::GAUSSIAN_128,
            384 => synthetic::seeds::GAUSSIAN_384,
            768 => synthetic::seeds::GAUSSIAN_768,
            1536 => synthetic::seeds::GAUSSIAN_1536,
            _ => 42_000_000 + dim as u64,
        };

        let ds = synthetic::generate_synthetic(
            args.n_vectors,
            args.n_queries,
            dim,
            synthetic::SyntheticDistribution::Gaussian,
            seed,
        );

        Ok((dim, ds.vectors, ds.n_vectors, ds.queries, ds.n_queries))
    } else {
        anyhow::bail!("unknown dataset: {}", args.dataset)
    }
}
