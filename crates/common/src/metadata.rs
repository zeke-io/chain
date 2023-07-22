use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerData {
    pub name: String,
    pub directory: Option<String>,
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

pub fn from_path(path: &str) -> Option<ServerMetadata> {
    let contents = fs::read_to_string(path);

    if let Ok(contents) = contents {
        let metadata: ServerMetadata = serde_yaml::from_str(&contents)
            .context("Failed to parse \"mcs.yml\" file")
            .ok()?;
        return Option::from(metadata);
    }

    None
}

pub fn from_folder(path: &str) -> Option<ServerMetadata> {
    let file_path = Path::new(path).join("mcs.yml");
    let contents = fs::read_to_string(file_path);

    if let Ok(contents) = contents {
        let metadata: ServerMetadata = serde_yaml::from_str(&contents)
            .context("Failed to parse \"mcs.yml\" file")
            .ok()?;
        return Option::from(metadata);
    }

    None
}
