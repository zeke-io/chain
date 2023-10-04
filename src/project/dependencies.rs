use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DependencyType {
    Mod,
    Plugin,
    DataPack,
    ResourcePack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependencyFile {
    pub filename: String,
    pub source: String,
    pub hash: String,
}
