mod installer;
pub mod manifests;
pub mod metadata;
pub mod settings;

use crate::project::manifests::{DependenciesManifest, Manifest, VersionManifest};
use crate::project::settings::ProjectSettings;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
