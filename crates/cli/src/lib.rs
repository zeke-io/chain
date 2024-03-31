pub use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "chain", bin_name = "chain", author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a project with a basic template
    New { path: PathBuf },
    /// Install the files and plugins for a project
    Install {
        #[arg(short, long)]
        force: bool,
    },
    /// Add a plugin to the project
    Add { name: String },
    /// Run the server project
    Run {
        #[arg(short, long)]
        prod: bool,
        #[arg(long)]
        no_setup: bool,
    },
    /// Pack the server and its files
    Pack {
        #[arg(long)]
        dev: bool,
    },
}
