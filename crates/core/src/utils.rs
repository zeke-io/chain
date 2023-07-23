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

pub async fn download_file(url: String, mut path: PathBuf) -> anyhow::Result<()> {
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Could not download file from \"{}\".", &url));
    }

    if path.is_dir() {
        let filename: String = get_filename_from_downloadable_file(&url);
        path = path.join(filename);
    }

    let mut file = File::create(path)?;
    let content = response.bytes().await?;

    file.write_all(&content)?;
    Ok(())
}

pub fn get_filename_from_downloadable_file(url: &str) -> String {
    fn get_filename_from_url(url: &str) -> String {
        let binding = Url::parse(url).unwrap();
        let url_path = binding.path_segments();

        return if let Some(file_name) = url_path.and_then(Iterator::last) {
            file_name.to_owned()
        } else {
            "downloaded_file.msc".to_owned()
        };
    }

    get_filename_from_url(url)
}
