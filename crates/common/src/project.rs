use crate::metadata::PluginEntry;
use crate::{metadata, utils};
use anyhow::Context;
use std::fs;
use std::path::PathBuf;

pub async fn install(force: bool) -> anyhow::Result<()> {
    let metadata = metadata::from_path("./").context("Cannot load metadata file")?;
    let server_directory = match metadata.server.server_directory {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir()?,
    };

    if let Some(plugins) = metadata.plugins {
        install_plugins(plugins, server_directory, force).await?;
    }

    Ok(())
}

async fn install_plugins(
    plugins: Vec<PluginEntry>,
    server_directory: PathBuf,
    force: bool,
) -> anyhow::Result<()> {
    let directory = server_directory.join("plugins");
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
