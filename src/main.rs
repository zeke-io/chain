pub mod commands;
pub mod logger;
pub mod project;
pub mod template;
pub mod util;

use crate::commands::{Cli, Commands};
use crate::project::packager;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let current_directory = std::env::current_dir()?;

    match cli.command {
        Commands::Init { path } => template::generate_template(path),
        Commands::Install { force } => project::install(current_directory, force).await,
        Commands::Run { prod, no_setup } => project::run(current_directory, prod, no_setup).await,
        Commands::Pack { dev } => packager::pack_server(current_directory, dev),
    }
}
