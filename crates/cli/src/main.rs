use clap::Parser;

use crate::commands::{Cli, Commands};

mod commands;
mod packager;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pack { path } => packager::pack_server(path),
        Commands::Unpack { path } => packager::unpack_server(path),
    }
}
