use anyhow::Context;
use project::manifests::{DependenciesManifest, VersionManifest};
use project::settings::ProjectSettings;
use project::{dependencies, load_project, process_files};
use std::ffi::OsString;
use std::io::ErrorKind;
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
        .context("Version manifest file was not found, make sure to run `crafty install` first")?;

    let dependencies = project
        .get_manifest::<DependenciesManifest>()
        .context("Dependency manifest was not found, make sure to run `crafty install` first")?;

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
        &root_directory.join(".crafty").join("dependencies"),
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
    let java: OsString = match (env::var("JAVA_BIN").ok(), env::var("JAVA_HOME").ok()) {
        (Some(java_bin), _) => PathBuf::from(java_bin).into(),
        (None, Some(java_home)) => PathBuf::from(java_home).join("bin/java").into(),
        (None, None) => "java".into(),
    };
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

    let child = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(mut child) => {
            child
                .wait()
                .await
                .expect("Failed to wait for child process");
            // TODO: Doing CTRL+C will make the server gracefully shutdown but crafty will quit immediately,
            //       so this warning is not printed at all unless if the server is stopped without signals (eg. the `stop` command)
            log::warn!("The server has been stopped!")
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                log::error!("Could not run server because java was not found!");
                log::warn!("Make sure java is set in your PATH environment variable and you can use it directly from your terminal,\nor set the path to the JRE in your .env file using the \"JAVA_BIN\" variable.");
            }
            _ => {
                log::error!(
                    "Could not run server because of an unknown error! ({})",
                    err.kind()
                );
            }
        },
    }

    Ok(())
}
