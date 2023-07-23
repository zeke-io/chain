use crate::metadata::{DependencyEntry, ServerMetadata};
use crate::{metadata, utils};
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionData {
    pub jar_file: String,
    pub source: String,
}

impl VersionData {
    pub fn get_path(project_data: &ProjectData) -> anyhow::Result<PathBuf> {
        let data_directory = project_data.data_directory.clone();
        let contents = fs::read_to_string(data_directory.join("version.yml"))
            .context("Could not find version info")?;

        let version_data: VersionData =
            serde_yaml::from_str(&contents).context("Could not parse version info")?;

        Ok(data_directory.join("versions").join(version_data.jar_file))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectSettings {
    pub java_runtime: String,
    #[serde(default)]
    pub jvm_options: Vec<String>,
    #[serde(default)]
    pub server_args: Vec<String>,
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

pub struct ProjectData {
    root_directory: PathBuf,
    data_directory: PathBuf,
    metadata: ServerMetadata,
    settings: ProjectSettings,
}

impl ProjectData {
    pub fn load<P: AsRef<Path>>(path: &P, is_dev: bool) -> anyhow::Result<Self> {
        let metadata = metadata::from_path(path)?;
        let settings = load_settings(path, is_dev)?;

        Ok(Self {
            root_directory: path.as_ref().to_path_buf(),
            data_directory: Path::new(path.as_ref()).join(".msc"),
            metadata,
            settings,
        })
    }

    pub fn get_metadata(&self) -> ServerMetadata {
        self.metadata.clone()
    }

    pub fn get_dependencies_manifest(&self) -> anyhow::Result<HashMap<String, String>> {
        let content = fs::read_to_string(&self.data_directory.join("dependencies.yml"))?;
        let manifest: HashMap<String, String> = serde_yaml::from_str(&content)?;

        Ok(manifest)
    }

    pub fn get_server_directory(&self) -> PathBuf {
        let server_directory = match &self.metadata.server.server_directory {
            Some(path) => PathBuf::from(path),
            None => Path::new(&self.root_directory).join("server"),
        };

        server_directory
    }

    pub fn get_data_directory(&self) -> &Path {
        Path::new(&self.data_directory)
    }

    pub fn get_dependencies_directory(&self) -> PathBuf {
        Path::new(&self.data_directory).join("dependencies")
    }

    pub fn get_versions_directory(&self) -> PathBuf {
        Path::new(&self.data_directory).join("versions")
    }

    pub fn get_settings(&self) -> ProjectSettings {
        self.settings.clone()
    }
}

pub fn load_settings<P: AsRef<Path>>(path: P, is_dev: bool) -> anyhow::Result<ProjectSettings> {
    let settings_file_name = if is_dev {
        "settings.dev.yml"
    } else {
        "settings.yml"
    };
    let settings_file = utils::append_or_check_file(path, settings_file_name)
        .context(format!("Could not find \"{}\" file", settings_file_name))?;

    let contents = fs::read_to_string(settings_file)
        .context(format!("Could not read \"{}\" file", settings_file_name))?;

    let settings: ProjectSettings = serde_yaml::from_str(&contents)
        .context(format!("Could not parse \"{}\" file", settings_file_name))?;

    Ok(settings)
}

pub async fn install(root_directory: PathBuf, force: bool) -> anyhow::Result<()> {
    let project_data = ProjectData::load(&root_directory, true)?;
    let metadata = project_data.metadata.clone();

    install_server_jar(
        &project_data.get_versions_directory(),
        project_data.get_data_directory(),
        metadata.server.jar.clone(),
    )
    .await?;

    install_plugins(
        &project_data.data_directory,
        &project_data.get_dependencies_directory(),
        metadata.dependencies,
        force,
    )
    .await?;

    Ok(())
}

async fn install_server_jar(
    directory: &PathBuf,
    data_directory: &Path,
    server_source_path: String,
) -> anyhow::Result<PathBuf> {
    let path: PathBuf;
    fs::create_dir_all(&directory)?;

    if utils::is_url(&server_source_path) {
        println!(
            "Downloading server JAR file \"{}\"...",
            utils::get_filename_from_downloadable_file(&server_source_path)
        );

        path = utils::download_file(server_source_path.clone(), directory.clone()).await?;
    } else {
        println!("Installing server JAR from \"{}\"...", server_source_path);
        let source_path = PathBuf::from(&server_source_path);
        let file_name = Path::new(&source_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("server.jar");
        let dest_path = Path::new(&directory).join(file_name);

        fs::copy(source_path, &dest_path).context("Could not copy server JAR file")?;
        path = dest_path;
    }

    let version_data = VersionData {
        source: server_source_path,
        jar_file: path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
            .into(),
    };

    let version_data_file = serde_yaml::to_string(&version_data)?;
    fs::write(data_directory.join("version.yml"), version_data_file)?;

    Ok(path)
}

async fn install_plugins(
    data_directory: &PathBuf,
    directory: &PathBuf,
    plugins: Vec<DependencyEntry>,
    force: bool,
) -> anyhow::Result<()> {
    fs::create_dir_all(&directory)?;

    for plugin in plugins.clone() {
        let plugin_path = directory.clone().join(&plugin.name);

        if !force && plugin_path.exists() {
            if !plugin_path.is_file() {
                return Err(anyhow!("Warning! The path \"{}\" for the plugin {} is a directory, please delete it or change the name of the plugin.", plugin_path.display(), &plugin.name));
            }

            continue;
        }

        if let Some(download_url) = plugin.download_url {
            println!(
                "Downloading \"{}\" from \"{}\"...",
                &plugin.name, &download_url
            );

            utils::download_file(download_url.clone(), plugin_path).await?;
            continue;
        }

        if let Some(path) = plugin.path {
            println!("Installing \"{}\" from \"{}\"...", &plugin.name, &path);
            let path = PathBuf::from(path);

            if !path.exists() {
                println!(
                    "The path \"{}\" does not exists, skipping plugin...",
                    path.display()
                );
                continue;
            } else if !path.is_file() {
                println!(
                    "The path \"{}\" is not a file, skipping plugin...",
                    path.display()
                );
                continue;
            }

            fs::copy(&path, plugin_path).context("Could not copy plugin")?;
            continue;
        }

        println!("Warning! No download url or local path has been provided for plugin \"{}\", skipping...", &plugin.name);
    }

    let mut dependencies_manifest: HashMap<String, String> = HashMap::new();
    for entry in plugins.clone() {
        let source = if let Some(path) = entry.path {
            path
        } else if let Some(url) = entry.download_url {
            url
        } else {
            return Err(anyhow!(
                "Invalid plugin source for \"{}\", aborting...",
                entry.name
            ));
        };

        dependencies_manifest.insert(entry.name, source);
    }
    fs::write(
        data_directory.join("dependencies.yml"),
        serde_yaml::to_string(&dependencies_manifest).unwrap(),
    )?;

    Ok(())
}
