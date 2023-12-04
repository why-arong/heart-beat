use std::process::{Command, Output};
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn execute_command(args: &super::cli::Args) -> Output {
    if let Some(script_path) = &args.script {
        if !is_executable(script_path) {
            eprintln!("Error: The script '{}' is not executable.", script_path);
            std::process::exit(1);
        }
        Command::new(script_path).output().expect("Failed to execute script")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(args.command.join(" "))
            .output()
            .expect("Failed to execute command")
    }
}

pub fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn execute_failure_script(script_path: &String) {
    if Path::new(script_path).exists() {
        let _ = Command::new(script_path).status();
    } else {
        eprintln!("Failure script not found: {}", script_path);
    }
}
