use std::path::Path;

use ann_bench_core::{AnnIndex, BuildConfig, DistanceMetric, QueryConfig, QueryResult};
use serde::{Deserialize, Serialize};
use usearch::{Index, IndexOptions, MetricKind, ScalarKind};

/// Build-time configuration for usearch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsearchBuildConfig {
    /// Number of bidirectional links per node (M parameter).
    #[serde(rename = "M")]
    pub m: usize,
    /// Size of the dynamic candidate list during construction.
    pub ef_construction: usize,
}

impl BuildConfig for UsearchBuildConfig {
    fn name(&self) -> &str {
        Box::leak(format!("M={},ef_c={}", self.m, self.ef_construction).into_boxed_str())
    }
}

impl Default for UsearchBuildConfig {
    fn default() -> Self {
        Self {
            m: 16,
            ef_construction: 200,
        }
    }
}

/// Query-time configuration for usearch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsearchQueryConfig {
    pub ef_search: usize,
}

impl QueryConfig for UsearchQueryConfig {
    fn name(&self) -> &str {
        Box::leak(format!("ef_search={}", self.ef_search).into_boxed_str())
    }
}

/// Default parameter sweep for usearch benchmarks.
pub fn default_sweep() -> Vec<UsearchQueryConfig> {
    [10, 20, 50, 100, 200, 500]
        .iter()
        .map(|&ef| UsearchQueryConfig { ef_search: ef })
        .collect()
}

fn to_metric_kind(metric: DistanceMetric) -> MetricKind {
    match metric {
        DistanceMetric::Euclidean => MetricKind::L2sq,
        DistanceMetric::Cosine => MetricKind::Cos,
        DistanceMetric::DotProduct => MetricKind::IP,
    }
}

/// Wrapper around usearch::Index that implements AnnIndex.
pub struct UsearchIndex {
    index: Index,
    #[allow(dead_code)] // kept for load() round-trip
    metric: DistanceMetric,
}

impl AnnIndex for UsearchIndex {
    type Build = UsearchBuildConfig;
    type Query = UsearchQueryConfig;

    fn build(
        vectors: &[f32],
        n: usize,
        dim: usize,
        metric: DistanceMetric,
        config: &Self::Build,
    ) -> anyhow::Result<Self> {
        let options = IndexOptions {
            dimensions: dim,
            metric: to_metric_kind(metric),
            quantization: ScalarKind::F32,
            connectivity: config.m,
            expansion_add: config.ef_construction,
            expansion_search: 200, // default, overridden per-query
            multi: false,
        };
        let index = Index::new(&options)
            .map_err(|e| anyhow::anyhow!("usearch Index::new failed: {e}"))?;
        index
            .reserve(n)
            .map_err(|e| anyhow::anyhow!("usearch reserve failed: {e}"))?;

        for i in 0..n {
            let vec = &vectors[i * dim..(i + 1) * dim];
            index
                .add(i as u64, vec)
                .map_err(|e| anyhow::anyhow!("usearch add({i}) failed: {e}"))?;
        }

        Ok(UsearchIndex { index, metric })
    }

    fn query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
    ) -> anyhow::Result<Vec<QueryResult>> {
        self.index.change_expansion_search(config.ef_search);
        let matches = self
            .index
            .search(vector, k)
            .map_err(|e| anyhow::anyhow!("usearch search failed: {e}"))?;
        Ok(matches
            .keys
            .iter()
            .zip(matches.distances.iter())
            .map(|(&key, &dist)| (key as usize, dist))
            .collect())
    }

    fn filtered_query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
        filter: &dyn Fn(usize) -> bool,
    ) -> anyhow::Result<Vec<QueryResult>> {
        self.index.change_expansion_search(config.ef_search);
        let matches = self
            .index
            .filtered_search(vector, k, |key: usearch::Key| filter(key as usize))
            .map_err(|e| anyhow::anyhow!("usearch filtered_search failed: {e}"))?;
        Ok(matches
            .keys
            .iter()
            .zip(matches.distances.iter())
            .map(|(&key, &dist)| (key as usize, dist))
            .collect())
    }

    fn save(&self, path: &Path) -> anyhow::Result<u64> {
        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non-UTF8 path"))?;
        self.index
            .save(path_str)
            .map_err(|e| anyhow::anyhow!("usearch save failed: {e}"))?;
        let bytes = std::fs::metadata(path)?.len();
        Ok(bytes)
    }

    fn load(path: &Path, metric: DistanceMetric) -> anyhow::Result<Self> {
        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non-UTF8 path"))?;
        // Create index with placeholder options; load restores real metadata.
        let options = IndexOptions {
            dimensions: 1,
            metric: to_metric_kind(metric),
            quantization: ScalarKind::F32,
            connectivity: 16,
            expansion_add: 128,
            expansion_search: 64,
            multi: false,
        };
        let index = Index::new(&options)
            .map_err(|e| anyhow::anyhow!("usearch Index::new for load failed: {e}"))?;
        index
            .load(path_str)
            .map_err(|e| anyhow::anyhow!("usearch load failed: {e}"))?;
        Ok(UsearchIndex { index, metric })
    }

    fn crate_name(&self) -> &str {
        "usearch"
    }

    fn supports_filtered_search(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_and_query() {
        let dim = 4;
        let vectors = vec![
            1.0f32, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let config = UsearchBuildConfig {
            m: 8,
            ef_construction: 50,
        };
        let index =
            UsearchIndex::build(&vectors, 4, dim, DistanceMetric::Euclidean, &config).unwrap();

        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let qc = UsearchQueryConfig { ef_search: 10 };
        let results = index.query(&query, 2, &qc).unwrap();

        assert!(!results.is_empty());
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

        let config = UsearchBuildConfig {
            m: 8,
            ef_construction: 50,
        };
        let index =
            UsearchIndex::build(&vectors, n, dim, DistanceMetric::Euclidean, &config).unwrap();

        let query = vec![50.0f32, 0.0];
        let qc = UsearchQueryConfig { ef_search: 50 };

        let results = index
            .filtered_query(&query, 5, &qc, &|id| id % 2 == 0)
            .unwrap();

        for (id, _dist) in &results {
            assert_eq!(id % 2, 0, "filtered result should be even, got {id}");
        }
    }

    #[test]
    fn test_save_and_load() {
        let dim = 4;
        let vectors = vec![
            1.0f32, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
            0.5, 0.5, 0.0, 0.0,
        ];
        let config = UsearchBuildConfig {
            m: 8,
            ef_construction: 50,
        };
        let index =
            UsearchIndex::build(&vectors, 5, dim, DistanceMetric::Euclidean, &config).unwrap();

        let tmp = tempfile::tempdir().unwrap();
        let save_path = tmp.path().join("test_index.usearch");
        let bytes = index.save(&save_path).unwrap();
        assert!(bytes > 0, "save should write non-zero bytes");

        let loaded = UsearchIndex::load(&save_path, DistanceMetric::Euclidean).unwrap();
        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let qc = UsearchQueryConfig { ef_search: 10 };
        let results = loaded.query(&query, 2, &qc).unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 < 0.01);
    }

    #[test]
    fn test_default_sweep() {
        let sweep = default_sweep();
        assert_eq!(sweep.len(), 6);
        assert_eq!(sweep[0].ef_search, 10);
        assert_eq!(sweep[5].ef_search, 500);
    }
}
