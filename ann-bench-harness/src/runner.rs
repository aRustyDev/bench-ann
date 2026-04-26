use std::path::Path;
use std::time::Instant;

use ann_bench_core::*;

use crate::filtered;
use crate::measurement::{self, compute_latency_percentiles};
use crate::memory;
use crate::output;
use crate::recall;

/// Configuration for a benchmark run.
pub struct BenchmarkRunConfig<'a, I: AnnIndex> {
    pub crate_version: &'a str,
    pub hardware: HardwareInfo,
    pub dataset: DatasetInfo,
    pub base_vectors: &'a [f32],
    pub n_base: usize,
    pub queries: &'a [f32],
    pub n_queries: usize,
    pub dim: usize,
    pub metric: DistanceMetric,
    pub build_config: I::Build,
    pub query_configs: Vec<I::Query>,
    pub k: usize,
    pub n_runs: usize,
    pub ground_truth: &'a [Vec<(usize, f32)>],
    pub output_dir: &'a Path,
}

/// Run a full benchmark for one adapter + dataset + build config.
///
/// Pipeline: build → memory → sweep all query configs → filtered → incremental → pareto → tier → output.
pub fn run_benchmark<I: AnnIndex>(
    config: BenchmarkRunConfig<'_, I>,
) -> anyhow::Result<BenchmarkResult> {
    let BenchmarkRunConfig {
        crate_version,
        hardware,
        dataset,
        base_vectors,
        n_base,
        queries,
        n_queries,
        dim,
        metric,
        build_config,
        query_configs,
        k,
        n_runs,
        ground_truth,
        output_dir,
    } = config;

    // === Build phase ===
    eprintln!("  Building index...");
    let rss_before = memory::snapshot_rss().unwrap_or(0);
    let build_start = Instant::now();
    let mut index = I::build(base_vectors, n_base, dim, metric, &build_config)?;
    let build_time_s = build_start.elapsed().as_secs_f64();
    let rss_after = memory::snapshot_rss().unwrap_or(0);

    let raw_data_bytes = (n_base * dim * 4) as u64;
    let memory_per_vector = if rss_after > rss_before {
        ((rss_after - rss_before).saturating_sub(raw_data_bytes)) as i64 / n_base as i64
    } else {
        0
    };

    eprintln!(
        "  Built in {:.1}s, RSS delta: {}MB, mem/vec: {}B",
        build_time_s,
        (rss_after.saturating_sub(rss_before)) / (1024 * 1024),
        memory_per_vector
    );

    // === Index size ===
    let tmp_index_path = output_dir.join("_tmp_index");
    let disk_bytes = index.save(&tmp_index_path)?;
    let disk_per_vector = disk_bytes as f64 / n_base as f64;
    let _ = std::fs::remove_file(&tmp_index_path);
    let _ = std::fs::remove_dir_all(&tmp_index_path);

    // === Query sweep ===
    let mut query_sweeps = Vec::with_capacity(query_configs.len());

    for qc in &query_configs {
        eprintln!("  Sweeping query config: {:?}", qc.name());

        let (run_times, best) = measurement::measure_queries(
            &mut index, queries, n_queries, dim, k, qc, n_runs,
        )?;

        let qps = n_queries as f64 / best.total_time_s;
        let (p50, p99) = compute_latency_percentiles(&best.per_query_us);

        let recall_1 = recall::compute_recall(&best.results, ground_truth, 1);
        let recall_10 = recall::compute_recall(&best.results, ground_truth, 10);
        let recall_100 = recall::compute_recall(&best.results, ground_truth, 100);

        eprintln!(
            "    recall@10={:.4}, QPS={:.0}, p50={}\u{b5}s, p99={}\u{b5}s",
            recall_10, qps, p50, p99
        );

        query_sweeps.push(QuerySweepResult {
            config: serde_json::to_value(qc)?,
            recall_at_1: recall_1,
            recall_at_10: recall_10,
            recall_at_100: recall_100,
            qps,
            latency_p50_us: p50,
            latency_p99_us: p99,
            run_times_s: run_times,
            best_run_s: best.total_time_s,
        });
    }

    // === Filtered benchmark ===
    let filtered_results = if index.supports_filtered_search() && !query_configs.is_empty() {
        eprintln!("  Running filtered benchmarks...");
        let qc = &query_configs[query_configs.len() / 2]; // mid-range config
        let mut results = Vec::new();
        for &card in &[10usize, 100, 1000] {
            match filtered::run_filtered_benchmark(
                &index, queries, n_queries, dim, k, qc, card, ground_truth,
            ) {
                Ok(fr) => {
                    // Find the unfiltered baseline at same config
                    let unfiltered_sweep = &query_sweeps[query_configs.len() / 2];
                    results.push(FilteredResult {
                        cardinality: fr.cardinality,
                        selectivity: fr.selectivity,
                        recall_at_10: fr.recall_at_10,
                        qps: fr.qps,
                        unfiltered_recall_at_10: unfiltered_sweep.recall_at_10,
                        unfiltered_qps: unfiltered_sweep.qps,
                    });
                }
                Err(e) => eprintln!("    Filtered (C={card}) failed: {e}"),
            }
        }
        if results.is_empty() { None } else { Some(results) }
    } else {
        None
    };

    // === Incremental benchmark ===
    // Skipped here — run separately if adapter supports it and dataset is large enough
    let incremental_result = None;

    // === Pareto frontier ===
    let pareto_points: Vec<(f64, f64)> = query_sweeps
        .iter()
        .map(|s| (s.recall_at_10, s.qps))
        .collect();
    let pareto_frontier = pareto::compute_pareto_frontier(&pareto_points);

    // === Tier classification ===
    let tiers = tiers::classify_tiers(
        &query_sweeps,
        build_time_s,
        memory_per_vector,
        disk_per_vector,
    );

    let crate_name = index.crate_name().to_string();

    let result = BenchmarkResult {
        schema_version: "1.0".to_string(),
        harness_version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono_now(),
        hardware,
        crate_name: crate_name.clone(),
        crate_version: crate_version.to_string(),
        dataset,
        build: BuildResult {
            config: serde_json::to_value(&build_config)?,
            time_s: build_time_s,
            memory_per_vector_bytes: memory_per_vector,
            rss_before_bytes: rss_before,
            rss_after_bytes: rss_after,
        },
        index_size: IndexSizeResult {
            disk_bytes,
            disk_per_vector_bytes: disk_per_vector,
        },
        query_sweeps,
        filtered: filtered_results,
        incremental: incremental_result,
        pareto_frontier,
        tier_classification: tiers,
    };

    // Write JSON output
    let filename = format!("{}_{}_{}.json",
        crate_name,
        result.dataset.name,
        build_config.name()
    );
    output::write_results_json(&result, &output_dir.join(&filename))?;
    eprintln!("  Results written to {}", filename);

    Ok(result)
}

fn chrono_now() -> String {
    // Simple UTC timestamp without pulling in chrono crate
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Basic ISO-8601 format
    let secs_per_day = 86400u64;
    let days = now / secs_per_day;
    let time_of_day = now % secs_per_day;
    let hours = time_of_day / 3600;
    let mins = (time_of_day % 3600) / 60;
    let secs = time_of_day % 60;

    // Approximate date from days since epoch (good enough for timestamps)
    let mut y = 1970i64;
    let mut remaining = days as i64;
    loop {
        let days_in_year = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let days_in_months = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 1;
    for &dim in &days_in_months {
        if remaining < dim as i64 {
            break;
        }
        remaining -= dim as i64;
        m += 1;
    }
    let d = remaining + 1;

    format!("{y:04}-{m:02}-{d:02}T{hours:02}:{mins:02}:{secs:02}Z")
}
