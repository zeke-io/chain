use clap::Parser;

use crate::commands::{Cli, Commands};

mod commands;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pack { .. } => {}
        Commands::Unpack { .. } => {}
    }
}
