use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct ServerData {
    pub name: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeData {
    pub java_path: String,
    pub server_jar: String,
    pub jvm_options: Vec<String>,
    pub server_args: Vec<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerMetadata {
    pub server: ServerData,
    pub runtime: RuntimeData
}