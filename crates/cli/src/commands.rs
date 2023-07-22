use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "msc")]
pub struct Cli {
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Installs the files and plugins")]
    Install {
        #[arg(short, long)]
        force: bool,
    },
    #[command(about = "Pack a server and its files")]
    Pack,
    #[command(about = "Unpack a server package")]
    Unpack {
        #[arg(short, long)]
        force: bool
    },
}
