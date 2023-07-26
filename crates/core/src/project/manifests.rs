use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub trait Manifest {
    type ManifestType;

    fn load_manifest(project_directory: &PathBuf) -> anyhow::Result<Self::ManifestType>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionManifest {
    pub jar_file: String,
    pub source: String,

    #[serde(skip_serializing)]
    pub versions_directory: PathBuf,
}

impl Manifest for VersionManifest {
    type ManifestType = VersionManifest;

    fn load_manifest(project_directory: &PathBuf) -> anyhow::Result<Self::ManifestType> {
        let data_directory = project_directory.join(".chain");
        let contents = fs::read_to_string(data_directory.join("version.yml"))
            .context("Could not find version info")?;

        let mut version_data: VersionManifest =
            serde_yaml::from_str(&contents).context("Could not parse version info")?;
        version_data.versions_directory = data_directory.join("versions");

        Ok(version_data)
    }
}
