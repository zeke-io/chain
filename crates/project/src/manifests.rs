use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::dependencies::{DependencyFile, DependencyType};

pub trait Manifest {
    type ManifestType;

    fn load_manifest(project_directory: &Path) -> anyhow::Result<Self::ManifestType>;
    fn save_manifest(&self, directory: &Path) -> anyhow::Result<()>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionManifest {
    pub jar_file: String,
    pub source: String,

    #[serde(skip_serializing, skip_deserializing)]
    pub versions_directory: PathBuf,
}

impl VersionManifest {
    pub fn new(source: &str, jar_path: PathBuf) -> Self {
        Self {
            jar_file: jar_path.into_os_string().into_string().unwrap(),
            source: source.to_string(),
            versions_directory: Default::default(),
        }
    }
}

impl Manifest for VersionManifest {
    type ManifestType = VersionManifest;

    fn load_manifest(project_directory: &Path) -> anyhow::Result<Self::ManifestType> {
        let data_directory = project_directory.join(".crafty");
        let contents = fs::read_to_string(data_directory.join("version.yml"))
            .context("Could not find version manifest file")?;

        let mut version_data: VersionManifest =
            serde_yaml::from_str(&contents).context("Could not parse version manifest file")?;
        version_data.versions_directory = data_directory.join("versions");

        Ok(version_data)
    }

    fn save_manifest(&self, directory: &Path) -> anyhow::Result<()> {
        let parsed_manifest = serde_yaml::to_string(&self)?;
        fs::write(directory, parsed_manifest).context("Could not save manifest file")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependencyDetails {
    pub dependency_type: DependencyType,
    pub files: Vec<DependencyFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependenciesManifest(pub(crate) HashMap<String, DependencyDetails>);

impl DependenciesManifest {
    pub fn new(dependencies: HashMap<String, DependencyDetails>) -> Self {
        Self(dependencies)
    }
}

impl Manifest for DependenciesManifest {
    type ManifestType = DependenciesManifest;

    fn load_manifest(project_directory: &Path) -> anyhow::Result<Self::ManifestType> {
        let data_directory = project_directory.join(".crafty");
        let contents = fs::read_to_string(data_directory.join("dependencies.yml"))
            .context("Could not find dependencies manifest file")?;

        let dependencies: DependenciesManifest = serde_yaml::from_str(&contents)
            .context("Could not parse dependencies manifest file")?;

        Ok(dependencies)
    }

    fn save_manifest(&self, directory: &Path) -> anyhow::Result<()> {
        let parsed_manifest = serde_yaml::to_string(&self.0)?;
        fs::write(directory, parsed_manifest).context("Could not save manifest file")
    }
}
