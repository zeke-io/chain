use chain::project;
use clap::Parser;
use std::path::PathBuf;

use crate::commands::{Cli, Commands};

mod commands;
mod packager;
mod template;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_directory = std::env::current_dir()?;

    match cli.command {
        Commands::Init { path } => template::generate_template(path),
        Commands::Install { force } => project::install(current_directory, force).await,
    }
}
