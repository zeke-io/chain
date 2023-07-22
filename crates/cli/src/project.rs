use anyhow::Context;
use common::metadata;
use common::metadata::PluginEntry;
use std::fs;
use std::path::PathBuf;

pub fn install() -> anyhow::Result<()> {
    let metadata = metadata::from_folder("./").context("Cannot load metadata file")?;
    let server_directory = match metadata.server.directory {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir()?,
    };

    if let Some(plugins) = metadata.plugins {
        install_plugins(plugins, server_directory)?;
    }

    Ok(())
}

fn install_plugins(plugins: Vec<PluginEntry>, server_directory: PathBuf) -> anyhow::Result<()> {
    let directory = server_directory.join("plugins");
    fs::create_dir_all(&directory)?;

    for plugin in plugins {
        let plugin_path = directory.clone().join(&plugin.name);

        if plugin_path.exists() {
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

            todo!();

            continue;
        }

        if let Some(path) = plugin.path {
            println!("Installing \"{}\" from \"{}\"...", &plugin.name, &path);

            todo!();

            continue;
        }

        println!("Warning! No download url or local path has been provided for plugin \"{}\", skipping...", &plugin.name);
    }

    Ok(())
}
