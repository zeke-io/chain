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

/// Detects if a file is most likely a binary based on the presence of null bytes and control characters.
///
/// It reads the first few bytes of the file
/// to analyze its content and checks for the occurrence of null bytes (0x00)
/// and the delete control character (0x7F).
///
/// If any byte that satisfies these conditions is found, the function returns `true`,
/// indicating that the file is likely binary.
///
/// # Arguments
///
/// * `file_path` - The path to the file to be analyzed.
///
/// # Returns
///
/// * `Ok(true)` - If the file is likely binary.
/// * `Ok(false)` - If the file is likely not binary (i.e., it's probably a text file).
/// * `Err(io::Error)` - If there's an error reading the file.
pub fn is_binary<P: AsRef<Path>>(file_path: P) -> anyhow::Result<bool> {
    const BUFFER_SIZE: usize = 256;
    let mut buffer = [0; BUFFER_SIZE];
    let mut file = File::open(file_path)?;

    let bytes_read = file.read(&mut buffer)?;
    let total_bytes = bytes_read.min(BUFFER_SIZE);

    let is_binary = buffer[..total_bytes]
        .iter()
        .any(|&byte| byte == 0 || byte == 0x7F);

    Ok(is_binary)
}
