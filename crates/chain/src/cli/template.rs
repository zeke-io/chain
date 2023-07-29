use chain::logger;
use inquire::{Confirm, Text};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn generate_template<P: AsRef<Path>>(opt_path: Option<P>) -> anyhow::Result<()> {
    let mut path: PathBuf;

    if let Some(some_path) = opt_path {
        path = PathBuf::from(some_path.as_ref());
    } else {
        path = std::env::current_dir()?;
    }

    let suggested_server_name: &str = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    let server_name = Text::new("What's the name of the server?")
        .with_default(suggested_server_name)
        .prompt()?;

    let create_directory = Confirm::new("Create a separate directory?")
        .with_placeholder("Yes|no")
        .prompt()?;

    if create_directory {
        path = path.join(&server_name);
    }

    let server_jar = Text::new("Provide a path or download url for the server jar:").prompt()?;

    fs::create_dir_all(&path)?;

    generate_project_file(path.as_path(), &server_name, &server_jar)?;
    generate_git_files(path.as_path())?;

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
    let template = r#"# Server name
name: {name}
# Server jar
server-jar: {jar}
"#;

    let contents = template
        .replace("{name}", server_name)
        .replace("{jar}", server_jar);

    generate_file(contents.as_bytes(), directory.join("chain.yml"))
}

fn generate_git_files(directory: &Path) -> anyhow::Result<()> {
    let git_ignore = r#"### Chain
.chain/

# Settings
settings.dev.yml

# Server
server/
out/
"#;

    let readme = r#"## Install
```bash
chain install
```

## Run the server (dev)
```bash
chainr --dev
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
