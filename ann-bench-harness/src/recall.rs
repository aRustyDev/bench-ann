use ann_bench_core::QueryResult;

/// Epsilon for distance-threshold recall computation.
const EPSILON: f32 = 1e-3;

/// Compute recall@k using distance-threshold method (ann-benchmarks pattern).
///
/// For each query, the threshold is the ground truth k-th neighbor's distance + epsilon.
/// A result is a match if its distance <= threshold.
///
/// `ann_results` — ANN results per query: Vec of (index, distance).
/// `ground_truth` — exact kNN per query: Vec of (index, distance), sorted by distance.
/// `k` — the k for recall@k.
///
/// Returns the mean recall across all queries.
pub fn compute_recall(
    ann_results: &[Vec<QueryResult>],
    ground_truth: &[Vec<(usize, f32)>],
    k: usize,
) -> f64 {
    assert_eq!(ann_results.len(), ground_truth.len());

    if ann_results.is_empty() {
        return 0.0;
    }

    let total: f64 = ann_results
        .iter()
        .zip(ground_truth.iter())
        .map(|(ann, gt)| {
            let gt_k = k.min(gt.len());
            if gt_k == 0 {
                return 0.0;
            }
            let threshold = gt[gt_k - 1].1 + EPSILON;
            let matches = ann.iter().take(k).filter(|&&(_, dist)| dist <= threshold).count();
            matches as f64 / gt_k as f64
        })
        .sum();

    total / ann_results.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_recall() {
        let gt = vec![vec![(0, 0.0f32), (1, 1.0), (2, 2.0)]];
        let ann = vec![vec![(0, 0.0f32), (1, 1.0), (2, 2.0)]];
        let r = compute_recall(&ann, &gt, 3);
        assert!((r - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_zero_recall() {
        let gt = vec![vec![(0, 0.0f32), (1, 1.0)]];
        let ann = vec![vec![(2, 10.0f32), (3, 20.0)]];
        let r = compute_recall(&ann, &gt, 2);
        assert!(r.abs() < 1e-6);
    }

    #[test]
    fn test_partial_recall() {
        let gt = vec![vec![(0, 0.0f32), (1, 1.0)]];
        // ANN found index 0 (correct) and index 5 (wrong, far away)
        let ann = vec![vec![(0, 0.0f32), (5, 100.0)]];
        let r = compute_recall(&ann, &gt, 2);
        assert!((r - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_recall_at_1() {
        let gt = vec![vec![(0, 0.0f32), (1, 1.0), (2, 2.0)]];
        let ann = vec![vec![(0, 0.0f32)]];
        let r = compute_recall(&ann, &gt, 1);
        assert!((r - 1.0).abs() < 1e-6);
    }
}
