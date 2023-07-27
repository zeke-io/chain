use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_template(path: PathBuf) -> anyhow::Result<()> {
    generate_gitignore(&path)?;
    Ok(())
}

fn generate_gitignore(path: &PathBuf) -> anyhow::Result<()> {
    let content = r#"### Chain
.chain/

# Settings
settings.dev.yml

# Server
server/
"#;

    let mut file = File::create(path.join("../../../../.gitignore"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
