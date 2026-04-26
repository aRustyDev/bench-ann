use std::path::PathBuf;

use clap::Args;

use ann_bench_harness::output;

#[derive(Args)]
pub struct TierArgs {
    /// Result JSON files to classify
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

pub fn execute(args: TierArgs) -> anyhow::Result<()> {
    for path in &args.files {
        let result = output::read_results_json(path)?;
        let t = &result.tier_classification;

        println!("=== {} ({}) ===", result.crate_name, result.dataset.name);
        println!("  recall@10:        {}", t.recall_at_10);
        println!("  QPS@0.95 recall:  {}", t.qps_at_0_95_recall);
        println!("  build time:       {}", t.build_time);
        println!("  memory/vector:    {}", t.memory_per_vector);
        println!("  disk/vector:      {}", t.disk_per_vector);
        println!("  latency p99:      {}", t.latency_p99);
        println!();
    }

    Ok(())
}
