use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{env, fs};

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use walkdir::WalkDir;

use crate::dependencies::Dependency;
use crate::manifests::{DependenciesManifest, Manifest, VersionManifest};
use crate::settings::ProjectSettings;

pub mod dependencies;
mod installer;
pub mod manifests;
pub mod packager;
pub mod settings;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub source: String,
    pub brand: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectMetadata {
    pub name: String,
    pub server: Server,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

pub struct Project {
    pub root_directory: PathBuf,
    pub project_details: ProjectMetadata,
}

impl Project {
    pub(crate) fn new(directory: &Path, details: ProjectMetadata) -> Self {
        Self {
            root_directory: directory.to_path_buf(),
            project_details: details,
        }
    }

    pub fn get_settings(&self, profile_name: Option<String>) -> anyhow::Result<ProjectSettings> {
        settings::load_settings(&self.root_directory, profile_name)
    }

    pub fn get_manifest<T: Manifest>(&self) -> anyhow::Result<T::ManifestType> {
        T::load_manifest(&self.root_directory)
    }
}

pub fn load_project<P: AsRef<Path>>(path: P) -> anyhow::Result<Project> {
    let path = path.as_ref();
    dotenv_flow::dotenv_flow().ok();

    let chain_file = utils::find_up_file(path, "chain.yml")
        .context("Could not find \"chain.yml\" file, please create one")?;
    let path = chain_file.parent().unwrap();

    let details_file = fs::read_to_string(&chain_file)?;
    let details: ProjectMetadata = serde_yaml::from_str(&details_file)
        .with_context(|| "The file \"chain.yml\" is invalid.")?;

    let project = Project::new(path, details);

    Ok(project)
}

pub async fn install(root_directory: PathBuf, _force: bool) -> anyhow::Result<()> {
    let project = load_project(root_directory)?;
    let server = &project.project_details.server;

    let server_jar_path = installer::download_server(
        &server.source,
        project.root_directory.join(".chain").join("versions"),
    )
    .await?;

    let version_manifest = VersionManifest::new(&server.source, server_jar_path);
    version_manifest.save_manifest(&project.root_directory.join(".chain").join("version.yml"))?;

    dependencies::install_dependencies(
        &project.project_details.dependencies,
        &project.root_directory,
    )
    .await?;

    Ok(())
}

pub async fn add_dependency(_directory: PathBuf, _dependency_id: String) -> anyhow::Result<()> {
    todo!()
}

pub fn process_files<P: AsRef<Path>>(
    root_directory: &Path,
    server_directory: P,
    settings: ProjectSettings,
) -> anyhow::Result<()> {
    fn inner_file_processor(source_path: &PathBuf, target_path: &PathBuf) -> anyhow::Result<()> {
        // If the file is (most likely) a binary, just copy it
        if utils::is_binary(source_path)? {
            fs::copy(source_path, target_path).with_context(|| {
                format!(
                    "Could not copy file \"{}\" to \"{}\"",
                    source_path.display(),
                    target_path.display()
                )
            })?;
            return Ok(());
        }

        let input = fs::read_to_string(source_path)?;
        let reg_exp = regex::Regex::new(r"\$(CHAIN_[A-Z_0-9]*)")?;
        let output = reg_exp.replace_all(&input, |caps: &regex::Captures<'_>| {
            let var_name = &caps[1];
            match env::var(var_name) {
                Ok(value) => value,
                Err(_) => {
                    log::warn!(
                        "Could not find environment variable \"{}\", replacing it with an empty value.",
                        var_name
                    );

                    "".into()
                }
            }
        });

        fs::write(target_path, output.as_bytes())?;
        Ok(())
    }

    let server_directory = server_directory.as_ref();

    for (target_path, source_path) in &settings.files {
        let source_path = root_directory.join(source_path);
        let target_path = server_directory.join(target_path);

        if !source_path.exists() {
            return Err(anyhow!(
                "Source path \"{}\" does not exists",
                source_path.display(),
            ));
        }

        if source_path.is_file() {
            fs::create_dir_all(target_path.parent().unwrap()).with_context(|| {
                format!("Could not create folders \"{}\"", target_path.display())
            })?;

            inner_file_processor(&source_path, &target_path)?;
        } else {
            if !target_path.exists() {
                fs::create_dir_all(&target_path)?;
            }

            for entry in WalkDir::new(&source_path)
                .into_iter()
                .filter_map(Result::ok)
            {
                let source = entry.path();
                let relative_path = source.strip_prefix(&source_path)?;
                let destination = target_path.join(relative_path);

                if source.is_file() {
                    inner_file_processor(&source.to_path_buf(), &destination)?;
                } else {
                    fs::create_dir_all(&destination).context(format!(
                        "Could not create directory \"{}\"",
                        destination.display()
                    ))?;
                }
            }
        }
    }

    Ok(())
}

/// Prepares the server files and runs the server jar
pub async fn run(
    root_directory: PathBuf,
    profile_name: Option<String>,
    no_setup: bool,
) -> anyhow::Result<()> {
    let project = load_project(root_directory)?;
    let project_directory = &project.root_directory;

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

    let server_directory = project_directory.join("server");
    if !server_directory.exists() || !server_directory.is_dir() {
        fs::create_dir_all(&server_directory).with_context(|| {
            format!(
                "Could not create server directory at \"{}\"",
                server_directory.display()
            )
        })?;
    }

    if !no_setup {
        dependencies::prepare_server_dependencies(
            dependencies,
            &project_directory.join(".chain").join("dependencies"),
            &server_directory,
        )?;

        process_files(
            project_directory,
            server_directory.clone(),
            settings.clone(),
        )?;
    } else {
        log::warn!("Skipping setup, this is only recommended when running the server for the first time...");
    }

    let server_jar = PathBuf::from(version.jar_file);

    log::info!("Running server...");

    let java_path = env::var("JAVA_BIN_PATH").unwrap_or_else(|_| "java".into());
    let mut command = Command::new(java_path);
    command.current_dir(server_directory);

    for arg in settings.jvm_options {
        command.arg(arg);
    }

    command.arg("-jar");
    command.arg(server_jar);

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
