use serde::{Deserialize, Serialize};

/// Distance metric for vector similarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DistanceMetric {
    Euclidean,
    Cosine,
    DotProduct,
}

impl std::fmt::Display for DistanceMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Euclidean => write!(f, "euclidean"),
            Self::Cosine => write!(f, "cosine"),
            Self::DotProduct => write!(f, "dotproduct"),
        }
    }
}

/// Query result: (vector_index, distance).
pub type QueryResult = (usize, f32);

/// Opaque build-time parameters, serialized as JSON for output.
pub trait BuildConfig: Serialize + std::fmt::Debug {
    fn name(&self) -> &str;
}

/// Opaque query-time parameters, serialized as JSON for output.
pub trait QueryConfig: Serialize + std::fmt::Debug {
    fn name(&self) -> &str;
}
