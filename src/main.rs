use anyhow::Result;
use clap::Parser;
use ntfs_catalog::cli::{Cli, run};

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}
