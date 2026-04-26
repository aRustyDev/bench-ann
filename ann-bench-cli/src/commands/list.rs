use clap::Args;

#[derive(Args)]
pub struct ListArgs {
    /// List only adapters
    #[arg(long)]
    pub adapters: bool,
    /// List only datasets
    #[arg(long)]
    pub datasets: bool,
}

/// Known adapters (compiled into the binary).
const ADAPTERS: &[(&str, &str)] = &[
    ("hnsw-rs", "NSW/HNSW — pure Rust, filtered search"),
];

/// Built-in dataset definitions.
const DATASETS: &[(&str, &str)] = &[
    ("sift-128", "SIFT1M: 1M vectors, 128d, euclidean (fvecs)"),
    ("synthetic-128", "Synthetic: 1M vectors, 128d"),
    ("synthetic-384", "Synthetic: 1M vectors, 384d"),
    ("synthetic-768", "Synthetic: 1M vectors, 768d"),
    ("synthetic-1536", "Synthetic: 1M vectors, 1536d"),
];

pub fn execute(args: ListArgs) -> anyhow::Result<()> {
    let show_all = !args.adapters && !args.datasets;

    if show_all || args.adapters {
        println!("Adapters:");
        for (name, desc) in ADAPTERS {
            println!("  {name:<20} {desc}");
        }
        if show_all {
            println!();
        }
    }

    if show_all || args.datasets {
        println!("Datasets:");
        for (name, desc) in DATASETS {
            println!("  {name:<20} {desc}");
        }
    }

    Ok(())
}
