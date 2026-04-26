use ann_bench_core::{AnnIndex, DistanceMetric};

use crate::measurement;
use crate::recall::compute_recall;

/// Result of an incremental update benchmark.
pub struct IncrementalPassResult {
    pub inserted: usize,
    pub deleted: usize,
    pub recall_at_10_after_update: f64,
    pub qps_after_update: f64,
    pub recall_at_10_fresh_build: f64,
    pub qps_fresh_build: f64,
}

/// Run incremental update benchmark.
///
/// Protocol:
/// 1. Build index on first 900K vectors
/// 2. Insert next 100K via insert()
/// 3. Delete first 100K via delete()
/// 4. Measure recall@10 and QPS on the post-update index
/// 5. Compare against a fresh build on vectors 100K..1M
#[allow(clippy::too_many_arguments)]
pub fn run_incremental_benchmark<I: AnnIndex>(
    base_vectors: &[f32],
    n_base: usize,
    dim: usize,
    metric: DistanceMetric,
    build_config: &I::Build,
    query_config: &I::Query,
    queries: &[f32],
    n_queries: usize,
    k: usize,
    ground_truth: &[Vec<(usize, f32)>],
    n_runs: usize,
) -> anyhow::Result<IncrementalPassResult> {
    let n_initial = (n_base * 9) / 10; // 900K
    let n_insert = n_base - n_initial;  // 100K
    let n_delete = n_insert;            // 100K

    // Step 1: Build on first 900K
    let initial_vecs = &base_vectors[..n_initial * dim];
    let mut index = I::build(initial_vecs, n_initial, dim, metric, build_config)?;

    // Step 2: Insert next 100K
    let insert_vecs = &base_vectors[n_initial * dim..];
    index.insert(insert_vecs, n_insert, dim, n_initial)?;

    // Step 3: Delete first 100K
    let delete_ids: Vec<usize> = (0..n_delete).collect();
    index.delete(&delete_ids)?;

    // Step 4: Measure on updated index
    let (_run_times, best) = measurement::measure_queries(
        &mut index,
        queries,
        n_queries,
        dim,
        k,
        query_config,
        n_runs,
    )?;
    let qps_after = n_queries as f64 / best.total_time_s;
    let recall_after = compute_recall(&best.results, ground_truth, k);

    // Step 5: Fresh build on vectors[100K..1M] for comparison
    let fresh_vecs = &base_vectors[n_delete * dim..];
    let n_fresh = n_base - n_delete;
    let mut fresh_index = I::build(fresh_vecs, n_fresh, dim, metric, build_config)?;
    let (_, fresh_best) = measurement::measure_queries(
        &mut fresh_index,
        queries,
        n_queries,
        dim,
        k,
        query_config,
        n_runs,
    )?;
    let qps_fresh = n_queries as f64 / fresh_best.total_time_s;
    let recall_fresh = compute_recall(&fresh_best.results, ground_truth, k);

    Ok(IncrementalPassResult {
        inserted: n_insert,
        deleted: n_delete,
        recall_at_10_after_update: recall_after,
        qps_after_update: qps_after,
        recall_at_10_fresh_build: recall_fresh,
        qps_fresh_build: qps_fresh,
    })
}
