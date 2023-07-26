use anyhow::{anyhow, Context};
use chain_core::project;
use chain_core::project::manifests::VersionManifest;
use chain_core::project::metadata::ProjectMetadata;
use chain_core::project::settings::ProjectSettings;
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "chainr")]
struct Args {
    #[arg(short, long)]
    dev: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let directory = std::env::current_dir()?;

    let project = project::load_project(&directory)?;
    let settings = project.get_settings(args.dev)?;
    let version = project.get_manifest::<VersionManifest>()?;

    let server_directory = directory.join("server");
    if !server_directory.exists() || !server_directory.is_dir() {
        fs::create_dir_all(&server_directory).with_context(|| {
            format!(
                "Could not create server directory \"{}\"",
                server_directory.display()
            )
        })?;
    }

    let server_jar = version
        .versions_directory
        .join(project.project_details.server_jar);

    // prepare_dependencies(
    //     project_data.get_dependencies_directory(),
    //     project_data.get_dependencies_manifest().unwrap(),
    //     project_data.get_metadata(),
    //     server_directory.join("plugins"),
    // )?;

    process_overrides(settings.clone(), server_directory.clone())?;

    run_server(server_directory, server_jar, settings)?.wait()?;

    Ok(())
}

fn prepare_dependencies(
    dependencies_folder: PathBuf,
    dependencies: HashMap<String, String>,
    metadata: ProjectMetadata,
    target_directory: PathBuf,
) -> anyhow::Result<()> {
    if metadata.dependencies.len() != dependencies.keys().len() {
        return Err(anyhow!(
            "Detected dependency changes, make sure to run `chain install` first"
        ));
    }

    for dependency_entry in metadata.dependencies {
        let source = dependencies
            .get(&dependency_entry.name)
            .expect("Detected dependency changes, make sure to run `chain install` first");

        if let Some(path) = dependency_entry.path {
            if &path != source {
                return Err(anyhow!(
                    "Detected dependency changes, make sure to run `chain install` first"
                ));
            }
        }

        if let Some(url) = dependency_entry.download_url {
            if &url != source {
                return Err(anyhow!(
                    "Detected dependency changes, make sure to run `chain install` first"
                ));
            }
        }
    }

    for dependency in dependencies.keys() {
        println!("Preparing dependency \"{}\"...", dependency);
        let dependency_file = Path::new(&dependencies_folder).join(dependency);

        if !dependency_file.exists() {
            return Err(anyhow!(
                "Dependency file \"{}\" was not found, make sure to run `chain install` first",
                dependency
            ));
        }

        fs::create_dir_all(&target_directory)?;
        fs::copy(&dependency_file, target_directory.join(dependency))?;
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
