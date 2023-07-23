use anyhow::Context;
use clap::Parser;
use core::project::{ProjectSettings, VersionData};
use core::{overrides, project};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "msr")]
struct Args {
    #[arg(short, long)]
    dev: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let directory = std::env::current_dir()?;

    let project_data: project::ProjectData = project::ProjectData::load(&directory, args.dev)?;
    let server_directory = project_data.get_server_directory();

    if !server_directory.exists() || !server_directory.is_dir() {
        fs::create_dir_all(&server_directory).with_context(|| {
            format!(
                "Could not create server directory \"{}\"",
                server_directory.display()
            )
        })?;
    }

    let server_jar = VersionData::get_path(&project_data)
        .context("Could not find the version file, make sure to run `msc install` first")?;

    // process_overrides(server_directory, args.prod)?;
    run_server(
        server_directory.as_path(),
        server_jar,
        project_data.get_settings(),
    )?
    .wait()?;

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

fn run_server(
    server_directory: &Path,
    server_jar: PathBuf,
    settings: ProjectSettings,
) -> anyhow::Result<Child> {
    let mut command = Command::new(settings.java_runtime);
    command.current_dir(server_directory);

    for arg in settings.jvm_options {
        command.arg(arg);
    }

    command.arg("-jar");
    command.arg(server_jar);

    for arg in settings.server_args {
        command.arg(arg);
    }

    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Cannot initialize server jar")
}
