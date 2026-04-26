use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};

/// Distribution for synthetic vector generation.
#[derive(Debug, Clone, Copy)]
pub enum SyntheticDistribution {
    /// Uniform on unit sphere (for cosine/inner product benchmarks).
    UnitSphere,
    /// Multivariate Gaussian, identity covariance (for L2 benchmarks).
    Gaussian,
}

/// Generated synthetic dataset with contiguous flat buffers.
pub struct SyntheticDataset {
    /// Flat buffer of n_vectors * dimension f32 values.
    pub vectors: Vec<f32>,
    /// Flat buffer of n_queries * dimension f32 values.
    pub queries: Vec<f32>,
    pub n_vectors: usize,
    pub n_queries: usize,
    pub dimension: usize,
    pub seed: u64,
}

/// Generate synthetic vectors for benchmarking.
///
/// Fixed seeds ensure reproducibility across runs.
pub fn generate_synthetic(
    n_vectors: usize,
    n_queries: usize,
    dimension: usize,
    distribution: SyntheticDistribution,
    seed: u64,
) -> SyntheticDataset {
    let total = n_vectors + n_queries;
    let mut flat = Vec::with_capacity(total * dimension);
    let mut rng = StdRng::seed_from_u64(seed);

    match distribution {
        SyntheticDistribution::UnitSphere => {
            // UnitSphere from rand_distr generates points uniformly on the
            // unit sphere in N dimensions. It requires const generic or runtime dim.
            // We generate each vector by sampling N(0,1) for each component then normalizing.
            let normal = Normal::new(0.0f32, 1.0f32).unwrap();
            for _ in 0..total {
                let mut vec: Vec<f32> = (0..dimension).map(|_| normal.sample(&mut rng)).collect();
                let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm > 0.0 {
                    for v in &mut vec {
                        *v /= norm;
                    }
                }
                flat.extend_from_slice(&vec);
            }
        }
        SyntheticDistribution::Gaussian => {
            let normal = Normal::new(0.0f32, 1.0f32).unwrap();
            for _ in 0..total {
                for _ in 0..dimension {
                    flat.push(normal.sample(&mut rng));
                }
            }
        }
    }

    let split = n_vectors * dimension;
    let queries = flat.split_off(split);

    SyntheticDataset {
        vectors: flat,
        queries,
        n_vectors,
        n_queries,
        dimension,
        seed,
    }
}

/// Well-known seeds for reproducible synthetic datasets.
pub mod seeds {
    pub const SPHERE_128: u64 = 42_000_128;
    pub const SPHERE_384: u64 = 42_000_384;
    pub const SPHERE_768: u64 = 42_000_768;
    pub const SPHERE_1536: u64 = 42_001_536;
    pub const GAUSSIAN_128: u64 = 43_000_128;
    pub const GAUSSIAN_384: u64 = 43_000_384;
    pub const GAUSSIAN_768: u64 = 43_000_768;
    pub const GAUSSIAN_1536: u64 = 43_001_536;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthetic_dimensions() {
        let ds = generate_synthetic(100, 10, 64, SyntheticDistribution::Gaussian, 42);
        assert_eq!(ds.vectors.len(), 100 * 64);
        assert_eq!(ds.queries.len(), 10 * 64);
        assert_eq!(ds.dimension, 64);
    }

    #[test]
    fn test_unit_sphere_normalized() {
        let ds = generate_synthetic(10, 0, 32, SyntheticDistribution::UnitSphere, 42);
        for vec in ds.vectors.chunks_exact(32) {
            let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
            assert!((norm - 1.0).abs() < 1e-5, "norm was {}", norm);
        }
    }

    #[test]
    fn test_reproducibility() {
        let ds1 = generate_synthetic(50, 5, 16, SyntheticDistribution::Gaussian, 12345);
        let ds2 = generate_synthetic(50, 5, 16, SyntheticDistribution::Gaussian, 12345);
        assert_eq!(ds1.vectors, ds2.vectors);
        assert_eq!(ds1.queries, ds2.queries);
    }
}
