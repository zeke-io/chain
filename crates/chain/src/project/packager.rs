use crate::project;
use crate::project::manifests::{DependenciesManifest, VersionManifest};
use crate::project::settings::ProjectSettings;
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use termion::{color, style};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

#[allow(unused_variables)]
#[allow(unreachable_code)]
pub fn pack_server<P: AsRef<Path>>(
    root_directory: P,
    is_dev: bool,
) -> anyhow::Result<()> {
    let root_directory = root_directory.as_ref();
    let out_directory = root_directory.join("out");
    let server_directory = out_directory.join("server");

    let project = project::load_project(root_directory)?;
    let settings = match project.get_settings(is_dev) {
        Ok(settings) => settings,
        Err(_) => {
            println!(
                "{}No settings file was provided, using default values...{}",
                color::Fg(color::Yellow),
                style::Reset
            );
            ProjectSettings::default()
        }
    };
    let version = project
        .get_manifest::<VersionManifest>()
        .context("Version manifest file was not found, make sure to run `chain install` first")?;
    let dependencies = project.get_manifest::<DependenciesManifest>().context(
        "Dependencies manifest file was not found, make sure to run `chain install` first",
    )?;

    println!(
        "{}Preparing server files...{}",
        color::Fg(color::Blue),
        style::Reset,
    );

    fs::create_dir_all(&server_directory)?;

    project::prepare_dependencies(
        dependencies.dependencies,
        project.project_details.dependencies,
        server_directory.join("plugins"),
    )?;

    project::process_overrides(settings.clone(), &server_directory)?;

    println!(
        "{}Generating start scripts...{}",
        color::Fg(color::Blue),
        style::Reset,
    );
    let server_jar: &str = Path::new(&version.jar_file)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap();
    generate_start_scripts(
        server_directory.as_path(),
        server_jar,
        settings
    )?;

    println!(
        "{}Generating ZIP file...{}",
        color::Fg(color::Blue),
        style::Reset,
    );
    create_zip(out_directory.join("server.zip"), server_directory.as_path())?;

    println!(
        "{}Server ZIP file has been generated at \"{}\"!{}",
        color::Fg(color::Green),
        out_directory.join("server.zip").display(),
        style::Reset,
    );
    Ok(())
}

fn generate_start_scripts(server_directory: &Path, server_jar: &str, settings: ProjectSettings) -> anyhow::Result<()> {
    fn inner(out_path: PathBuf, contents: &str, server_jar: &str, settings: &ProjectSettings) -> anyhow::Result<()> {
        let contents = contents
            .replace("{jvm_options}", &settings.jvm_options.join(" "))
            .replace("{server_jar}", server_jar)
            .replace("{server_args}", &settings.server_args.join(" "));

        let mut bash_file = File::create(out_path)?;
        bash_file.write_all(contents.as_bytes())?;
        Ok(())
    }

    let bash_script = r#"#!/bin/bash
# Script generated by Chain

java {jvm_options} -jar {server_jar} {server_args}
"#;
    let batch_script = r#"@echo off
:: Script generated by Chain

java {jvm_options} -jar {server_jar} {server_args}
"#;

    inner(server_directory.join("start.sh"), bash_script, server_jar, &settings)?;
    inner(server_directory.join("start.bat"), batch_script, server_jar, &settings)?;
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
