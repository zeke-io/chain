use serde::{Deserialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerData {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeData {
    pub java_path: String,
    pub server_jar: String,
    pub jvm_options: Vec<String>,
    pub server_args: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerMetadata {
    pub server: ServerData,
    pub runtime: RuntimeData,
}

pub fn from_path(path: &str) -> Option<ServerMetadata> {
    let contents = fs::read_to_string(path);

    if let Ok(contents) = contents {
        let metadata: ServerMetadata = toml::from_str(&contents).unwrap();
        return Option::from(metadata);
    }

    None
}

pub fn from_folder(path: &str) -> Option<ServerMetadata> {
    let file_path = Path::new(path).join("mcs.toml");
    let contents = fs::read_to_string(file_path);

    if let Ok(contents) = contents {
        let metadata: ServerMetadata = toml::from_str(&contents).unwrap();
        return Option::from(metadata);
    }

    None
}
