use serde::{Deserialize, Serialize};

/// Hardware metadata captured at benchmark time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: String,
    pub cores_used: u32,
    pub ram_gb: u32,
    pub os: String,
    pub storage: String,
}

/// Dataset metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name: String,
    pub source: String,
    pub n_vectors: usize,
    pub n_queries: usize,
    pub dimension: usize,
    pub metric: String,
}

/// Build phase results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub config: serde_json::Value,
    pub time_s: f64,
    pub memory_per_vector_bytes: i64,
    pub rss_before_bytes: u64,
    pub rss_after_bytes: u64,
}

/// Index size on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSizeResult {
    pub disk_bytes: u64,
    pub disk_per_vector_bytes: f64,
}

/// A single query sweep result (one parameter setting).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySweepResult {
    pub config: serde_json::Value,
    pub recall_at_1: f64,
    pub recall_at_10: f64,
    pub recall_at_100: f64,
    pub qps: f64,
    pub latency_p50_us: u64,
    pub latency_p99_us: u64,
    pub run_times_s: Vec<f64>,
    pub best_run_s: f64,
}

/// Filtered benchmark results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredResult {
    pub cardinality: usize,
    pub selectivity: f64,
    pub recall_at_10: f64,
    pub qps: f64,
    pub unfiltered_recall_at_10: f64,
    pub unfiltered_qps: f64,
}

/// Incremental update benchmark results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalResult {
    pub inserted: usize,
    pub deleted: usize,
    pub recall_at_10_after_update: f64,
    pub qps_after_update: f64,
    pub recall_at_10_fresh_build: f64,
    pub qps_fresh_build: f64,
}

/// Pareto frontier point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParetoPoint {
    pub recall_at_10: f64,
    pub qps: f64,
}

/// Tier classification for a metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Unacceptable,
    Acceptable,
    Good,
    Excellent,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unacceptable => write!(f, "unacceptable"),
            Self::Acceptable => write!(f, "acceptable"),
            Self::Good => write!(f, "good"),
            Self::Excellent => write!(f, "excellent"),
        }
    }
}

/// Tier classifications for all primary metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierClassification {
    pub recall_at_10: Tier,
    pub qps_at_0_95_recall: Tier,
    pub build_time: Tier,
    pub memory_per_vector: Tier,
    pub disk_per_vector: Tier,
    pub latency_p99: Tier,
}

/// Top-level benchmark result for one (crate, dataset, build_config) combination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub schema_version: String,
    pub harness_version: String,
    pub timestamp: String,
    pub hardware: HardwareInfo,
    pub crate_name: String,
    pub crate_version: String,
    pub dataset: DatasetInfo,
    pub build: BuildResult,
    pub index_size: IndexSizeResult,
    pub query_sweeps: Vec<QuerySweepResult>,
    pub filtered: Option<Vec<FilteredResult>>,
    pub incremental: Option<IncrementalResult>,
    pub pareto_frontier: Vec<ParetoPoint>,
    pub tier_classification: TierClassification,
}
