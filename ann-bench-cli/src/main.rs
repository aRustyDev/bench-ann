mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ann-bench", about = "ANN Benchmark Harness for Vector DS&A")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run benchmarks for specified adapters
    Run(commands::run::RunArgs),
    /// Compute and cache ground truth for a dataset
    GroundTruth(commands::ground_truth::GroundTruthArgs),
    /// Compute Pareto frontiers from result JSON files
    Pareto(commands::pareto::ParetoArgs),
    /// Classify results against metric tier thresholds
    Tier(commands::tier::TierArgs),
    /// List available adapters and datasets
    List(commands::list::ListArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => commands::run::execute(args),
        Commands::GroundTruth(args) => commands::ground_truth::execute(args),
        Commands::Pareto(args) => commands::pareto::execute(args),
        Commands::Tier(args) => commands::tier::execute(args),
        Commands::List(args) => commands::list::execute(args),
    }
}
