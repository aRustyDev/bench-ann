use std::time::Instant;

use ann_bench_core::{AnnIndex, QueryResult};

/// Measurement result for a single query pass.
pub struct QueryPassResult {
    /// Total time for the pass in seconds.
    pub total_time_s: f64,
    /// Per-query results (index, distance).
    pub results: Vec<Vec<QueryResult>>,
    /// Per-query durations in microseconds (only populated for the best run).
    pub per_query_us: Vec<u64>,
}

/// Measure QPS and collect results for one query parameter setting.
///
/// Protocol:
/// 1. Set query parameters via prepare_query()
/// 2. 1 warmup run (discarded)
/// 3. `n_runs` measurement runs
/// 4. Take min total time (best run)
/// 5. QPS = n_queries / min_total_time
///
/// Returns (all run times, best run results with per-query latencies).
pub fn measure_queries<I: AnnIndex>(
    index: &mut I,
    queries: &[f32],
    n_queries: usize,
    dim: usize,
    k: usize,
    config: &I::Query,
    n_runs: usize,
) -> anyhow::Result<(Vec<f64>, QueryPassResult)> {
    index.prepare_query(config);

    // Warmup run (discarded)
    for qi in 0..n_queries {
        let q = &queries[qi * dim..(qi + 1) * dim];
        let _ = index.query(q, k, config)?;
    }

    let mut run_times = Vec::with_capacity(n_runs);
    let mut best_time = f64::MAX;
    let mut best_results = Vec::new();
    let mut best_per_query_us = Vec::new();

    for _ in 0..n_runs {
        let mut results = Vec::with_capacity(n_queries);
        let mut per_query_us = Vec::with_capacity(n_queries);

        let pass_start = Instant::now();
        for qi in 0..n_queries {
            let q = &queries[qi * dim..(qi + 1) * dim];
            let q_start = Instant::now();
            let r = index.query(q, k, config)?;
            per_query_us.push(q_start.elapsed().as_micros() as u64);
            results.push(r);
        }
        let elapsed = pass_start.elapsed().as_secs_f64();
        run_times.push(elapsed);

        if elapsed < best_time {
            best_time = elapsed;
            best_results = results;
            best_per_query_us = per_query_us;
        }
    }

    Ok((
        run_times,
        QueryPassResult {
            total_time_s: best_time,
            results: best_results,
            per_query_us: best_per_query_us,
        },
    ))
}

/// Compute p50 and p99 latencies from per-query durations (microseconds).
pub fn compute_latency_percentiles(per_query_us: &[u64]) -> (u64, u64) {
    if per_query_us.is_empty() {
        return (0, 0);
    }

    let mut sorted = per_query_us.to_vec();
    sorted.sort_unstable();

    let p50_idx = (sorted.len() as f64 * 0.50).ceil() as usize - 1;
    let p99_idx = (sorted.len() as f64 * 0.99).ceil() as usize - 1;

    (sorted[p50_idx], sorted[p99_idx.min(sorted.len() - 1)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latency_percentiles() {
        // 100 values: 1..=100
        let data: Vec<u64> = (1..=100).collect();
        let (p50, p99) = compute_latency_percentiles(&data);
        assert_eq!(p50, 50);
        assert_eq!(p99, 99);
    }

    #[test]
    fn test_latency_single() {
        let data = vec![42];
        let (p50, p99) = compute_latency_percentiles(&data);
        assert_eq!(p50, 42);
        assert_eq!(p99, 42);
    }
}
