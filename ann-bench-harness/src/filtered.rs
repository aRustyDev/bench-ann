use std::time::Instant;

use ann_bench_core::{AnnIndex, QueryResult};

use crate::recall::compute_recall;

/// Result of a filtered benchmark pass.
pub struct FilteredPassResult {
    pub cardinality: usize,
    pub selectivity: f64,
    pub recall_at_10: f64,
    pub qps: f64,
}

/// Run filtered ANN benchmark for a single cardinality.
///
/// Filter predicate: `|id| id % cardinality == 0` (selects ~1/cardinality of dataset).
/// Only called for adapters where `supports_filtered_search() == true`.
#[allow(clippy::too_many_arguments)]
pub fn run_filtered_benchmark<I: AnnIndex>(
    index: &I,
    queries: &[f32],
    n_queries: usize,
    dim: usize,
    k: usize,
    config: &I::Query,
    cardinality: usize,
    ground_truth: &[Vec<(usize, f32)>],
) -> anyhow::Result<FilteredPassResult> {
    let filter = |id: usize| -> bool { id.is_multiple_of(cardinality) };
    let selectivity = 1.0 / cardinality as f64;

    let mut results: Vec<Vec<QueryResult>> = Vec::with_capacity(n_queries);

    let start = Instant::now();
    for qi in 0..n_queries {
        let q = &queries[qi * dim..(qi + 1) * dim];
        let r = index.filtered_query(q, k, config, &filter)?;
        results.push(r);
    }
    let elapsed = start.elapsed().as_secs_f64();
    let qps = n_queries as f64 / elapsed;

    // Compute filtered ground truth: refilter the full GT to only include filtered indices
    let filtered_gt: Vec<Vec<(usize, f32)>> = ground_truth
        .iter()
        .map(|gt_row| {
            let mut filtered: Vec<(usize, f32)> = gt_row
                .iter()
                .filter(|&&(idx, _)| filter(idx))
                .copied()
                .collect();
            filtered.truncate(k);
            filtered
        })
        .collect();

    let recall = compute_recall(&results, &filtered_gt, k);

    Ok(FilteredPassResult {
        cardinality,
        selectivity,
        recall_at_10: recall,
        qps,
    })
}
