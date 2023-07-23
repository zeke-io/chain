use anyhow::Context;
use clap::Parser;
use core::metadata::ServerMetadata;
use core::{metadata, overrides};
use std::fs;
use std::path::Path;
use std::process::{Child, Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "msr")]
struct Args {
    #[arg(short, long)]
    prod: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let metadata = metadata::from_path("./mcs.yml").context("Cannot load metadata file")?;

    let directory = match metadata.server.server_directory.clone() {
        Some(directory) => directory,
        None => "./server".into(),
    };
    let directory = Path::new(&directory);

    if !directory.exists() || !directory.is_dir() {
        fs::create_dir_all(directory).with_context(|| {
            format!(
                "Could not create server directory \"{}\"",
                directory.display()
            )
        })?;
    }

    process_overrides(directory, args.prod)?;
    run_server(metadata, directory)?.wait()?;

    Ok(())
}

fn process_overrides(directory: &Path, prod_enabled: bool) -> anyhow::Result<()> {
    let is_dev = !prod_enabled;
    let override_data = overrides::from_folder("./");

    if let Some(data) = override_data {
        for file in data.keys() {
            let file_override = data.get(file);

            if let Some(file_override) = file_override {
                let file_source = if is_dev {
                    &file_override.source.dev
                } else {
                    &file_override.source.prod
                };
                let file_target_path = directory.join(file);
                let file_source_path = std::env::current_dir()?.join(file_source);

                fs::create_dir_all(file_target_path.parent().unwrap()).with_context(|| {
                    format!("Could not create file \"{}\".", file_target_path.display())
                })?;

                fs::copy(&file_source_path, &file_target_path).with_context(|| {
                    format!(
                        "Could not copy file \"{}\" to \"{}\".",
                        file_source_path.display(),
                        file_target_path.display()
                    )
                })?;
            }
        }
    }

    Ok(())
}

fn run_server(metadata: ServerMetadata, directory: &Path) -> anyhow::Result<Child> {
    let java_command = match metadata.runtime.java_path {
        Some(java) => java,
        None => "java".into(),
    };

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
