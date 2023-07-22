use clap::Parser;

use crate::commands::{Cli, Commands};

mod commands;
mod packager;
mod project;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install => project::install(),
        Commands::Pack { path } => packager::pack_server(path),
        Commands::Unpack {
            package_path,
            force_all,
        } => packager::unpack_server(package_path, force_all),
    }
}
