use anyhow::{anyhow, Context};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) async fn download_server(
    source: &str,
    target_directory: PathBuf,
) -> anyhow::Result<PathBuf> {
    fs::create_dir_all(&target_directory)?;

    if utils::is_url(source) {
        log::info!(
            "Downloading server JAR file \"{}\"...",
            utils::get_filename_from_url(source)
        );

        Ok(download_file(source.into(), target_directory).await?)
    } else {
        log::info!("Installing server JAR from \"{}\"...", source);

        let source_path = PathBuf::from(source);
        let file_name = Path::new(&source_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("server.jar");
        let dest_path = Path::new(&target_directory).join(file_name);

        fs::copy(source_path, &dest_path).context("Could not copy server JAR file")?;
        Ok(dest_path)
    }
}

pub async fn download_file(url: String, mut destination: PathBuf) -> anyhow::Result<PathBuf> {
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Could not download file from \"{}\".", &url));
    }

    if destination.is_dir() {
        let filename: String = utils::get_filename_from_url(&url);
        destination = destination.join(filename);
    }

    let mut file = File::create(&destination)?;
    let content = response.bytes().await?;

    file.write_all(&content)?;
    Ok(destination)
}
