use crate::util;
use crate::logger;
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) async fn download_server(
    source: &str,
    target_directory: PathBuf,
) -> anyhow::Result<PathBuf> {
    fs::create_dir_all(&target_directory)?;

    if util::url::is_url(source) {
        logger::info(&format!(
            "Downloading server JAR file \"{}\"...",
            util::url::get_filename_from_url(source)
        ));

        Ok(util::url::download_file(source.into(), target_directory).await?)
    } else {
        logger::info(&format!("Installing server JAR from \"{}\"...", source));

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
