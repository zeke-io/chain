use anyhow::Context;
use chain::project::ProjectDetails;
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

    let project = ProjectDetails {
        name: server_name,
        server_jar,
        dependencies: Default::default(),
    };

    generate_project_file(path.as_path(), project)?;
    generate_gitignore(path.as_path())?;
    Ok(())
}

fn generate_project_file(directory: &Path, project: ProjectDetails) -> anyhow::Result<()> {
    let parsed = serde_yaml::to_string(&project)?;
    fs::write(directory.join("chain.yml"), parsed).context("Could not create \"chain.yml\" file")
}

fn generate_gitignore(directory: &Path) -> anyhow::Result<()> {
    let content = r#"### Chain
.chain/

# Settings
settings.dev.yml

# Server
server/
"#;

    let mut file = File::create(directory.join(".gitignore"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
