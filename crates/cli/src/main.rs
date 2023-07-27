use chain_core::project;
use clap::Parser;
use std::path::PathBuf;

use crate::commands::{Cli, Commands};

mod commands;
mod packager;
mod template;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let directory = match cli.path {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir()?,
    };

    match cli.command {
        Commands::Init => template::generate_template(directory),
        Commands::Install { force } => project::install(directory, force).await,
        _ => {
            println!("This command is not implemented yet!");
            Ok(())
        }
    }
}
