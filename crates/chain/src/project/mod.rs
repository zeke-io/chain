mod installer;
pub mod manifests;
pub mod packager;
pub mod settings;

use crate::project::manifests::{
    DependenciesManifest, DependencyDetails, Manifest, VersionManifest,
};
use crate::project::settings::ProjectSettings;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectDetails {
    pub name: String,
    pub server_jar: String,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

pub struct Project {
    pub root_directory: PathBuf,
    pub project_details: ProjectDetails,
}

impl Project {
    pub(crate) fn new(directory: &Path, details: ProjectDetails) -> Self {
        Self {
            root_directory: directory.to_path_buf(),
            project_details: details,
        }
    }

    pub fn get_settings(&self, is_dev: bool) -> anyhow::Result<ProjectSettings> {
        settings::load_settings(&self.root_directory, is_dev)
    }

    pub fn get_manifest<T: Manifest>(&self) -> anyhow::Result<T::ManifestType> {
        let manifest = T::load_manifest(&self.root_directory).unwrap();
        Ok(manifest)
    }
}

pub fn load_project<P: AsRef<Path>>(path: P) -> anyhow::Result<Project> {
    let path = path.as_ref();

    let details_file = fs::read_to_string(path.join("chain.yml"))
        .context("Could not find \"chain.yml\" file, please create one")?;
    let details: ProjectDetails = serde_yaml::from_str(&details_file)
        .with_context(|| "The project file \"chain.yml\" is invalid.")?;

    let project = Project::new(path, details);

    Ok(project)
}

pub async fn install(root_directory: PathBuf, _force: bool) -> anyhow::Result<()> {
    let project = load_project(root_directory)?;

    let server_jar_path = installer::download_server(
        &project.project_details.server_jar,
        project.root_directory.join(".chain").join("versions"),
    )
    .await?;

    let version_manifest =
        VersionManifest::new(&project.project_details.server_jar, server_jar_path);
    version_manifest.save_manifest(&project.root_directory.join(".chain").join("version.yml"))?;

    let dependencies = installer::download_plugins(
        &project.project_details.dependencies,
        project.root_directory.join(".chain").join("dependencies"),
    )
    .await?;

    let dependencies_manifest = DependenciesManifest::new(dependencies);
    dependencies_manifest.save_manifest(
        &project
            .root_directory
            .join(".chain")
            .join("dependencies.yml"),
    )?;

    Ok(())
}

pub fn prepare_dependencies(
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

pub fn process_files<P: AsRef<Path>>(
    settings: ProjectSettings,
    server_directory: P,
) -> anyhow::Result<()> {
    let server_directory = server_directory.as_ref();

    for (target_path, source_path) in settings.files {
        let source_path = Path::new(&source_path);
        let target_path = server_directory.join(target_path);

        if !source_path.exists() {
            return Err(anyhow!(
                "Source path \"{}\" does not exists",
                source_path.display()
            ));
        }

        if source_path.is_file() {
            fs::create_dir_all(&target_path.parent().unwrap()).with_context(|| {
                format!("Could not create folders \"{}\"", target_path.display())
            })?;

            fs::copy(&source_path, &target_path).with_context(|| {
                format!(
                    "Could not copy file \"{}\" to \"{}\"",
                    source_path.display(),
                    target_path.display()
                )
            })?;
        } else {
            if !target_path.exists() {
                fs::create_dir_all(&target_path)?;
            }

            for entry in WalkDir::new(source_path).into_iter().filter_map(Result::ok) {
                let source = entry.path();
                let relative_path = source.strip_prefix(source_path)?;
                let destination = target_path.join(relative_path);

                if source.is_file() {
                    fs::copy(&source, &destination).context(format!(
                        "Could not copy file \"{}\" to \"{}\"",
                        source.display(),
                        destination.display()
                    ))?;
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
