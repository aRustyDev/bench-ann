use std::path::Path;

use ann_bench_core::{AnnIndex, BuildConfig, DistanceMetric, QueryConfig, QueryResult};
use hnsw_rs::prelude::*;
use serde::{Deserialize, Serialize};

/// Build-time configuration for hnsw_rs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswBuildConfig {
    /// Number of bidirectional links per node (M parameter).
    #[serde(rename = "M")]
    pub m: usize,
    /// Size of the dynamic candidate list during construction.
    pub ef_construction: usize,
    /// Maximum number of elements the index can hold.
    pub max_elements: usize,
}

impl BuildConfig for HnswBuildConfig {
    fn name(&self) -> &str {
        // Leak a formatted string for the static-ish lifetime.
        // Fine for a benchmark tool.
        Box::leak(format!("M={},ef_c={}", self.m, self.ef_construction).into_boxed_str())
    }
}

impl Default for HnswBuildConfig {
    fn default() -> Self {
        Self {
            m: 16,
            ef_construction: 200,
            max_elements: 1_100_000,
        }
    }
}

/// Query-time configuration for hnsw_rs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswQueryConfig {
    pub ef_search: usize,
}

impl QueryConfig for HnswQueryConfig {
    fn name(&self) -> &str {
        Box::leak(format!("ef_search={}", self.ef_search).into_boxed_str())
    }
}

/// Default parameter sweep for hnsw_rs benchmarks.
pub fn default_sweep() -> Vec<HnswQueryConfig> {
    [10, 20, 50, 100, 200, 500]
        .iter()
        .map(|&ef| HnswQueryConfig { ef_search: ef })
        .collect()
}

/// Wrapper around hnsw_rs that implements AnnIndex.
///
/// We use an enum to handle the different distance types at runtime,
/// since hnsw_rs is generic over the distance function.
pub enum HnswIndex {
    L2(HnswInner<DistL2>),
    Cosine(HnswInner<DistCosine>),
    Dot(HnswInner<DistDot>),
}

pub struct HnswInner<D: Distance<f32> + Default + Send + Sync> {
    hnsw: Hnsw<'static, f32, D>,
    // We need to keep the data alive since hnsw_rs borrows it
    _data: Vec<Vec<f32>>,
    _dim: usize,
    _n: usize,
}

impl AnnIndex for HnswIndex {
    type Build = HnswBuildConfig;
    type Query = HnswQueryConfig;

    fn build(
        vectors: &[f32],
        n: usize,
        dim: usize,
        metric: DistanceMetric,
        config: &Self::Build,
    ) -> anyhow::Result<Self> {
        match metric {
            DistanceMetric::Euclidean => {
                let inner = build_inner::<DistL2>(vectors, n, dim, config)?;
                Ok(HnswIndex::L2(inner))
            }
            DistanceMetric::Cosine => {
                let inner = build_inner::<DistCosine>(vectors, n, dim, config)?;
                Ok(HnswIndex::Cosine(inner))
            }
            DistanceMetric::DotProduct => {
                let inner = build_inner::<DistDot>(vectors, n, dim, config)?;
                Ok(HnswIndex::Dot(inner))
            }
        }
    }

