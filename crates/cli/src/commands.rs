use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "msc")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Pack a server and its files")]
    Pack { path: Option<String> },
    #[command(about = "Unpack a server package")]
    Unpack {
        #[arg(short, long)]
        force_all: bool,
        package_path: String,
    },
}
