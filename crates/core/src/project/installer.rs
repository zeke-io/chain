use crate::project::manifests::VersionManifest;
use crate::{util};
use anyhow::{anyhow, Context};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::project::metadata::DependencyEntry;

pub(crate) async fn install_server_jar(
    directory: &PathBuf,
    data_directory: &Path,
    server_source_path: String,
) -> anyhow::Result<PathBuf> {
    let path: PathBuf;
    fs::create_dir_all(&directory)?;

    if util::url::is_url(&server_source_path) {
        println!(
            "Downloading server JAR file \"{}\"...",
            util::url::get_filename_from_url(&server_source_path)
        );

        path = util::url::download_file(server_source_path.clone(), directory.clone()).await?;
    } else {
        println!("Installing server JAR from \"{}\"...", server_source_path);
        let source_path = PathBuf::from(&server_source_path);
        let file_name = Path::new(&source_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("server.jar");
        let dest_path = Path::new(&directory).join(file_name);

        fs::copy(source_path, &dest_path).context("Could not copy server JAR file")?;
        path = dest_path;
    }

    let version_data = VersionManifest {
        source: server_source_path,
        jar_file: path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
            .into(),
    };

    let version_data_file = serde_yaml::to_string(&version_data)?;
    fs::write(data_directory.join("version.yml"), version_data_file)?;

    Ok(path)
}

pub(crate) async fn install_plugins(
    data_directory: &PathBuf,
    directory: &PathBuf,
    plugins: Vec<DependencyEntry>,
    force: bool,
) -> anyhow::Result<()> {
    fs::create_dir_all(&directory)?;

    for plugin in plugins.clone() {
        let plugin_path = directory.clone().join(&plugin.name);

        if !force && plugin_path.exists() {
            if !plugin_path.is_file() {
                return Err(anyhow!("Warning! The path \"{}\" for the plugin {} is a directory, please delete it or change the name of the plugin.", plugin_path.display(), &plugin.name));
            }

            continue;
        }

        if let Some(download_url) = plugin.download_url {
            println!(
                "Downloading \"{}\" from \"{}\"...",
                &plugin.name, &download_url
            );

            util::url::download_file(download_url.clone(), plugin_path).await?;
            continue;
        }

        if let Some(path) = plugin.path {
            println!("Installing \"{}\" from \"{}\"...", &plugin.name, &path);
            let path = PathBuf::from(path);

            if !path.exists() {
                println!(
                    "The path \"{}\" does not exists, skipping plugin...",
                    path.display()
                );
                continue;
            } else if !path.is_file() {
                println!(
                    "The path \"{}\" is not a file, skipping plugin...",
                    path.display()
                );
                continue;
            }

            fs::copy(&path, plugin_path).context("Could not copy plugin")?;
            continue;
        }

        println!("Warning! No download url or local path has been provided for plugin \"{}\", skipping...", &plugin.name);
    }

    let mut dependencies_manifest: HashMap<String, String> = HashMap::new();
    for entry in plugins.clone() {
        let source = if let Some(path) = entry.path {
            path
        } else if let Some(url) = entry.download_url {
            url
        } else {
            return Err(anyhow!(
                "Invalid plugin source for \"{}\", aborting...",
                entry.name
            ));
        };

        dependencies_manifest.insert(entry.name, source);
    }
    fs::write(
        data_directory.join("dependencies.yml"),
        serde_yaml::to_string(&dependencies_manifest).unwrap(),
    )?;

    Ok(())
}
