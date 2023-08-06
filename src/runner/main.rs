use chain::project;
use clap::Parser;

// This whole runner binary is currently deprecated in favor of the run command in chain,
// but I am not deleting this yet because I do have some plans for this (for future versions).
// - zeke-io
// TODO: Expand, re-implement or delete this

#[derive(Parser, Debug)]
#[command(name = "chainr", bin_name = "chainr", author, version)]
struct Args {
    #[arg(short, long)]
    dev: bool,
    #[arg(long)]
    no_setup: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let directory = std::env::current_dir()?;

    project::run(directory, !args.dev, args.no_setup).await?;
    Ok(())
}
