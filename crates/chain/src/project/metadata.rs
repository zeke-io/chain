use crate::util;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependencyEntry {
    pub name: String,
    pub download_url: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMetadata {
    pub name: String,
    pub server_jar: String,
    pub server_directory: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<DependencyEntry>,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<ProjectMetadata> {
    let metadata_file = util::file::append_or_check_file(path, "chain.yml")
        .context("Could not find \"chain.yml\" file")?;

    let contents =
        fs::read_to_string(metadata_file).context("Could not read \"chain.yml\" file")?;

    let metadata: ProjectMetadata =
        serde_yaml::from_str(&contents).context("Failed to parse \"chain.yml\" file")?;

    Ok(metadata)
}
