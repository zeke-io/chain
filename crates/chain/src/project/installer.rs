use crate::project::manifests::DependencyDetails;
use crate::util;
use anyhow::{anyhow, Context};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) async fn download_server(
    source: &str,
    target_directory: PathBuf,
) -> anyhow::Result<PathBuf> {
    let path: PathBuf;
    fs::create_dir_all(&target_directory)?;

    if util::url::is_url(source) {
        println!(
            "Downloading server JAR file \"{}\"...",
            util::url::get_filename_from_url(source)
        );

        path = util::url::download_file(source.into(), target_directory).await?;
    } else {
        println!("Installing server JAR from \"{}\"...", source);

        let source_path = PathBuf::from(source);
        let file_name = Path::new(&source_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("server.jar");
        let dest_path = Path::new(&target_directory).join(file_name);

        fs::copy(source_path, &dest_path).context("Could not copy server JAR file")?;
        path = dest_path;
    }

    Ok(path)
}

pub(crate) async fn download_plugins(
    dependencies: &HashMap<String, String>,
    target_directory: PathBuf,
) -> anyhow::Result<HashMap<String, DependencyDetails>> {
    fs::create_dir_all(&target_directory)?;

    let mut installed_dependencies: HashMap<String, DependencyDetails> = HashMap::new();
    for (id, source) in dependencies {
        if util::url::is_url(source) {
            println!(
                "Downloading \"{}\" from \"{}\"...",
                util::url::get_filename_from_url(source),
                source
            );

            let path = util::url::download_file(source.clone(), target_directory.clone()).await?;

            installed_dependencies.insert(
                id.clone(),
                DependencyDetails {
                    source: source.clone(),
                    file_path: path.into_os_string().into_string().unwrap(),
                },
            );
            continue;
        } else {
            println!("Installing \"{}\" from \"{}\"...", id, source);
            let source = PathBuf::from(source);
            let fallback_name = format!("{}.jar", id);
            let file_name = Path::new(&source)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&fallback_name);

            let target_directory = target_directory.join(file_name);

            if !source.exists() {
                return Err(anyhow!("The path \"{}\" does not exists", source.display()));
            } else if source.is_dir() {
                return Err(anyhow!("The path \"{}\" is not a file", source.display()));
            }

            fs::copy(&source, &target_directory)?;
            installed_dependencies.insert(
                id.clone(),
                DependencyDetails {
                    source: source.into_os_string().into_string().unwrap(),
                    file_path: target_directory.into_os_string().into_string().unwrap(),
                },
            );
            continue;
        }
    }

    Ok(installed_dependencies)
}
