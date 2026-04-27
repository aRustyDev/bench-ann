use std::path::Path;

use ann_bench_core::{AnnIndex, BuildConfig, DistanceMetric, QueryConfig, QueryResult};
use instant_distance::{Builder, Hnsw, Search};
use serde::{Deserialize, Serialize};

/// A point wrapper that carries its data, original index, and distance metric.
#[derive(Clone, Serialize, Deserialize)]
pub struct MetricPoint {
    data: Vec<f32>,
    id: usize,
    metric: DistanceMetric,
}

impl instant_distance::Point for MetricPoint {
    fn distance(&self, other: &Self) -> f32 {
        debug_assert_eq!(self.data.len(), other.data.len());
        match self.metric {
            // Squared L2 — matches ground truth euclidean_sq
            DistanceMetric::Euclidean => self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| {
                    let d = a - b;
                    d * d
                })
                .sum(),
            // 1 - cosine_similarity — matches ground truth cosine_distance
            DistanceMetric::Cosine => {
                let dot: f32 = self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum();
                let norm_a: f32 = self.data.iter().map(|x| x * x).sum::<f32>().sqrt();
                let norm_b: f32 = other.data.iter().map(|x| x * x).sum::<f32>().sqrt();
                let denom = norm_a * norm_b;
                if denom == 0.0 {
                    1.0
                } else {
                    1.0 - (dot / denom)
                }
            }
            // Negated dot product — matches ground truth -dot(a,b)
            DistanceMetric::DotProduct => {
                let dot: f32 = self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum();
                -dot
            }
        }
    }
}

/// Build-time configuration for instant-distance.
///
/// Note: instant-distance hardcodes M=32. Only ef_construction is configurable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantDistanceBuildConfig {
    /// Size of the dynamic candidate list during construction.
    pub ef_construction: usize,
}

impl BuildConfig for InstantDistanceBuildConfig {
    fn name(&self) -> &str {
        Box::leak(format!("M=32,ef_c={}", self.ef_construction).into_boxed_str())
    }
}

impl Default for InstantDistanceBuildConfig {
    fn default() -> Self {
        Self {
            ef_construction: 200,
        }
    }
}

/// Query-time configuration for instant-distance.
///
/// instant-distance bakes ef_search into the index at build time. To sweep
/// different ef_search values, the index is rebuilt in `prepare_query()`.
/// This happens outside the measurement loop so it does not affect QPS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantDistanceQueryConfig {
    pub ef_search: usize,
}

impl QueryConfig for InstantDistanceQueryConfig {
    fn name(&self) -> &str {
        Box::leak(format!("ef_search={}", self.ef_search).into_boxed_str())
    }
}

/// Default parameter sweep for instant-distance benchmarks.
pub fn default_sweep() -> Vec<InstantDistanceQueryConfig> {
    [10, 20, 50, 100, 200, 500, 1000, 2000]
        .iter()
        .map(|&ef| InstantDistanceQueryConfig { ef_search: ef })
        .collect()
}

/// Wrapper around instant_distance::Hnsw that implements AnnIndex.
///
/// Stores the original points so the index can be rebuilt with different
/// ef_search values during parameter sweeps.
pub struct InstantDistanceIndex {
    hnsw: Hnsw<MetricPoint>,
    /// Kept for rebuilding with different ef_search in prepare_query.
    points: Vec<MetricPoint>,
    metric: DistanceMetric,
    dim: usize,
    ef_construction: usize,
    current_ef_search: usize,
}

impl AnnIndex for InstantDistanceIndex {
    type Build = InstantDistanceBuildConfig;
    type Query = InstantDistanceQueryConfig;

    fn build(
        vectors: &[f32],
        n: usize,
        dim: usize,
        metric: DistanceMetric,
        config: &Self::Build,
    ) -> anyhow::Result<Self> {
        let points: Vec<MetricPoint> = (0..n)
            .map(|i| MetricPoint {
                data: vectors[i * dim..(i + 1) * dim].to_vec(),
                id: i,
                metric,
            })
            .collect();

        let initial_ef_search = 100; // reasonable default, overridden by prepare_query
        let (hnsw, _ids) = Builder::default()
            .ef_construction(config.ef_construction)
            .ef_search(initial_ef_search)
            .build_hnsw(points.clone());

        Ok(InstantDistanceIndex {
            hnsw,
            points,
            metric,
            dim,
            ef_construction: config.ef_construction,
            current_ef_search: initial_ef_search,
        })
    }

