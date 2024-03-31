use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

use inquire::{Confirm, Text};

const AIKAR_FLAGS: &[&str] = &[
    "-Daikars.new.flags=true",
    "-XX:+UseG1GC",
    "-XX:+ParallelRefProcEnabled",
    "-XX:MaxGCPauseMillis=200",
    "-XX:+UnlockExperimentalVMOptions",
    "-XX:+DisableExplicitGC",
    "-XX:+AlwaysPreTouch",
    "-XX:G1NewSizePercent=30",
    "-XX:G1MaxNewSizePercent=40",
    "-XX:G1HeapRegionSize=8M",
    "-XX:G1ReservePercent=20",
    "-XX:G1HeapWastePercent=5",
    "-XX:G1MixedGCCountTarget=4",
    "-XX:InitiatingHeapOccupancyPercent=15",
    "-XX:G1MixedGCLiveThresholdPercent=90",
    "-XX:G1RSetUpdatingPauseTimePercent=5",
    "-XX:SurvivorRatio=32",
    "-XX:+PerfDisableSharedMem",
    "-XX:MaxTenuringThreshold=1",
];

pub fn generate_template<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let path = path.as_ref();
    log::info!(
        r#"Project files will be generated at {:?}!"#,
        path.display()
    );

    let suggested_server_name: &str = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let server_name = Text::new("What's the name of the server?")
        .with_default(suggested_server_name)
        .prompt()?;

    let use_flags = Confirm::new("Would you like to use Aikar's flags?")
        .with_default(true)
        .prompt()?;

    let server_jar = Text::new("Provide a path or download url for the server jar:")
        .with_placeholder("(Optional)")
        .prompt()?;

    fs::create_dir_all(path)?;
    generate_project_file(path, &server_name, &server_jar, use_flags)?;
    generate_git_files(path)?;

    log::info!("Project files generated at \"{}\"!", path.display());
    Ok(())
}

fn generate_file(contents: &[u8], file_path: PathBuf) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents)
}

fn generate_project_file(
    directory: &Path,
    server_name: &str,
    server_jar: &str,
    _use_aikar_flags: bool,
) -> io::Result<()> {
    let chain = r#"name: {name}

server:
  source: {source}
  brand:
  version:
"#
    .replace("{name}", server_name)
    .replace("{source}", server_jar);
    generate_file(chain.as_bytes(), directory.join("chain.yml"))?;

    // TODO: Generate settings file
    /*let mut settings: ProjectSettings = ProjectSettings {
        jvm_options: vec!["-Dfile.encoding=UTF-8".to_string(), "-Xmx4G".to_string()],
        server_args: vec!["--nogui".to_string()],
        files: Default::default(),
    };
    if use_aikar_flags {
        settings
            .jvm_options
            .extend(AIKAR_FLAGS.iter().map(|&s| s.to_string()));
    }

    let settings = serde_yaml::to_string(&settings)?;*/
    // generate_file(settings.as_bytes(), directory.join("settings.yml"))?;
    Ok(())
}

fn generate_git_files(directory: &Path) -> io::Result<()> {
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
    generate_file(git_ignore.as_bytes(), directory.join(".gitignore"))?;

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
    generate_file(readme.as_bytes(), directory.join("README.md"))?;
    Ok(())
}
