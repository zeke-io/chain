use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentValue {
    pub dev: String,
    pub prod: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Override {
    pub source: EnvironmentValue,
}

pub fn from_folder(path: &str) -> Option<HashMap<String, Override>> {
    let file_path = Path::new(path).join("overrides.yml");
    let contents = fs::read_to_string(file_path);

    if let Ok(contents) = contents {
        let overrides: HashMap<String, Override> = serde_yaml::from_str(&contents)
            .context("Failed to parse \"overrides.yml\" file")
            .ok()?;
        return Option::from(overrides);
    }

    None
}
