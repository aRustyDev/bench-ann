use std::path::PathBuf;

use clap::Args;

use ann_bench_core::*;
use ann_bench_datasets::{fvecs, ground_truth, synthetic};
use ann_bench_harness::runner::{self, BenchmarkRunConfig};

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
    match args.adapter.as_str() {
        "hnsw-rs" => run_hnsw_rs(&args),
        "usearch" => run_usearch(&args),
        "instant-distance" => run_instant_distance(&args),
        other => {
            anyhow::bail!(
                "unknown adapter: {other}. Run `ann-bench list --adapters` to see available adapters."
            );
        }
    }
}

fn run_hnsw_rs(args: &RunArgs) -> anyhow::Result<()> {
    use ann_bench_hnsw_rs::{default_sweep, HnswBuildConfig};

    let metric = parse_metric(&args.metric)?;
    let (dim, base, n_base, queries, n_queries) = load_dataset(args)?;

    eprintln!(
        "Running hnsw-rs on {} ({} vectors, {}d, {})",
        args.dataset, n_base, dim, args.metric
    );

    // Load or compute ground truth
    let gt_k = 100;
    let gt_cache = args.gt_dir.join(format!(
        "{}_{}_k{}",
        args.dataset, args.metric, gt_k
    ));
    let gt = if gt_cache.exists() {
        eprintln!("  Loading cached ground truth from {}", gt_cache.display());
        ground_truth::load_ground_truth(&gt_cache)?
    } else {
        eprintln!("  Computing ground truth (k={gt_k})...");
        let gt = ground_truth::compute_ground_truth(
            &base, n_base, &queries, n_queries, dim, gt_k, metric,
        );
        ground_truth::save_ground_truth(&gt, &gt_cache)?;
        eprintln!("  Cached to {}", gt_cache.display());
        gt
    };

    let build_config = HnswBuildConfig::default();
    let query_configs = default_sweep();

    let hardware = HardwareInfo {
        cpu: detect_cpu(),
        cores_used: 1,
        ram_gb: detect_ram_gb(),
        os: detect_os(),
        storage: "NVMe SSD".to_string(),
    };

    let dataset_info = DatasetInfo {
        name: args.dataset.clone(),
        source: if args.dataset.starts_with("sift") { "fvecs" } else { "synthetic" }.to_string(),
        n_vectors: n_base,
        n_queries,
        dimension: dim,
        metric: args.metric.clone(),
    };

    std::fs::create_dir_all(&args.output_dir)?;

    let config = BenchmarkRunConfig {
        crate_version: "0.3.4",
        hardware,
        dataset: dataset_info,
        base_vectors: &base,
        n_base,
        queries: &queries,
        n_queries,
        dim,
        metric,
        build_config,
        query_configs,
        k: args.k,
        n_runs: args.runs,
        ground_truth: &gt,
        output_dir: &args.output_dir,
    };

    let result = runner::run_benchmark::<ann_bench_hnsw_rs::HnswIndex>(config)?;

    eprintln!(
        "\n  Done. Best recall@10: {:.4}, Best QPS: {:.0}",
        result.query_sweeps.iter().map(|s| s.recall_at_10).fold(0.0_f64, f64::max),
        result.query_sweeps.iter().map(|s| s.qps).fold(0.0_f64, f64::max),
    );

    Ok(())
}

fn run_usearch(args: &RunArgs) -> anyhow::Result<()> {
    use ann_bench_usearch::{default_sweep, UsearchBuildConfig};

    let metric = parse_metric(&args.metric)?;
    let (dim, base, n_base, queries, n_queries) = load_dataset(args)?;

    eprintln!(
        "Running usearch on {} ({} vectors, {}d, {})",
        args.dataset, n_base, dim, args.metric
    );

    let gt_k = 100;
    let gt_cache = args.gt_dir.join(format!(
        "{}_{}_k{}",
        args.dataset, args.metric, gt_k
    ));
    let gt = if gt_cache.exists() {
        eprintln!("  Loading cached ground truth from {}", gt_cache.display());
        ground_truth::load_ground_truth(&gt_cache)?
    } else {
        eprintln!("  Computing ground truth (k={gt_k})...");
        let gt = ground_truth::compute_ground_truth(
            &base, n_base, &queries, n_queries, dim, gt_k, metric,
        );
        ground_truth::save_ground_truth(&gt, &gt_cache)?;
        eprintln!("  Cached to {}", gt_cache.display());
        gt
    };

    let build_config = UsearchBuildConfig::default();
    let query_configs = default_sweep();

    let hardware = HardwareInfo {
        cpu: detect_cpu(),
        cores_used: 1,
        ram_gb: detect_ram_gb(),
        os: detect_os(),
        storage: "NVMe SSD".to_string(),
    };

    let dataset_info = DatasetInfo {
        name: args.dataset.clone(),
        source: if args.dataset.starts_with("sift") { "fvecs" } else { "synthetic" }.to_string(),
        n_vectors: n_base,
        n_queries,
        dimension: dim,
        metric: args.metric.clone(),
    };

    std::fs::create_dir_all(&args.output_dir)?;

    let config = BenchmarkRunConfig {
        crate_version: "2.25.1",
        hardware,
        dataset: dataset_info,
        base_vectors: &base,
        n_base,
        queries: &queries,
        n_queries,
        dim,
        metric,
        build_config,
        query_configs,
        k: args.k,
        n_runs: args.runs,
        ground_truth: &gt,
        output_dir: &args.output_dir,
    };

    let result = runner::run_benchmark::<ann_bench_usearch::UsearchIndex>(config)?;

    eprintln!(
        "\n  Done. Best recall@10: {:.4}, Best QPS: {:.0}",
        result.query_sweeps.iter().map(|s| s.recall_at_10).fold(0.0_f64, f64::max),
        result.query_sweeps.iter().map(|s| s.qps).fold(0.0_f64, f64::max),
    );

    Ok(())
}

