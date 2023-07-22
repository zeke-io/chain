use anyhow::anyhow;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use url::Url;

pub fn is_url(input: &str) -> bool {
    if let Ok(_) = Url::parse(input) {
        return true;
    }

    false
}

pub fn append_or_check_file<P: AsRef<Path>>(path: P, file_name: &str) -> Option<PathBuf> {
    let path = path.as_ref();

    if path.is_file() {
        if let Some(path_file_name) = path.file_name() {
            if file_name == path_file_name {
                return Some(path.to_path_buf());
            }
        }

        return None;
    }

    let mut path_buf = path.to_path_buf();
    path_buf.push(file_name);

    if !path_buf.exists() {
        return None;
    }

    Some(path_buf)
}

pub async fn download_file(url: String, path: PathBuf) -> anyhow::Result<()> {
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Could not download plugin from \"{}\".", url));
    }

    let mut file = File::create(path)?;
    let content = response.bytes().await?;

    file.write_all(&content)?;
    Ok(())
}
