use common::{from_folder, ServerMetadata};
use std::fs::File;
use std::io::Write;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn pack_server(path: Option<String>) -> anyhow::Result<()> {
    let path = match path {
        Some(path) => path,
        None => "./".to_string(),
    };

    let metadata = from_folder(path.as_str()).expect("Cannot load metadata file");

    create_zip(metadata)?;

    Ok(())
}

pub fn unpack_server(path: String) -> anyhow::Result<()> {
    println!("{}", path);
    Ok(())
}

fn create_zip(metadata: ServerMetadata) -> anyhow::Result<()> {
    let file = File::create(format!("{}.mscpack", metadata.server.name))?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let metadata_content = toml::to_string(&metadata).unwrap();
    zip.start_file("mcs.toml", options)?;
    zip.write_all(metadata_content.as_bytes())?;

    zip.finish()?;
    Ok(())
}
