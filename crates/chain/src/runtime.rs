use anyhow::Context;
use project::manifests::{DependenciesManifest, VersionManifest};
use project::settings::ProjectSettings;
use project::{dependencies, load_project, process_files};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{env, fs};
use tokio::process::Command;

pub async fn run_project(
    root_directory: PathBuf,
    profile_name: Option<String>,
    no_setup: bool,
) -> anyhow::Result<()> {
    let project = load_project(&root_directory)?;
    let settings = project.get_settings(profile_name).unwrap_or_else(|_| {
        log::warn!("No settings file was found, using default values...");
        ProjectSettings::default()
    });

    let version = project
        .get_manifest::<VersionManifest>()
        .context("Version manifest file was not found, make sure to run `chain install` first")?;

    let dependencies = project
        .get_manifest::<DependenciesManifest>()
        .context("Dependency manifest was not found, make sure to run `chain install` first")?;

    let server_directory = root_directory.join("server");
    fs::create_dir_all(&server_directory)?;

    if no_setup {
        log::warn!("Skipping setup, this is only recommended when running the server for the first time...");
    } else {
        prepare_files(&root_directory, &server_directory, dependencies, &settings)?;
    }
    run_server(&server_directory, version.jar_file, settings).await
}

fn prepare_files(
    root_directory: &Path,
    server_directory: &Path,
    dependencies: DependenciesManifest,
    settings: &ProjectSettings,
) -> anyhow::Result<()> {
    dependencies::prepare_server_dependencies(
        dependencies,
        &root_directory.join(".chain").join("dependencies"),
        &server_directory,
    )?;

    // TODO: Refactor
    process_files(root_directory, server_directory, settings.clone())
}

async fn run_server<T: AsRef<Path>, U: AsRef<Path>>(
    server_directory: T,
    server_jar: U,
    settings: ProjectSettings,
) -> anyhow::Result<()> {
    log::info!("Running server...");
    let java = env::var("JAVA_BIN").unwrap_or_else(|_| "java".into());
    let mut command = Command::new(java);
    command.kill_on_drop(true);
    command.current_dir(server_directory.as_ref());

    for arg in settings.jvm_options {
        command.arg(arg);
    }

    command.arg("-jar");
    command.arg(server_jar.as_ref());

    for arg in settings.server_args {
        command.arg(arg);
    }

    let mut child = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let _ = tokio::signal::ctrl_c().await;
    child.kill().await.expect("Failed to kill child process");
    child
        .wait()
        .await
        .expect("Failed to wait for child process");

    Ok(())
}
