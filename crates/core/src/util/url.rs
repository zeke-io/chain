use anyhow::anyhow;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

pub fn is_url(input: &str) -> bool {
    if let Ok(_) = Url::parse(input) {
        return true;
    }

    false
}

pub async fn download_file(url: String, mut destination: PathBuf) -> anyhow::Result<PathBuf> {
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(anyhow!("Could not download file from \"{}\".", &url));
    }

    if destination.is_dir() {
        let filename: String = get_filename_from_url(&url);
        destination = destination.join(filename);
    }

    let mut file = File::create(&destination)?;
    let content = response.bytes().await?;

    file.write_all(&content)?;
    Ok(destination)
}

pub fn get_filename_from_url(url: &str) -> String {
    fn inner(url: &str) -> String {
        let binding = Url::parse(url).unwrap();
        let url_path = binding.path_segments();

        return if let Some(file_name) = url_path.and_then(Iterator::last) {
            file_name.to_owned()
        } else {
            "file.chaindf".to_owned()
        };
    }

    inner(url)
}
