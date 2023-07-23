use anyhow::{anyhow, Context};
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

    process_overrides(project_data.get_settings(), server_directory.as_path())?;
    run_server(
        server_directory.as_path(),
        server_jar,
        project_data.get_settings(),
    )?
    .wait()?;

    Ok(())
}

fn process_overrides(settings: ProjectSettings, server_directory: &Path) -> anyhow::Result<()> {
    for file_target in settings.overrides.keys() {
        let value = settings.overrides.get(file_target).unwrap();
        let source_file = Path::new(value);

        if !source_file.exists() {
            return Err(anyhow!(
                "Override file \"{}\" does not exists",
                source_file.display()
            ));
        }

        let file_target = server_directory.join(file_target);

        fs::create_dir_all(file_target.parent().unwrap())
            .with_context(|| format!("Could not create file \"{}\".", file_target.display()))?;

        fs::copy(&source_file, &file_target).with_context(|| {
            format!(
                "Could not copy file \"{}\" to \"{}\".",
                source_file.display(),
                file_target.display()
            )
        })?;
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
