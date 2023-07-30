use crate::project::manifests::{DependenciesManifest, VersionManifest};
use crate::project::settings::ProjectSettings;
use crate::{logger, project};
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

#[allow(unused_variables)]
#[allow(unreachable_code)]
pub fn pack_server<P: AsRef<Path>>(root_directory: P, is_dev: bool) -> anyhow::Result<()> {
    let root_directory = root_directory.as_ref();
    let out_directory = root_directory.join("out");
    let server_directory = out_directory.join("server");

    let project = project::load_project(root_directory)?;
    let project_directory = &project.root_directory;

    let settings = match project.get_settings(is_dev) {
        Ok(settings) => settings,
        Err(_) => {
            logger::warn("No settings file was found, using default values...");
            ProjectSettings::default()
        }
    };
    let version = project
        .get_manifest::<VersionManifest>()
        .context("Version manifest file was not found, make sure to run `chain install` first")?;
    let dependencies = project.get_manifest::<DependenciesManifest>().context(
        "Dependencies manifest file was not found, make sure to run `chain install` first",
    )?;

    logger::info("Preparing server files...");
    fs::create_dir_all(&server_directory)?;

    project::prepare_dependencies(
        dependencies.dependencies,
        project.project_details.dependencies,
        server_directory.join("plugins"),
    )?;

    project::process_files(project_directory, &server_directory, settings.clone())?;

    let server_jar = Path::new(&version.jar_file);
    let server_jar_name: &str = Path::new(&version.jar_file)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("server.jar");
    fs::copy(server_jar, server_directory.join(server_jar_name))
        .context("Could not copy server JAR file")?;

    logger::info("Generating start scripts...");
    generate_start_scripts(
        server_directory.as_path(),
        &settings.java_runtime.clone(),
        server_jar_name,
        settings,
    )?;

    logger::info("Generating ZIP file, this might take a while...");
    create_zip(out_directory.join("server.zip"), server_directory.as_path())?;

    logger::success(&format!(
        "Server ZIP file has been generated at \"{}\"!",
        out_directory.join("server.zip").display()
    ));
    Ok(())
}

fn generate_start_scripts(
    server_directory: &Path,
    jar_path: &str,
    server_jar: &str,
    settings: ProjectSettings,
) -> anyhow::Result<()> {
    fn inner(
        out_path: PathBuf,
        contents: &str,
        jar_path: &str,
        server_jar: &str,
        settings: &ProjectSettings,
    ) -> anyhow::Result<()> {
        let contents = contents
            .replace("{jar_path}", jar_path)
            .replace("{jvm_options}", &settings.jvm_options.join(" "))
            .replace("{server_jar}", server_jar)
            .replace("{server_args}", &settings.server_args.join(" "));

        let mut bash_file = File::create(out_path)?;
        bash_file.write_all(contents.as_bytes())?;
        Ok(())
    }

    let bash_script = r#"#!/bin/bash
# Script generated by Chain

{jar_path} {jvm_options} -jar {server_jar} {server_args}
"#;
    let batch_script = r#"@echo off
:: Script generated by Chain

{jar_path} {jvm_options} -jar {server_jar} {server_args}
"#;

    inner(
        server_directory.join("start.sh"),
        bash_script,
        jar_path,
        server_jar,
        &settings,
    )?;
    inner(
        server_directory.join("start.bat"),
        batch_script,
        jar_path,
        server_jar,
        &settings,
    )?;
    Ok(())
}

fn create_zip(out_directory: PathBuf, server_directory: &Path) -> anyhow::Result<()> {
    let file = File::create(out_directory)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let walker = WalkDir::new(server_directory);
    let it = walker.into_iter().filter_map(|e| e.ok());

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(server_directory)?.to_str().unwrap();

        if path.is_file() {
            zip.start_file(name, options)?;
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}
