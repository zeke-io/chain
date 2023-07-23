use crate::utils;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerData {
    pub name: String,
    pub jar: String,
    pub server_directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependencyEntry {
    pub name: String,
    pub download_url: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMetadata {
    pub server: ServerData,
    #[serde(default)]
    pub dependencies: Vec<DependencyEntry>,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<ServerMetadata> {
    let metadata_file =
        utils::append_or_check_file(path, "mcs.yml").context("Could not find \"mcs.yml\" file")?;

    let contents = fs::read_to_string(metadata_file).context("Could not read \"mcs.yml\" file")?;

    let metadata: ServerMetadata =
        serde_yaml::from_str(&contents).context("Failed to parse \"mcs.yml\" file")?;

    Ok(metadata)
}
