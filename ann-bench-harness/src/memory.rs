/// Snapshot current process RSS (resident set size) in bytes.
///
/// Uses the `memory-stats` crate which captures mmap'd memory.
/// Returns `None` if the platform doesn't support RSS reporting.
pub fn snapshot_rss() -> Option<u64> {
    memory_stats::memory_stats().map(|stats| stats.physical_mem as u64)
}
