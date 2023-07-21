use std::fs;
use std::path::Path;
use anyhow::Context;
use std::process::{Child, Command, Stdio};
use common::mcs;
use common::mcs::ServerMetadata;

fn main() -> anyhow::Result<()> {
    let metadata = mcs::from_path("./mcs.toml").context("Cannot load metadata file")?;

    run_server(metadata)?.wait()?;

    Ok(())
}

fn run_server(metadata: ServerMetadata) -> anyhow::Result<Child> {
    let java_command = match metadata.runtime.java_path {
        Some(java) => java,
        None => "java".into(),
    };
    let directory = match metadata.server.directory {
        Some(directory) => directory,
        None => "./".into(),
    };
    let directory = Path::new(&directory);

    if !directory.exists() || !directory.is_dir() {
        fs::create_dir_all(directory)
            .with_context(|| format!("Could not create server directory \"{}\"", directory.display()))?;
    }

    let mut command = Command::new(java_command);
    command.current_dir(directory);

    if let Some(args) = metadata.runtime.jvm_options {
        for arg in args {
            command.arg(arg);
        }
    }

    command.arg("-jar");
    let jar_path = std::env::current_dir()?.join(metadata.runtime.server_jar);

    command.arg(jar_path);

    if let Some(args) = metadata.runtime.server_args {
        for arg in args {
            command.arg(arg);
        }
    }

    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Cannot initialize server jar")
}
