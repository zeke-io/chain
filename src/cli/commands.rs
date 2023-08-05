use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "chain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a project with a basic template")]
    Init { path: Option<PathBuf> },
    #[command(about = "Install the files and plugins for a project")]
    Install {
        #[arg(short, long)]
        force: bool,
    },
    #[command(about = "Run the server project")]
    Run {
        #[arg(short, long)]
        prod: bool,
    },
    #[command(about = "Pack the server and its files")]
    Pack {
        #[arg(long)]
        dev: bool,
    },
}
