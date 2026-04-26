use crate::results::ParetoPoint;

/// Compute the Pareto frontier from (recall@10, QPS) points.
///
/// Algorithm:
/// 1. Sort by recall descending; break ties by QPS descending
/// 2. Walk the sorted list, keeping points with QPS > max_qps_so_far
/// 3. Result: as recall decreases, QPS monotonically non-decreases
pub fn compute_pareto_frontier(points: &[(f64, f64)]) -> Vec<ParetoPoint> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut sorted: Vec<(f64, f64)> = points.to_vec();
    sorted.sort_by(|a, b| {
        b.0.partial_cmp(&a.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut frontier = Vec::new();
    let mut max_qps = 0.0_f64;

    for &(recall, qps) in &sorted {
        if qps > max_qps {
            frontier.push(ParetoPoint {
                recall_at_10: recall,
                qps,
            });
            max_qps = qps;
        }
    }

    frontier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let frontier = compute_pareto_frontier(&[]);
        assert!(frontier.is_empty());
    }

    #[test]
    fn test_single_point() {
        let frontier = compute_pareto_frontier(&[(0.95, 5000.0)]);
        assert_eq!(frontier.len(), 1);
        assert_eq!(frontier[0].recall_at_10, 0.95);
        assert_eq!(frontier[0].qps, 5000.0);
    }

    #[test]
    fn test_pareto_frontier() {
        // Points: (recall, QPS)
        // Dominated point (0.90, 3000) should be excluded since (0.96, 5100) has both better
        let points = vec![
            (0.99, 1200.0),
            (0.96, 5100.0),
            (0.85, 15230.0),
            (0.90, 3000.0), // dominated by (0.96, 5100)
        ];
        let frontier = compute_pareto_frontier(&points);
        assert_eq!(frontier.len(), 3);
        // Sorted by recall descending
        assert_eq!(frontier[0].recall_at_10, 0.99);
        assert_eq!(frontier[1].recall_at_10, 0.96);
        assert_eq!(frontier[2].recall_at_10, 0.85);
        // QPS monotonically non-decreasing
        assert!(frontier[0].qps <= frontier[1].qps);
        assert!(frontier[1].qps <= frontier[2].qps);
    }

    #[test]
    fn test_all_dominated() {
        // Only the best-QPS point survives
        let points = vec![
            (0.99, 100.0),
            (0.95, 200.0),
            (0.90, 300.0),
        ];
        let frontier = compute_pareto_frontier(&points);
        // Each has higher QPS than previous, so all survive
        assert_eq!(frontier.len(), 3);
    }
}
