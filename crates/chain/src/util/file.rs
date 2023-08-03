use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

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

pub fn find_up_file(path: &Path, file_name: &str) -> Option<PathBuf> {
    let mut path: PathBuf = path.into();
    let file = Path::new(file_name);

    loop {
        path.push(file);

        if path.is_file() {
            break Some(path);
        }

        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}

/// Hacky way to know if a file is a binary (might not be accurate)
pub fn is_binary<P: AsRef<Path>>(path: P) -> anyhow::Result<bool> {
    const BYTES_TO_CHECK: usize = 256;
    let mut buffer = [0; BYTES_TO_CHECK];
    let mut file = File::open(path)?;

    let num_bytes = file.read(&mut buffer)?;
    for &byte in buffer[..num_bytes].iter() {
        if byte == 0 {
            return Ok(true);
        }
    }

    Ok(false)
}
