use crate::project::ProjectData;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionManifest {
    pub jar_file: String,
    pub source: String,
}

impl VersionManifest {
    pub fn get_path(project_data: &ProjectData) -> anyhow::Result<PathBuf> {
        let data_directory = project_data.data_directory.clone();
        let contents = fs::read_to_string(data_directory.join("version.yml"))
            .context("Could not find version info")?;

        let version_data: VersionManifest =
            serde_yaml::from_str(&contents).context("Could not parse version info")?;

        Ok(data_directory.join("versions").join(version_data.jar_file))
    }
}
