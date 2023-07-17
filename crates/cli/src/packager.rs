use common::{from_folder, ServerMetadata};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

pub struct EntryFile {
    pub path: String,
    pub contents: Vec<u8>,
    pub checksum: String,
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

    let metadata_content = toml::to_string(&metadata).unwrap();
    zip.start_file("mcs.toml", options)?;
    zip.write_all(metadata_content.as_bytes())?;

    for file in files {
        println!("[{}] {}", file.checksum, file.path);
        zip.start_file(file.path, options)?;
        zip.write_all(&*file.contents)?;
    }

    zip.finish()?;
    Ok(())
}

pub fn pack_server(path: Option<String>) -> anyhow::Result<()> {
    let path = match path {
        Some(path) => path,
        None => "./".to_string(),
    };

    let metadata = from_folder(path.as_str()).expect("Cannot load metadata file");

    let files = load_files(path.as_str())?;
    create_package(metadata.clone(), files)?;

    println!(
        "Package created as \"{}\"",
        format!("{}.mscpack", metadata.server.name)
    );
    Ok(())
}

fn extract_files(archive: &mut ZipArchive<File>, force_all: bool) -> anyhow::Result<()> {
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let output_path = match zip_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*zip_file.name()).ends_with('/') {
            fs::create_dir_all(&output_path)?;
            continue;
        }

        // Buffer
        let mut buffer = Vec::new();
        zip_file.read_to_end(&mut buffer)?;

        // Check if file exists
        let existing_file = Path::new(&output_path);
        if existing_file.exists() {
            if !force_all && verify_checksum(&buffer, existing_file)? {
                println!("Skipping file \"{}\"", output_path.display());
                continue;
            }

            println!(
                "Overriding file \"{}\" ({} bytes)",
                output_path.display(),
                zip_file.size()
            );
        } else {
            println!(
                "Extracting file \"{}\" ({} bytes)",
                output_path.display(),
                zip_file.size()
            );
        }

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

fn verify_checksum(buffer: &Vec<u8>, existing_file: &Path) -> anyhow::Result<bool> {
    let file_bytes = fs::read(existing_file)?;
    let checksum = md5::compute(file_bytes);
    let new_checksum = md5::compute(buffer);

    Ok(checksum.eq(&new_checksum))
}

pub fn unpack_server(path: String, force_all: bool) -> anyhow::Result<()> {
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;
    println!("Unpacking \"{}\"...", path);

    extract_files(&mut archive, force_all)?;

    println!("Done!");
    Ok(())
}
