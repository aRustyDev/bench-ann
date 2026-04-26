use std::path::Path;

use crate::types::{BuildConfig, DistanceMetric, QueryConfig, QueryResult};

/// Core trait that every crate adapter must implement.
pub trait AnnIndex: Sized {
    type Build: BuildConfig;
    type Query: QueryConfig;

    /// Build an index from contiguous vector data.
    /// `vectors` is a flat buffer of n*dim f32 values, row-major.
    fn build(
        vectors: &[f32],
        n: usize,
        dim: usize,
        metric: DistanceMetric,
        config: &Self::Build,
    ) -> anyhow::Result<Self>;

    /// Optional: pre-allocate internal buffers for the given query config.
    /// Called once before a query pass. Default: no-op.
    fn prepare_query(&mut self, _config: &Self::Query) {}

    /// Query the index for the k nearest neighbors.
    /// Returns results sorted by distance (ascending).
    fn query(
        &self,
        vector: &[f32],
        k: usize,
        config: &Self::Query,
    ) -> anyhow::Result<Vec<QueryResult>>;

    /// Query with a filter predicate. Only vectors where `filter(id)` returns
    /// true are eligible results.
    fn filtered_query(
        &self,
        _vector: &[f32],
        _k: usize,
        _config: &Self::Query,
        _filter: &dyn Fn(usize) -> bool,
    ) -> anyhow::Result<Vec<QueryResult>> {
        Err(anyhow::anyhow!("filtered search not supported"))
    }

    /// Serialize the index to disk. Returns the total bytes written.
    fn save(&self, path: &Path) -> anyhow::Result<u64>;

    /// Load a serialized index from disk.
    fn load(path: &Path, metric: DistanceMetric) -> anyhow::Result<Self>;

    /// Human-readable crate name (e.g., "hnsw_rs").
    fn crate_name(&self) -> &str;

    /// Whether this adapter supports filtered queries.
    fn supports_filtered_search(&self) -> bool {
        false
    }

    /// Whether this adapter supports incremental insert/delete.
    fn supports_incremental_updates(&self) -> bool {
        false
    }

    /// Insert vectors into an existing index (if supported).
    fn insert(
        &mut self,
        _vectors: &[f32],
        _n: usize,
        _dim: usize,
        _start_id: usize,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("incremental insert not supported"))
    }

    /// Delete vectors from an existing index by ID (if supported).
    fn delete(&mut self, _ids: &[usize]) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("incremental delete not supported"))
    }
}