fn run_instant_distance(args: &RunArgs) -> anyhow::Result<()> {
    use ann_bench_instant_distance::{default_sweep, InstantDistanceBuildConfig};

    let metric = parse_metric(&args.metric)?;
    let (dim, base, n_base, queries, n_queries) = load_dataset(args)?;

    eprintln!(
        "Running instant-distance on {} ({} vectors, {}d, {})",
        args.dataset, n_base, dim, args.metric
    );

    let gt_k = 100;
    let gt_cache = args.gt_dir.join(format!(
        "{}_{}_k{}",
        args.dataset, args.metric, gt_k
    ));
    let gt = if gt_cache.exists() {
        eprintln!("  Loading cached ground truth from {}", gt_cache.display());
        ground_truth::load_ground_truth(&gt_cache)?
    } else {
        eprintln!("  Computing ground truth (k={gt_k})...");
        let gt = ground_truth::compute_ground_truth(
            &base, n_base, &queries, n_queries, dim, gt_k, metric,
        );
        ground_truth::save_ground_truth(&gt, &gt_cache)?;
        eprintln!("  Cached to {}", gt_cache.display());
        gt
    };

    let build_config = InstantDistanceBuildConfig::default();
    let query_configs = default_sweep();

    let hardware = HardwareInfo {
        cpu: detect_cpu(),
        cores_used: 1,
        ram_gb: detect_ram_gb(),
        os: detect_os(),
        storage: "NVMe SSD".to_string(),
    };

    let dataset_info = DatasetInfo {
        name: args.dataset.clone(),
        source: if args.dataset.starts_with("sift") { "fvecs" } else { "synthetic" }.to_string(),
        n_vectors: n_base,
        n_queries,
        dimension: dim,
        metric: args.metric.clone(),
    };

    std::fs::create_dir_all(&args.output_dir)?;

    let config = BenchmarkRunConfig {
        crate_version: "0.6.1",
        hardware,
        dataset: dataset_info,
        base_vectors: &base,
        n_base,
        queries: &queries,
        n_queries,
        dim,
        metric,
        build_config,
        query_configs,
        k: args.k,
        n_runs: args.runs,
        ground_truth: &gt,
        output_dir: &args.output_dir,
    };

    let result = runner::run_benchmark::<ann_bench_instant_distance::InstantDistanceIndex>(config)?;

    eprintln!(
        "\n  Done. Best recall@10: {:.4}, Best QPS: {:.0}",
        result.query_sweeps.iter().map(|s| s.recall_at_10).fold(0.0_f64, f64::max),
        result.query_sweeps.iter().map(|s| s.qps).fold(0.0_f64, f64::max),
    );

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

#[allow(clippy::type_complexity)]
fn load_dataset(args: &RunArgs) -> anyhow::Result<(usize, Vec<f32>, usize, Vec<f32>, usize)> {
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

fn detect_cpu() -> String {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        "unknown".to_string()
    }
}

fn detect_ram_gb() -> u32 {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.trim().parse::<u64>().ok())
            .map(|b| (b / (1024 * 1024 * 1024)) as u32)
            .unwrap_or(0)
    }
    #[cfg(not(target_os = "macos"))]
    {
        0
    }
}

fn detect_os() -> String {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("sw_vers")
            .args(["-productVersion"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|v| format!("macOS {}", v.trim()))
            .unwrap_or_else(|| "macOS".to_string())
    }
    #[cfg(target_os = "linux")]
    {
        "Linux".to_string()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "unknown".to_string()
    }
}
