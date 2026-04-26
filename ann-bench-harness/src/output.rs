use std::fs;
use std::path::Path;

use ann_bench_core::BenchmarkResult;

/// Write a BenchmarkResult to a JSON file.
pub fn write_results_json(result: &BenchmarkResult, path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(result)?;
    fs::write(path, json)?;
    Ok(())
}

/// Read a BenchmarkResult from a JSON file.
pub fn read_results_json(path: &Path) -> anyhow::Result<BenchmarkResult> {
    let json = fs::read_to_string(path)?;
    let result: BenchmarkResult = serde_json::from_str(&json)?;
    Ok(result)
}
