use crate::utils;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerData {
    pub name: String,
    pub server_directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeData {
    pub server_jar: String,
    pub java_path: Option<String>,
    pub jvm_options: Option<Vec<String>>,
    pub server_args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginEntry {
    pub name: String,
    pub download_url: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMetadata {
    pub server: ServerData,
    pub runtime: RuntimeData,
    pub plugins: Option<Vec<PluginEntry>>,
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Option<ServerMetadata> {
    let metadata_file = utils::append_or_check_file(path, "mcs.yml")
        .context("Could not find \"mcs.yml\" file")
        .ok()?;
    let contents = fs::read_to_string(metadata_file);

    if let Ok(contents) = contents {
        let metadata: ServerMetadata = serde_yaml::from_str(&contents)
            .context("Failed to parse \"mcs.yml\" file")
            .ok()?;
        return Option::from(metadata);
    }

    None
}
