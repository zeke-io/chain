use crate::util;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectSettings {
    pub java_runtime: String,
    #[serde(default)]
    pub jvm_options: Vec<String>,
    #[serde(default)]
    pub server_args: Vec<String>,
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

pub(crate) fn load_settings<P: AsRef<Path>>(
    path: P,
    is_dev: bool,
) -> anyhow::Result<ProjectSettings> {
    let settings_file_name = if is_dev {
        "settings.dev.yml"
    } else {
        "settings.yml"
    };
    let settings_file = util::file::append_or_check_file(path, settings_file_name)
        .context(format!("Could not find \"{}\" file", settings_file_name))?;

    let contents = fs::read_to_string(settings_file)
        .context(format!("Could not read \"{}\" file", settings_file_name))?;

    let settings: ProjectSettings = serde_yaml::from_str(&contents)
        .context(format!("Could not parse \"{}\" file", settings_file_name))?;

    Ok(settings)
}
