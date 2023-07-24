pub mod manifests;
pub mod settings;
mod installer;
pub mod metadata;

use crate::project::settings::ProjectSettings;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::project::metadata::ProjectMetadata;

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
}

pub async fn install(root_directory: PathBuf, force: bool) -> anyhow::Result<()> {
    let project_data = ProjectData::load(&root_directory, true)?;
    let metadata = project_data.metadata.clone();

    installer::install_server_jar(
        &project_data.get_versions_directory(),
        project_data.get_data_directory(),
        metadata.server_jar.clone(),
    )
    .await?;

    installer::install_plugins(
        &project_data.data_directory,
        &project_data.get_dependencies_directory(),
        metadata.dependencies,
        force,
    )
    .await?;

    Ok(())
}
