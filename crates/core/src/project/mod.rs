mod installer;
pub mod manifests;
pub mod metadata;
pub mod settings;

use crate::project::manifests::{Manifest, VersionManifest};
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

/*#[deprecated]
pub struct ProjectData {
    root_directory: PathBuf,
    data_directory: PathBuf,
    metadata: ProjectMetadata,
    settings: ProjectSettings,
}

impl ProjectData {
    pub fn load<P: AsRef<Path>>(path: &P, is_dev: bool) -> anyhow::Result<Self> {
        let metadata = metadata::from_path(path)?;
        let settings = settings::load_settings(path, is_dev)?;

        Ok(Self {
            root_directory: path.as_ref().to_path_buf(),
            data_directory: Path::new(path.as_ref()).join(".chain"),
            metadata,
            settings,
        })
    }

    pub fn get_metadata(&self) -> ProjectMetadata {
        self.metadata.clone()
    }

    pub fn get_dependencies_manifest(&self) -> anyhow::Result<HashMap<String, String>> {
        let content = fs::read_to_string(&self.data_directory.join("dependencies.yml"))?;
        let manifest: HashMap<String, String> = serde_yaml::from_str(&content)?;

        Ok(manifest)
    }

    pub fn get_server_directory(&self) -> PathBuf {
        let server_directory = match &self.metadata.server_directory {
            Some(path) => PathBuf::from(path),
            None => Path::new(&self.root_directory).join("server"),
        };

        server_directory
    }

    pub fn get_data_directory(&self) -> &Path {
        Path::new(&self.data_directory)
    }

    pub fn get_dependencies_directory(&self) -> PathBuf {
        Path::new(&self.data_directory).join("dependencies")
    }

    pub fn get_versions_directory(&self) -> PathBuf {
        Path::new(&self.data_directory).join("versions")
    }

    pub fn get_settings(&self) -> ProjectSettings {
        self.settings.clone()
    }
}*/

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

    // installer::install_plugins(
    //     &project_data.data_directory,
    //     &project_data.get_dependencies_directory(),
    //     metadata.dependencies,
    //     force,
    // )
    // .await?;

    Ok(())
}
