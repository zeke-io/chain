use crate::project::manifests::{DependenciesManifest, DependencyDetails, Manifest};
use crate::project::Dependency;
use crate::util;
use crate::util::logger;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

pub async fn install_dependencies(
    dependencies: &HashMap<String, Dependency>,
    root_directory: &Path,
) -> anyhow::Result<()> {
    let mut installed_dependencies: HashMap<String, DependencyDetails> = HashMap::new();

    for (id, dependency) in dependencies {
        let mut files = vec![];

        if let Some(source) = &dependency.source {
            let file = install_from_source(
                &id,
                &source,
                root_directory.join(".chain").join("dependencies"),
            )
            .await?;
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
    if util::url::is_url(source) {
        logger::info(&format!(
            "Installing \"{}\" from \"{}\"...",
            util::url::get_filename_from_url(source),
            source
        ));

        util::url::download_file(source.into(), destination.to_path_buf()).await?;
    } else {
        logger::info(&format!("Installing \"{}\" from \"{}\"...", id, source));
        let source = PathBuf::from(source);
        let fallback_name = format!("{}.jar", id);
        let file_name = Path::new(&source)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&fallback_name);
        let target_directory = destination.join(file_name);

        if !source.exists() {
            return Err(anyhow!("The path \"{}\" does not exist", source.display()));
        } else if source.is_dir() {
            return Err(anyhow!("The path \"{}\" is not a file", source.display()));
        }

        fs::copy(&source, &target_directory)?;
    }

    Ok(DependencyFile {
        filename: "".to_string(),
        source: source.into(),
        hash: "".to_string(),
    })
}
