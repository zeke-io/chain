use common::{from_folder, ServerMetadata};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use termion::{color, style};
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

struct EntryFile {
    pub path: String,
    pub contents: Vec<u8>,
    pub checksum: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManifestFile {
    pub path: String,
    pub checksum: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    pub file: Vec<ManifestFile>,
}

pub fn pack_server(path: Option<String>) -> anyhow::Result<()> {
    let path = match path {
        Some(path) => path,
        None => "./".to_string(),
    };

    let metadata = from_folder(path.as_str()).expect("Cannot load metadata file");

    println!(
        "{}Preparing package...{}",
        color::Fg(color::Green),
        style::Reset
    );
    let files = load_files(path.as_str())?;
    create_package(metadata.clone(), files)?;

    println!(
        "{}{}Package created as \"{}\".{}",
        color::Fg(color::Green),
        style::Bold,
        format!("{}.mscpack", metadata.server.name),
        style::Reset,
    );
    Ok(())
}

pub fn unpack_server(path: String, force_all: bool) -> anyhow::Result<()> {
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;
    println!(
        "{}Unpacking \"{}\"...{}",
        color::Fg(color::Green),
        path,
        style::Reset,
    );

    extract_files(&mut archive, force_all)?;

    println!(
        "{}{}Done!{}",
        color::Fg(color::Green),
        style::Bold,
        style::Reset
    );
    Ok(())
}

fn load_files(path: &str) -> anyhow::Result<Vec<EntryFile>> {
    let mut files: Vec<EntryFile> = Vec::new();

    let path = Path::new(path);
    let mut walker = ignore::WalkBuilder::new(path);
    walker.add_ignore("./.mcsignore");

    for result in walker.build() {
        match result {
            Ok(entry) => {
                if let Some(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path().to_string_lossy().into_owned();
                        let contents = fs::read(&entry.path())?;
                        let checksum = format!("{:x}", md5::compute(&contents));

                        files.push(EntryFile {
                            path,
                            contents,
                            checksum,
                        });
                    }
                }
            }
            Err(err) => {
                eprintln!("ERROR: {}", err);
                exit(1)
            }
        }
    }

    Ok(files)
}

fn create_package(metadata: ServerMetadata, files: Vec<EntryFile>) -> anyhow::Result<()> {
    let file = File::create(format!("{}.mscpack", metadata.server.name))?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut manifest_files = Vec::new();

    for (i, file) in files.iter().enumerate() {
        println!(
            "{}{}{} {}/{} {} Processing file {}\"{}\" {}[{}]{}",
            color::Bg(color::Yellow),
            color::Fg(color::Black),
            style::Bold,
            i + 1,
            files.len(),
            style::Reset,
            color::Fg(color::Cyan),
            file.path,
            color::Fg(color::LightBlack),
            file.checksum,
            style::Reset,
        );

        zip.start_file(&file.path, options)?;
        zip.write_all(&*file.contents)?;
        manifest_files.push(ManifestFile {
            path: file.path.clone(),
            checksum: file.checksum.clone(),
        });
    }

    let manifest = Manifest {
        file: manifest_files,
    };

    let metadata_content = toml::to_string(&manifest).unwrap();
    zip.start_file("manifest.toml", options)?;
    zip.write_all(metadata_content.as_bytes())?;

    zip.finish()?;
    Ok(())
}

fn extract_files(archive: &mut ZipArchive<File>, force_all: bool) -> anyhow::Result<()> {
    let manifest_filename = "manifest.toml";
    let manifest: Manifest;

    if let Some(mut manifest_bytes) = archive.by_name(manifest_filename).ok() {
        let mut manifest_string = String::new();
        manifest_bytes.read_to_string(&mut manifest_string)?;

        manifest = toml::from_str(&manifest_string)?;
    } else {
        panic!("Invalid package file.")
    }

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let output_path = match zip_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if zip_file.name().to_owned() == manifest_filename {
            continue;
        }

        if (*zip_file.name()).ends_with('/') {
            fs::create_dir_all(&output_path)?;
            continue;
        }

        // Buffer
        let mut buffer = Vec::new();
        zip_file.read_to_end(&mut buffer)?;

        // Check if file exists
        let existing_file = Path::new(&output_path);
        let mut overriding: bool = false;
        if existing_file.exists() {
            let manifest_file: Option<&ManifestFile> = manifest
                .file
                .iter()
                .find(|f| PathBuf::from(&f.path).eq(&output_path));

            if !force_all && verify_checksum(&buffer, manifest_file, existing_file)? {
                // println!("Skipping file \"{}\"", output_path.display());
                continue;
            }

            overriding = true;
        }

        println!(
            "{} file \"{}\" {}({} bytes)",
            if overriding {
                format!("{}Overriding", color::Fg(color::Yellow))
            } else {
                format!("{}Extracting", color::Fg(color::Cyan))
            },
            output_path.display(),
            style::Reset,
            zip_file.size()
        );

        // Create parent folders
        if let Some(p) = output_path.parent() {
            if !p.exists() {
                fs::create_dir_all(p)?;
            }
        }

        // Write file
        let mut out_file = File::create(&output_path)?;
        out_file.write_all(&buffer)?;
    }

    Ok(())
}

fn verify_checksum(
    buffer: &Vec<u8>,
    manifest_file: Option<&ManifestFile>,
    existing_file: &Path,
) -> anyhow::Result<bool> {
    let file_bytes = fs::read(existing_file)?;
    let checksum = md5::compute(file_bytes);

    if let Some(manifest) = manifest_file {
        return Ok(manifest.checksum == format!("{:x}", checksum));
    } else {
        println!(
            "{}Warning! The file was not found in the manifest{}",
            color::Fg(color::Yellow),
            style::Reset,
        )
    }

    let new_checksum = md5::compute(buffer);
    Ok(checksum.eq(&new_checksum))
}
