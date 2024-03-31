use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::installer;
use crate::manifests::{DependenciesManifest, DependencyDetails, Manifest};

// Workaround for https://github.com/serde-rs/serde/issues/368
pub const fn default_bool<const V: bool>() -> bool {
    V
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Dependency {
    pub source: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type", default)]
    pub dependency_type: DependencyType,
    #[serde(default = "default_bool::<false>")]
    pub required: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Mod,
    #[default]
    Plugin,
    DataPack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependencyFile {
    pub filename: String,
    pub source: String,
    pub hash: String,
}

pub async fn install_dependencies(
    dependencies: &HashMap<String, Dependency>,
    root_directory: &Path,
) -> anyhow::Result<()> {
    let mut installed_dependencies: HashMap<String, DependencyDetails> = HashMap::new();

    for (id, dependency) in dependencies {
        let mut files = vec![];

        if let Some(source) = &dependency.source {
            let file = match install_from_source(
                id,
                source,
                root_directory.join(".chain").join("dependencies"),
            )
            .await
            {
                Ok(file) => file,
                Err(err) => {
                    if dependency.required {
                        return Err(err);
                    } else {
                        log::warn!("Could not install {} ({}), skipping...", id, err);
                        continue;
                    }
                }
            };
            files = vec![file];
        } else if let Some(_version) = &dependency.version {
            // TODO: Implement
            // download_version(id, version)?;
            files = vec![];
        }

        installed_dependencies.insert(
            id.clone(),
            DependencyDetails {
                dependency_type: DependencyType::Plugin,
                files,
            },
        );
    }

    let manifest = DependenciesManifest::new(installed_dependencies);
    manifest.save_manifest(&root_directory.join(".chain").join("dependencies.yml"))?;
    Ok(())
}

async fn install_from_source(
    id: &str,
    source: &str,
    destination: PathBuf,
) -> anyhow::Result<DependencyFile> {
    let file_path = if utils::is_url(source) {
        let filename = utils::get_filename_from_url(source);
        log::info!(
            "Installing \"{}\" ({}) from \"{}\"...",
            id,
            filename,
            source
        );

        let destination_file = destination.join(filename);
        fs::create_dir_all(destination_file.parent().unwrap())?;
        installer::download_file(source.into(), destination_file).await?
    } else {
        log::info!("Installing \"{}\" from \"{}\"...", id, source);
        let source = PathBuf::from(source);
        let fallback_name = format!("{}.jar", id);
        let filename = Path::new(&source)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&fallback_name);
        let target_directory = destination.join(filename);

        if !source.exists() {
            return Err(anyhow!("The path \"{}\" does not exist", source.display()));
        } else if source.is_dir() {
            return Err(anyhow!("The path \"{}\" is not a file", source.display()));
        }

        fs::copy(&source, &target_directory)?;
        target_directory
    };

    let filename = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap()
        .to_string();

    Ok(DependencyFile {
        filename,
        source: source.into(),
        hash: "".to_string(),
    })
}

pub fn prepare_server_dependencies(
    dependencies: DependenciesManifest,
    dependencies_directory: &Path,
    server_directory: &Path,
) -> anyhow::Result<()> {
    for (id, dependency) in dependencies.0 {
        log::info!("Preparing dependency {}...", &id);
        let dependency_files = dependency.files;
        let destination_path = match dependency.dependency_type {
            DependencyType::Mod => server_directory.join("mods"),
            DependencyType::Plugin => server_directory.join("plugins"),
            _ => {
                return Err(anyhow!(
                    "Dependency type {:?} for {} is not yet supported!",
                    dependency.dependency_type,
                    id
                ));
            }
        };

        for file in dependency_files {
            let file_path = dependencies_directory.join(file.filename);
            if !file_path.exists() {
                return Err(anyhow!(
                    "Dependency \"{}\" was not found, make sure to run `chain install` first",
                    id
                ));
            }

            fs::create_dir_all(&destination_path)?;
            fs::copy(
                &file_path,
                &destination_path.join(file_path.file_name().unwrap()),
            )?;
        }
    }

    Ok(())
}
