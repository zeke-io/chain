use clap::Parser;
use core::project;
use std::path::PathBuf;

use crate::commands::{Cli, Commands};

mod commands;
mod packager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let directory = match cli.path {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir()?,
    };

    match cli.command {
        Commands::Install { force } => project::install(directory, force).await,
        _ => {
            println!("This command is not implemented yet!");
            Ok(())
        }
    }
}
