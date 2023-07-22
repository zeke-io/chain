use crate::metadata::PluginEntry;
use crate::{metadata, utils};
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn install(root_directory: PathBuf, force: bool) -> anyhow::Result<()> {
    let metadata = metadata::from_path(&root_directory).context("Cannot load metadata file")?;

    #[allow(unused_variables)]
    let server_directory = match metadata.server.server_directory {
        Some(path) => PathBuf::from(path),
        None => root_directory.join("server"),
    };

    let data_folder = root_directory.join(".msc");

    install_server_jar(&data_folder, metadata.runtime.server_jar).await?;

    if let Some(plugins) = metadata.plugins {
        install_plugins(&data_folder, plugins, force).await?;
    }

    Ok(())
}

async fn install_server_jar(directory: &PathBuf, server_source_path: String) -> anyhow::Result<()> {
    let directory = directory.join("versions");
    fs::create_dir_all(&directory)?;

    if utils::is_url(&server_source_path) {
        println!(
            "Downloading server JAR file \"{}\"...",
            server_source_path
        );

        utils::download_file(server_source_path, directory.join("server.jar")).await?;
    } else {
        println!("Installing server JAR from \"{}\"...", server_source_path);
        let source_path = PathBuf::from(server_source_path);

        fs::copy(source_path, directory.join("server.jar")).context("Could not copy server JAR file")?;
    }

    Ok(())
}

async fn install_plugins(
    directory: &PathBuf,
    plugins: Vec<PluginEntry>,
    force: bool,
) -> anyhow::Result<()> {
    let directory = directory.join("plugins");
    fs::create_dir_all(&directory)?;

    for plugin in plugins {
        let plugin_path = directory.clone().join(&plugin.name);

        if !force && plugin_path.exists() {
            if !plugin_path.is_file() {
                println!("Warning! The path \"{}\" for the plugin {} is a directory, please delete it or change the name of the plugin.", plugin_path.display(), &plugin.name)
            }

            continue;
        }

        if let Some(download_url) = plugin.download_url {
            println!(
                "Downloading \"{}\" from \"{}\"...",
                &plugin.name, &download_url
            );

            utils::download_file(download_url, plugin_path).await?;

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

            fs::copy(path, plugin_path).context("Could not copy plugin")?;

            continue;
        }

        println!("Warning! No download url or local path has been provided for plugin \"{}\", skipping...", &plugin.name);
    }

    Ok(())
}