    fn query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
    ) -> anyhow::Result<Vec<QueryResult>> {
        match self {
            HnswIndex::L2(inner) => query_inner(&inner.hnsw, vector, k, config),
            HnswIndex::Cosine(inner) => query_inner(&inner.hnsw, vector, k, config),
            HnswIndex::Dot(inner) => query_inner(&inner.hnsw, vector, k, config),
        }
    }

    fn filtered_query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
        filter: &dyn Fn(usize) -> bool,
    ) -> anyhow::Result<Vec<QueryResult>> {
        match self {
            HnswIndex::L2(inner) => filtered_query_inner(&inner.hnsw, vector, k, config, filter),
            HnswIndex::Cosine(inner) => {
                filtered_query_inner(&inner.hnsw, vector, k, config, filter)
            }
            HnswIndex::Dot(inner) => filtered_query_inner(&inner.hnsw, vector, k, config, filter),
        }
    }

    fn save(&self, path: &Path) -> anyhow::Result<u64> {
        let dir = path.parent().unwrap_or(Path::new("."));
        let basename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("hnsw_index");

        match self {
            HnswIndex::L2(inner) => inner.hnsw.file_dump(dir, basename)?,
            HnswIndex::Cosine(inner) => inner.hnsw.file_dump(dir, basename)?,
            HnswIndex::Dot(inner) => inner.hnsw.file_dump(dir, basename)?,
        };

        // Calculate total size of dumped files
        let graph_file = dir.join(format!("{basename}.hnsw.graph"));
        let data_file = dir.join(format!("{basename}.hnsw.data"));
        let mut total = 0u64;
        if graph_file.exists() {
            total += std::fs::metadata(&graph_file)?.len();
        }
        if data_file.exists() {
            total += std::fs::metadata(&data_file)?.len();
        }
        Ok(total)
    }

    fn load(path: &Path, metric: DistanceMetric) -> anyhow::Result<Self> {
        let dir = path.parent().unwrap_or(Path::new("."));
        let basename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("hnsw_index");

        // HnswIo must outlive the Hnsw it loads ('a: 'b constraint).
        // We leak the reloader since load() is not on the benchmark hot path —
        // it's only used for save/load round-trip validation, not measurement.
        match metric {
            DistanceMetric::Euclidean => {
                let reloader = Box::leak(Box::new(HnswIo::new(dir, basename)));
                let hnsw: Hnsw<f32, DistL2> = reloader.load_hnsw()?;
                Ok(HnswIndex::L2(HnswInner {
                    hnsw,
                    _data: Vec::new(),
                    _dim: 0,
                    _n: 0,
                }))
            }
            DistanceMetric::Cosine => {
                let reloader = Box::leak(Box::new(HnswIo::new(dir, basename)));
                let hnsw: Hnsw<f32, DistCosine> = reloader.load_hnsw()?;
                Ok(HnswIndex::Cosine(HnswInner {
                    hnsw,
                    _data: Vec::new(),
                    _dim: 0,
                    _n: 0,
                }))
            }
            DistanceMetric::DotProduct => {
                let reloader = Box::leak(Box::new(HnswIo::new(dir, basename)));
                let hnsw: Hnsw<f32, DistDot> = reloader.load_hnsw()?;
                Ok(HnswIndex::Dot(HnswInner {
                    hnsw,
                    _data: Vec::new(),
                    _dim: 0,
                    _n: 0,
                }))
            }
        }
    }

    fn crate_name(&self) -> &str {
        "hnsw_rs"
    }

    fn supports_filtered_search(&self) -> bool {
        true
    }
}

fn build_inner<D: Distance<f32> + Default + Send + Sync + 'static>(
    vectors: &[f32],
    n: usize,
    dim: usize,
    config: &HnswBuildConfig,
) -> anyhow::Result<HnswInner<D>> {
    let max_layer = 16;
    let hnsw = Hnsw::<f32, D>::new(config.m, config.max_elements, max_layer, config.ef_construction, D::default());

    // Convert flat buffer to Vec<Vec<f32>> for insertion
    // hnsw_rs needs owned data that outlives the index
    let data: Vec<Vec<f32>> = (0..n)
        .map(|i| vectors[i * dim..(i + 1) * dim].to_vec())
        .collect();

    // Insert with parallel_insert for performance
    // hnsw_rs insert takes (&[T], usize) where usize is the data id
    for (id, vec) in data.iter().enumerate() {
        hnsw.insert((vec.as_slice(), id));
    }

    Ok(HnswInner {
        hnsw,
        _data: data,
        _dim: dim,
        _n: n,
    })
}

