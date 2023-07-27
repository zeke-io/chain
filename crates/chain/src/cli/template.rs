use anyhow::Context;
use chain::project::ProjectDetails;
use inquire::Text;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_template(path: PathBuf) -> anyhow::Result<()> {
    let server_name = Text::new("What's the name of the server?").prompt()?;
    let server_jar = Text::new("Provide the path or download url for the server jar:").prompt()?;

    let project = ProjectDetails {
        name: server_name,
        server_jar,
        dependencies: Default::default(),
    };

    generate_project_file(&path, project)?;
    generate_gitignore(&path)?;
    Ok(())
}

fn generate_project_file(directory: &PathBuf, project: ProjectDetails) -> anyhow::Result<()> {
    let parsed = serde_yaml::to_string(&project)?;
    fs::write(directory.join("chain.yml"), parsed).context("Could not create \"chain.yml\" file")
}

fn generate_gitignore(directory: &PathBuf) -> anyhow::Result<()> {
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
