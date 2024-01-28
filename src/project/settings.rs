use crate::util::logger;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectSettings {
    #[serde(default)]
    pub jvm_options: Vec<String>,
    #[serde(default)]
    pub server_args: Vec<String>,
    #[serde(default)]
    pub files: HashMap<String, String>,
}

pub(crate) fn load_settings<P: AsRef<Path>>(
    path: P,
    is_dev: bool,
) -> anyhow::Result<ProjectSettings> {
    fn inner(path: PathBuf) -> anyhow::Result<ProjectSettings> {
        let settings_file =
            fs::read_to_string(&path).context("Could not find settings file, please create one")?;

        let settings: ProjectSettings = serde_yaml::from_str(&settings_file).context(format!(
            "The settings file at \"{}\" is invalid.",
            path.display()
        ))?;

        Ok(settings)
    }

    let path = path.as_ref();

    if is_dev {
        match inner(path.join("settings.dev.yml")) {
            Ok(settings) => return Ok(settings),
            Err(err) => logger::warn(&format!(
                "{}\nAttempting to load \"settings.yml\" file...",
                err
            )),
        }
    }

    inner(path.join("settings.yml"))
}