fn query_inner<D: Distance<f32> + Default + Send + Sync>(
    hnsw: &Hnsw<'_, f32, D>,
    vector: &[f32],
    k: usize,
    config: &HnswQueryConfig,
) -> anyhow::Result<Vec<QueryResult>> {
    let neighbours = hnsw.search(vector, k, config.ef_search);
    Ok(neighbours
        .iter()
        .map(|n| (n.d_id, n.distance))
        .collect())
}

fn filtered_query_inner<D: Distance<f32> + Default + Send + Sync>(
    hnsw: &Hnsw<'_, f32, D>,
    vector: &[f32],
    k: usize,
    config: &HnswQueryConfig,
    filter: &dyn Fn(usize) -> bool,
) -> anyhow::Result<Vec<QueryResult>> {
    // hnsw_rs FilterT is a trait — we need to wrap our closure
    struct ClosureFilter<'a>(&'a dyn Fn(usize) -> bool);

    impl<'a> hnsw_rs::filter::FilterT for ClosureFilter<'a> {
        fn hnsw_filter(&self, id: &usize) -> bool {
            (self.0)(*id)
        }
    }

    let wrapper = ClosureFilter(filter);
    let neighbours = hnsw.search_filter(vector, k, config.ef_search, Some(&wrapper));
    Ok(neighbours
        .iter()
        .map(|n| (n.d_id, n.distance))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_and_query() {
        let dim = 4;
        let vectors = vec![
            1.0f32, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        let config = HnswBuildConfig {
            m: 8,
            ef_construction: 50,
            max_elements: 100,
        };
        let index = HnswIndex::build(&vectors, 4, dim, DistanceMetric::Euclidean, &config).unwrap();

        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let qc = HnswQueryConfig { ef_search: 10 };
        let results = index.query(&query, 2, &qc).unwrap();

        assert!(!results.is_empty());
        // Closest should be vector 0
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 < 0.01);
    }

    #[test]
    fn test_filtered_query() {
        let dim = 2;
        let n = 100;
        let mut vectors = Vec::with_capacity(n * dim);
        for i in 0..n {
            vectors.push(i as f32);
            vectors.push(0.0);
        }

        let config = HnswBuildConfig {
            m: 8,
            ef_construction: 50,
            max_elements: 200,
        };
        let index =
            HnswIndex::build(&vectors, n, dim, DistanceMetric::Euclidean, &config).unwrap();

        let query = vec![50.0f32, 0.0];
        let qc = HnswQueryConfig { ef_search: 50 };

        // Only allow even-indexed vectors
        let results = index
            .filtered_query(&query, 5, &qc, &|id| id % 2 == 0)
            .unwrap();

        for (id, _dist) in &results {
            assert_eq!(id % 2, 0, "filtered result should be even, got {id}");
        }
    }

    #[test]
    fn test_default_sweep() {
        let sweep = default_sweep();
        assert_eq!(sweep.len(), 6);
        assert_eq!(sweep[0].ef_search, 10);
        assert_eq!(sweep[5].ef_search, 500);
    }

    #[test]
    fn test_save_and_load() {
        let dim = 4;
        let vectors = vec![
            1.0f32, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
            0.5, 0.5, 0.0, 0.0,
        ];
        let config = HnswBuildConfig {
            m: 8,
            ef_construction: 50,
            max_elements: 100,
        };
        let index =
            HnswIndex::build(&vectors, 5, dim, DistanceMetric::Euclidean, &config).unwrap();

        let tmp = tempfile::tempdir().unwrap();
        let save_path = tmp.path().join("test_index");
        let bytes = index.save(&save_path).unwrap();
        assert!(bytes > 0, "save should write non-zero bytes");

        // Load and query
        let loaded = HnswIndex::load(&save_path, DistanceMetric::Euclidean).unwrap();
        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let qc = HnswQueryConfig { ef_search: 10 };
        let results = loaded.query(&query, 2, &qc).unwrap();

        assert!(!results.is_empty());
        // Closest should be vector 0 (exact match)
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 < 0.01);
    }
}
