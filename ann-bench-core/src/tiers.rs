use crate::results::{QuerySweepResult, Tier, TierClassification};

/// Classify recall@10 into a tier.
pub fn tier_recall(recall: f64) -> Tier {
    if recall >= 0.99 {
        Tier::Excellent
    } else if recall >= 0.95 {
        Tier::Good
    } else if recall >= 0.90 {
        Tier::Acceptable
    } else {
        Tier::Unacceptable
    }
}

/// Classify QPS (at recall >= 0.95, 128d 1M vectors) into a tier.
pub fn tier_qps(qps: f64) -> Tier {
    if qps > 10_000.0 {
        Tier::Excellent
    } else if qps >= 1_000.0 {
        Tier::Good
    } else if qps >= 100.0 {
        Tier::Acceptable
    } else {
        Tier::Unacceptable
    }
}

/// Classify build time (seconds, 1M 128d single-thread) into a tier.
pub fn tier_build_time(seconds: f64) -> Tier {
    if seconds < 10.0 {
        Tier::Excellent
    } else if seconds < 60.0 {
        Tier::Good
    } else if seconds <= 600.0 {
        Tier::Acceptable
    } else {
        Tier::Unacceptable
    }
}

/// Classify memory per vector (bytes, 128d f32) into a tier.
pub fn tier_memory_per_vector(bytes: i64) -> Tier {
    if bytes < 128 {
        Tier::Excellent
    } else if bytes < 512 {
        Tier::Good
    } else if bytes <= 2048 {
        Tier::Acceptable
    } else {
        Tier::Unacceptable
    }
}

/// Classify disk per vector (bytes, 128d f32) into a tier.
pub fn tier_disk_per_vector(bytes: f64) -> Tier {
    if bytes < 256.0 {
        Tier::Excellent
    } else if bytes < 1024.0 {
        Tier::Good
    } else if bytes <= 4096.0 {
        Tier::Acceptable
    } else {
        Tier::Unacceptable
    }
}

/// Classify latency p99 (microseconds, 1M 128d) into a tier.
pub fn tier_latency_p99(us: u64) -> Tier {
    let ms = us as f64 / 1000.0;
    if ms < 1.0 {
        Tier::Excellent
    } else if ms < 10.0 {
        Tier::Good
    } else if ms <= 100.0 {
        Tier::Acceptable
    } else {
        Tier::Unacceptable
    }
}

/// Compute full tier classification from benchmark data.
///
/// `sweeps` — all query sweep results for this benchmark.
/// `build_time_s` — build time in seconds.
/// `memory_per_vector` — memory per vector in bytes.
/// `disk_per_vector` — disk per vector in bytes.
pub fn classify_tiers(
    sweeps: &[QuerySweepResult],
    build_time_s: f64,
    memory_per_vector: i64,
    disk_per_vector: f64,
) -> TierClassification {
    // Best recall@10 across all sweeps
    let best_recall = sweeps
        .iter()
        .map(|s| s.recall_at_10)
        .fold(0.0_f64, f64::max);

    // QPS at recall >= 0.95 (best QPS among sweeps meeting the threshold)
    let qps_at_target = sweeps
        .iter()
        .filter(|s| s.recall_at_10 >= 0.95)
        .map(|s| s.qps)
        .fold(0.0_f64, f64::max);

    // Best (lowest) p99 latency among sweeps with recall >= 0.95
    let p99_at_target = sweeps
        .iter()
        .filter(|s| s.recall_at_10 >= 0.95)
        .map(|s| s.latency_p99_us)
        .min()
        .unwrap_or(u64::MAX);

    TierClassification {
        recall_at_10: tier_recall(best_recall),
        qps_at_0_95_recall: tier_qps(qps_at_target),
        build_time: tier_build_time(build_time_s),
        memory_per_vector: tier_memory_per_vector(memory_per_vector),
        disk_per_vector: tier_disk_per_vector(disk_per_vector),
        latency_p99: tier_latency_p99(p99_at_target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_recall() {
        assert_eq!(tier_recall(0.995), Tier::Excellent);
        assert_eq!(tier_recall(0.97), Tier::Good);
        assert_eq!(tier_recall(0.92), Tier::Acceptable);
        assert_eq!(tier_recall(0.85), Tier::Unacceptable);
    }

    #[test]
    fn test_tier_qps() {
        assert_eq!(tier_qps(15000.0), Tier::Excellent);
        assert_eq!(tier_qps(5000.0), Tier::Good);
        assert_eq!(tier_qps(500.0), Tier::Acceptable);
        assert_eq!(tier_qps(50.0), Tier::Unacceptable);
    }

    #[test]
    fn test_tier_build_time() {
        assert_eq!(tier_build_time(5.0), Tier::Excellent);
        assert_eq!(tier_build_time(30.0), Tier::Good);
        assert_eq!(tier_build_time(300.0), Tier::Acceptable);
        assert_eq!(tier_build_time(700.0), Tier::Unacceptable);
    }

    #[test]
    fn test_tier_memory() {
        assert_eq!(tier_memory_per_vector(64), Tier::Excellent);
        assert_eq!(tier_memory_per_vector(300), Tier::Good);
        assert_eq!(tier_memory_per_vector(1024), Tier::Acceptable);
        assert_eq!(tier_memory_per_vector(4096), Tier::Unacceptable);
    }

    #[test]
    fn test_tier_disk() {
        assert_eq!(tier_disk_per_vector(128.0), Tier::Excellent);
        assert_eq!(tier_disk_per_vector(512.0), Tier::Good);
        assert_eq!(tier_disk_per_vector(2048.0), Tier::Acceptable);
        assert_eq!(tier_disk_per_vector(8192.0), Tier::Unacceptable);
    }

    #[test]
    fn test_tier_latency_p99() {
        assert_eq!(tier_latency_p99(500), Tier::Excellent);
        assert_eq!(tier_latency_p99(5000), Tier::Good);
        assert_eq!(tier_latency_p99(50_000), Tier::Acceptable);
        assert_eq!(tier_latency_p99(200_000), Tier::Unacceptable);
    }
}
