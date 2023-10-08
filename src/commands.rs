use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "chain", bin_name = "chain", author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a project with a basic template")]
    New { path: PathBuf },
    #[command(about = "Install the files and plugins for a project")]
    Install {
        #[arg(short, long)]
        force: bool,
    },
    #[command(about = "Add a plugin to the project")]
    Add { name: String },
    #[command(about = "Run the server project")]
    Run {
        #[arg(short, long)]
        prod: bool,
        #[arg(long)]
        no_setup: bool,
    },
    #[command(about = "Pack the server and its files")]
    Pack {
        #[arg(long)]
        dev: bool,
    },
}
