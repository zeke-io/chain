use crate::util::logger;
use inquire::Text;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn generate_template(path: &Path) -> anyhow::Result<()> {
    logger::info(&format!(
        r#"Project files will be generated at {:?}!"#,
        path.display()
    ));

    let suggested_server_name: &str = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let server_name = Text::new("What's the name of the server?")
        .with_default(suggested_server_name)
        .prompt()?;

    let server_jar = Text::new("Provide a path or download url for the server jar:")
        .with_placeholder("(Optional)")
        .prompt()?;

    fs::create_dir_all(path)?;
    generate_project_file(path, &server_name, &server_jar)?;
    generate_git_files(path)?;

    logger::success(&format!(
        "Project files generated at \"{}\"!",
        path.display()
    ));
    Ok(())
}

fn generate_file(contents: &[u8], file_path: PathBuf) -> anyhow::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents)?;
    Ok(())
}

fn generate_project_file(
    directory: &Path,
    server_name: &str,
    server_jar: &str,
) -> anyhow::Result<()> {
    let chain = r#"name: {name}

server:
  source: {source}
  brand: 
  version: 
"#
    .replace("{name}", server_name)
    .replace("{source}", server_jar);

    let settings = r#"jvm-options:
  - "-Dfile.encoding=UTF-8"
  - "-Xmx4G"

server-args:
  - "--nogui"
"#
    .replace("{name}", server_name);

    generate_file(chain.as_bytes(), directory.join("chain.yml"))?;
    generate_file(settings.as_bytes(), directory.join("settings.yml"))?;
    Ok(())
}

fn generate_git_files(directory: &Path) -> anyhow::Result<()> {
    let git_ignore = r#"### Chain
.chain/

# Settings
settings.dev.yml

# Server
server/
out/

# Env files
.env*.local
"#;

    let readme = r#"## Install
```bash
chain install
```

## Run the server (dev)
```bash
chain run
```

## Pack the server
```bash
chain pack
```

Powered by [Chain](https://github.com/zeke-io/chain)
"#;

    generate_file(git_ignore.as_bytes(), directory.join(".gitignore"))?;
    generate_file(readme.as_bytes(), directory.join("README.md"))?;
    Ok(())
}
