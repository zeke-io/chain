use common::from_folder;

pub fn pack_server(path: Option<String>) -> anyhow::Result<()> {
    let path = match path {
        Some(path) => path,
        None => "./".to_string()
    };

    let metadata = from_folder(path.as_str())
        .expect("Cannot load metadata file");

    println!("{:?}", metadata);
    Ok(())
}

pub fn unpack_server(path: String) -> anyhow::Result<()> {
    println!("{}", path);
    Ok(())
}
