use anyhow::Context;

use cli::{Cli, Commands, Parser};

use crate::project::packager;

pub mod project;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let current_directory = std::env::current_dir()?;

    // Initialize logger
    colog::init();

    match cli.command {
        Commands::New { path } => templater::generate_template(current_directory.join(path))
            .context("Generating template"),
        Commands::Install { force } => project::install(current_directory, force).await,
        Commands::Add { name } => project::add_dependency(current_directory, name).await,
        Commands::Run { prod, no_setup } => project::run(current_directory, prod, no_setup).await,
        Commands::Pack { dev } => packager::pack_server(current_directory, dev),
    }
}
