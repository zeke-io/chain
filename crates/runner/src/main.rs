use anyhow::{anyhow, Context};
use chain::project;
use chain::project::manifests::{DependenciesManifest, DependencyDetails, VersionManifest};
use chain::project::settings::ProjectSettings;
use clap::Parser;
use std::collections::HashMap;
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
        prepare_dependencies(
            dependencies.dependencies,
            project.project_details.dependencies,
            server_directory.join("plugins"),
        )?;

        process_overrides(settings.clone(), server_directory.clone())?;
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

fn prepare_dependencies(
    cached_dependencies: HashMap<String, DependencyDetails>,
    dependencies: HashMap<String, String>,
    target_directory: PathBuf,
) -> anyhow::Result<()> {
    fn compare_dependencies(
        dependencies: HashMap<String, String>,
        cached_dependencies: &HashMap<String, DependencyDetails>,
    ) -> bool {
        if dependencies.len() != cached_dependencies.len() {
            return false;
        }

        for (id, source) in dependencies {
            if let Some(dep_details) = cached_dependencies.get(id.as_str()) {
                if source != dep_details.source {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    if !compare_dependencies(dependencies, &cached_dependencies) {
        return Err(anyhow!(
            "Detected dependency changes, make sure to run `chain install` first"
        ));
    }

    for (id, dep_details) in cached_dependencies {
        let dependency_file = Path::new(&dep_details.file_path);
        if !dependency_file.exists() {
            return Err(anyhow!(
                "Dependency \"{}\" was not found, make sure to run `chain install` first",
                id
            ));
        }

        fs::create_dir_all(&target_directory)?;
        fs::copy(
            &dependency_file,
            target_directory.join(
                dependency_file
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or(format!("{}.jar", id).as_str()),
            ),
        )?;
    }

    Ok(())
}

fn process_overrides<P: AsRef<Path>>(
    settings: ProjectSettings,
    server_directory: P,
) -> anyhow::Result<()> {
    let server_directory = server_directory.as_ref();
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
