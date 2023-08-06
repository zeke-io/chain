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
    #[command(about = "Initializes a project with a basic template")]
    Init { path: Option<PathBuf> },
    #[command(about = "Installs the files and plugins")]
    Install {
        #[arg(short, long)]
        force: bool,
    },
    #[command(about = "Pack a server and its files")]
    Pack {
        #[arg(long)]
        dev: bool,
    },
}
