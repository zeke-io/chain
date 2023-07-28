use anyhow::Context;
use chain::project;
use chain::project::manifests::{DependenciesManifest, VersionManifest};
use chain::project::settings::ProjectSettings;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use termion::{color, style};

#[derive(Parser, Debug)]
#[command(name = "chainr")]
struct Args {
    #[arg(short, long)]
    dev: bool,
    #[arg(long)]
    no_setup: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let directory = std::env::current_dir()?;

    let project = project::load_project(&directory)?;
    let settings = match project.get_settings(args.dev) {
        Ok(settings) => settings,
        Err(_) => {
            println!(
                "{}No settings file was provided, using default values...{}",
                color::Fg(color::Yellow),
                style::Reset
            );
            ProjectSettings::default()
        }
    };
    let version = project
        .get_manifest::<VersionManifest>()
        .context("Version manifest file was not found, make sure to run `chain install` first")?;
    let dependencies = project.get_manifest::<DependenciesManifest>().context(
        "Dependencies manifest file was not found, make sure to run `chain install` first",
    )?;

    let server_jar = PathBuf::from(version.jar_file);

    let server_directory = directory.join("server");
    if !server_directory.exists() || !server_directory.is_dir() {
        fs::create_dir_all(&server_directory).with_context(|| {
            format!(
                "Could not create server directory \"{}\"",
                server_directory.display()
            )
        })?;
    }

    if !args.no_setup {
        project::prepare_dependencies(
            dependencies.dependencies,
            project.project_details.dependencies,
            server_directory.join("plugins"),
        )?;

        project::process_files(settings.clone(), server_directory.clone())?;
    } else {
        println!(
            "{}Skipping setup, this is only recommended when running the server for the first time...{}",
            color::Fg(color::Yellow),
            style::Reset
        );
    }

    println!(
        "{}Running server...{}",
        color::Fg(color::Green),
        style::Reset
    );
    run_server(server_directory, server_jar, settings)?.wait()?;
    Ok(())
}

fn run_server<P: AsRef<Path>>(
    server_directory: P,
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
