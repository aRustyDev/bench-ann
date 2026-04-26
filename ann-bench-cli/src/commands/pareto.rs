use std::path::PathBuf;

use clap::Args;

use ann_bench_core::pareto::compute_pareto_frontier;
use ann_bench_harness::output;

#[derive(Args)]
pub struct ParetoArgs {
    /// Result JSON files to process
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

pub fn execute(args: ParetoArgs) -> anyhow::Result<()> {
    for path in &args.files {
        let result = output::read_results_json(path)?;

        let points: Vec<(f64, f64)> = result
            .query_sweeps
            .iter()
            .map(|s| (s.recall_at_10, s.qps))
            .collect();

        let frontier = compute_pareto_frontier(&points);

        println!("=== {} ({}) ===", result.crate_name, result.dataset.name);
        println!("  {:>10}  {:>10}", "recall@10", "QPS");
        for p in &frontier {
            println!("  {:>10.4}  {:>10.0}", p.recall_at_10, p.qps);
        }
        println!();
    }

    Ok(())
}