    fn prepare_query(&mut self, config: &Self::Query) {
        if config.ef_search != self.current_ef_search {
            let (hnsw, _ids) = Builder::default()
                .ef_construction(self.ef_construction)
                .ef_search(config.ef_search)
                .build_hnsw(self.points.clone());
            self.hnsw = hnsw;
            self.current_ef_search = config.ef_search;
        }
    }

    fn query(
        &self,
        vector: &[f32],
        k: usize,
        _config: &Self::Query,
    ) -> anyhow::Result<Vec<QueryResult>> {
        let query_point = MetricPoint {
            data: vector.to_vec(),
            id: usize::MAX,
            metric: self.metric,
        };

        let mut search = Search::default();
        let results: Vec<QueryResult> = self
            .hnsw
            .search(&query_point, &mut search)
            .take(k)
            .map(|item| (item.point.id, item.distance))
            .collect();

        Ok(results)
    }

    fn save(&self, path: &Path) -> anyhow::Result<u64> {
        let wrapper = SerializedIndex {
            hnsw: &self.hnsw,
            points: &self.points,
            metric: self.metric,
            dim: self.dim,
            ef_construction: self.ef_construction,
            ef_search: self.current_ef_search,
        };
        let bytes = bincode::serialize(&wrapper)?;
        std::fs::write(path, &bytes)?;
        Ok(bytes.len() as u64)
    }

    fn load(path: &Path, metric: DistanceMetric) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;
        let loaded: OwnedSerializedIndex = bincode::deserialize(&bytes)?;

        anyhow::ensure!(
            loaded.metric == metric,
            "metric mismatch: saved {:?}, requested {:?}",
            loaded.metric,
            metric
        );

        Ok(InstantDistanceIndex {
            hnsw: loaded.hnsw,
            dim: loaded.dim,
            metric: loaded.metric,
            ef_construction: loaded.ef_construction,
            current_ef_search: loaded.ef_search,
            points: loaded.points,
        })
    }

    fn crate_name(&self) -> &str {
        "instant-distance"
    }

    fn supports_filtered_search(&self) -> bool {
        false
    }
}

/// Serialization wrapper (borrows from the index).
#[derive(Serialize)]
struct SerializedIndex<'a> {
    hnsw: &'a Hnsw<MetricPoint>,
    points: &'a [MetricPoint],
    metric: DistanceMetric,
    dim: usize,
    ef_construction: usize,
    ef_search: usize,
}

/// Deserialization wrapper (owns the data).
#[derive(Deserialize)]
struct OwnedSerializedIndex {
    hnsw: Hnsw<MetricPoint>,
    points: Vec<MetricPoint>,
    metric: DistanceMetric,
    dim: usize,
    ef_construction: usize,
    ef_search: usize,
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
        let config = InstantDistanceBuildConfig {
            ef_construction: 50,
        };
        let index = InstantDistanceIndex::build(
            &vectors,
            4,
            dim,
            DistanceMetric::Euclidean,
            &config,
        )
        .unwrap();

        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let qc = InstantDistanceQueryConfig { ef_search: 10 };
        let results = index.query(&query, 2, &qc).unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 < 0.01);
    }

    #[test]
    fn test_prepare_query_rebuilds() {
        let dim = 4;
        let vectors = vec![
            1.0f32, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let config = InstantDistanceBuildConfig {
            ef_construction: 50,
        };
        let mut index = InstantDistanceIndex::build(
            &vectors,
            4,
            dim,
            DistanceMetric::Euclidean,
            &config,
        )
        .unwrap();

        // Trigger rebuild with different ef_search
        let qc = InstantDistanceQueryConfig { ef_search: 200 };
        index.prepare_query(&qc);
        assert_eq!(index.current_ef_search, 200);

        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let results = index.query(&query, 2, &qc).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].0, 0);
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
        let config = InstantDistanceBuildConfig {
            ef_construction: 50,
        };
        let index = InstantDistanceIndex::build(
            &vectors,
            5,
            dim,
            DistanceMetric::Euclidean,
            &config,
        )
        .unwrap();

        let tmp = tempfile::tempdir().unwrap();
        let save_path = tmp.path().join("test_index.bin");
        let bytes = index.save(&save_path).unwrap();
        assert!(bytes > 0, "save should write non-zero bytes");

        let loaded =
            InstantDistanceIndex::load(&save_path, DistanceMetric::Euclidean).unwrap();
        let query = vec![1.0f32, 0.0, 0.0, 0.0];
        let qc = InstantDistanceQueryConfig { ef_search: 10 };
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
