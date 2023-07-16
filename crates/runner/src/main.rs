use std::process::{Child, Command, exit, Stdio};
use common::{from_path, ServerMetadata};

fn main() -> anyhow::Result<()> {
    let metadata = from_path("./mcs.toml")
        .expect("Cannot load metadata file");

    run_server(metadata).wait()?;

    Ok(())
}

fn run_server(metadata: ServerMetadata) -> Child {
    let mut command = Command::new(&metadata.runtime.java_path);
    command.current_dir("./");

    for arg in metadata.runtime.jvm_options {
        command.arg(arg);
    }

    command.arg("-jar");
    command.arg(metadata.runtime.server_jar);

    for arg in metadata.runtime.server_args {
        command.arg(arg);
    }

    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
}
