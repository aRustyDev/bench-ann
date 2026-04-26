use ann_bench_core::DistanceMetric;

/// Compute the squared Euclidean distance between two vectors.
#[inline]
pub fn euclidean_sq(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let d = x - y;
            d * d
        })
        .sum()
}

/// Compute dot product (inner product) between two vectors.
#[inline]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Compute cosine distance = 1 - cosine_similarity.
/// Vectors need not be pre-normalized.
#[inline]
pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = dot_product(a, b);
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    let denom = norm_a * norm_b;
    if denom == 0.0 {
        1.0
    } else {
        1.0 - (dot / denom)
    }
}

/// Compute distance between two vectors using the specified metric.
#[inline]
pub fn compute_distance(a: &[f32], b: &[f32], metric: DistanceMetric) -> f32 {
    match metric {
        DistanceMetric::Euclidean => euclidean_sq(a, b),
        DistanceMetric::DotProduct => -dot_product(a, b), // negate for "smaller is closer"
        DistanceMetric::Cosine => cosine_distance(a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_sq() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        assert!((euclidean_sq(&a, &b) - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_same() {
        let a = [3.0, 4.0];
        assert!((euclidean_sq(&a, &a)).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert!((dot_product(&a, &b) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_identical() {
        let a = [1.0, 2.0, 3.0];
        assert!(cosine_distance(&a, &a).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_orthogonal() {
        let a = [1.0, 0.0];
        let b = [0.0, 1.0];
        assert!((cosine_distance(&a, &b) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_zero_vector() {
        let a = [0.0, 0.0];
        let b = [1.0, 1.0];
        assert!((cosine_distance(&a, &b) - 1.0).abs() < 1e-6);
    }
}
