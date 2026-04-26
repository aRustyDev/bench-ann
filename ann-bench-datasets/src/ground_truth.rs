use std::fs;
use std::path::Path;

use anyhow::Context;
use rayon::prelude::*;

use ann_bench_core::DistanceMetric;

use crate::distance::compute_distance;

/// Compute exact k nearest neighbors via brute force.
///
/// Parallelized with rayon (one query per thread). Deterministic output.
/// Returns: for each query, a Vec of (neighbor_index, distance) sorted by distance ascending.
pub fn compute_ground_truth(
    base: &[f32],
    n_base: usize,
    queries: &[f32],
    n_queries: usize,
    dim: usize,
    k: usize,
    metric: DistanceMetric,
) -> Vec<Vec<(usize, f32)>> {
    assert_eq!(base.len(), n_base * dim);
    assert_eq!(queries.len(), n_queries * dim);

    (0..n_queries)
        .into_par_iter()
        .map(|qi| {
            let query = &queries[qi * dim..(qi + 1) * dim];

            // Compute all distances
            let mut dists: Vec<(usize, f32)> = (0..n_base)
                .map(|bi| {
                    let base_vec = &base[bi * dim..(bi + 1) * dim];
                    (bi, compute_distance(query, base_vec, metric))
                })
                .collect();

            // Partial sort to get top-k
            let nth = k.min(dists.len()) - 1;
            dists.select_nth_unstable_by(nth, |a, b| {
                a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
            });
            dists.truncate(k);
            dists.sort_by(|a, b| {
                a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
            });

            dists
        })
        .collect()
}

/// Save ground truth to a directory as two binary files:
/// - `gt_indices.bin`: n_queries rows of k i32 indices
/// - `gt_distances.bin`: n_queries rows of k f32 distances
pub fn save_ground_truth(
    gt: &[Vec<(usize, f32)>],
    dir: &Path,
) -> anyhow::Result<()> {
    fs::create_dir_all(dir)?;

    let k = gt.first().map(|v| v.len()).unwrap_or(0);

    // Save indices
    let mut idx_data = Vec::with_capacity(gt.len() * k * 4);
    let mut dist_data = Vec::with_capacity(gt.len() * k * 4);

    for row in gt {
        for &(idx, dist) in row {
            idx_data.extend_from_slice(&(idx as i32).to_le_bytes());
            dist_data.extend_from_slice(&dist.to_le_bytes());
        }
    }

    fs::write(dir.join("gt_indices.bin"), &idx_data)
        .context("writing gt_indices.bin")?;
    fs::write(dir.join("gt_distances.bin"), &dist_data)
        .context("writing gt_distances.bin")?;

    // Save metadata
    let meta = format!("n_queries={}\nk={}\n", gt.len(), k);
    fs::write(dir.join("gt_meta.txt"), meta)?;

    Ok(())
}

/// Load cached ground truth from a directory.
pub fn load_ground_truth(dir: &Path) -> anyhow::Result<Vec<Vec<(usize, f32)>>> {
    let meta = fs::read_to_string(dir.join("gt_meta.txt"))
        .context("reading gt_meta.txt")?;

    let mut n_queries = 0usize;
    let mut k = 0usize;
    for line in meta.lines() {
        if let Some(val) = line.strip_prefix("n_queries=") {
            n_queries = val.parse()?;
        } else if let Some(val) = line.strip_prefix("k=") {
            k = val.parse()?;
        }
    }

    let idx_bytes = fs::read(dir.join("gt_indices.bin"))
        .context("reading gt_indices.bin")?;
    let dist_bytes = fs::read(dir.join("gt_distances.bin"))
        .context("reading gt_distances.bin")?;

    assert_eq!(idx_bytes.len(), n_queries * k * 4);
    assert_eq!(dist_bytes.len(), n_queries * k * 4);

    let mut result = Vec::with_capacity(n_queries);
    for qi in 0..n_queries {
        let mut row = Vec::with_capacity(k);
        for ki in 0..k {
            let offset = (qi * k + ki) * 4;
            let idx = i32::from_le_bytes(idx_bytes[offset..offset + 4].try_into().unwrap()) as usize;
            let dist = f32::from_le_bytes(dist_bytes[offset..offset + 4].try_into().unwrap());
            row.push((idx, dist));
        }
        result.push(row);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_truth_small() {
        // 4 base vectors in 2D, 2 queries
        let base = vec![
            0.0, 0.0, // vec 0
            1.0, 0.0, // vec 1
            0.0, 1.0, // vec 2
            1.0, 1.0, // vec 3
        ];
        let queries = vec![
            0.0, 0.0, // query 0: closest to vec 0
            1.0, 1.0, // query 1: closest to vec 3
        ];

        let gt = compute_ground_truth(&base, 4, &queries, 2, 2, 2, DistanceMetric::Euclidean);

        assert_eq!(gt.len(), 2);
        // Query 0: (0,0) -> closest are vec0=(0,0) dist=0, then vec1=(1,0) or vec2=(0,1) dist=1
        assert_eq!(gt[0][0].0, 0);
        assert!(gt[0][0].1.abs() < 1e-6);
        // Query 1: (1,1) -> closest is vec3=(1,1) dist=0
        assert_eq!(gt[1][0].0, 3);
        assert!(gt[1][0].1.abs() < 1e-6);
    }

    #[test]
    fn test_ground_truth_save_load() {
        let gt = vec![
            vec![(0usize, 0.0f32), (1, 1.0)],
            vec![(3, 0.0), (2, 1.0)],
        ];
        let tmp = tempfile::tempdir().unwrap();
        save_ground_truth(&gt, tmp.path()).unwrap();
        let loaded = load_ground_truth(tmp.path()).unwrap();
        assert_eq!(loaded.len(), gt.len());
        for (a, b) in gt.iter().zip(loaded.iter()) {
            for ((ai, ad), (bi, bd)) in a.iter().zip(b.iter()) {
                assert_eq!(ai, bi);
                assert!((ad - bd).abs() < 1e-6);
            }
        }
    }
}
