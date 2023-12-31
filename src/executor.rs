use std::process::{Command, Output};
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub struct FailureScriptEnv {
    pub exit_code: i32,
    pub unix_time: u64,
    pub interval: u64,
    pub fail_pid: u32,
}

pub fn execute_command(args: &super::cli::Args) -> (Output, u32) {
    let child = if let Some(script_path) = &args.script {
        if !is_executable(script_path) {
            eprintln!("Error: The script '{}' is not executable.", script_path);
            std::process::exit(1);
        }
        Command::new(script_path).spawn().expect("Failed to execute script")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(args.command.join(" "))
            .spawn()
            .expect("Failed to execute command")
    };

    let cpid = child.id();
    let output = child.wait_with_output().expect("Failed to wait on child");

    (output, cpid)
}

pub fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn execute_failure_script(args: &super::cli::Args, envs: FailureScriptEnv) {
    if let Some(script_path) = args.failure_script.as_ref().map(|s| AsRef::<Path>::as_ref(s)) {
        if !is_executable(script_path) {
            eprintln!("Error: The script '{}' is not executable.", script_path.display());
            std::process::exit(1);
        }
        Command::new(script_path)
            .env("HEAT_FAIL_CODE", envs.exit_code.to_string())
            .env("HEAT_FAIL_TIME", envs.unix_time.to_string())
            .env("HEAT_FAIL_INTERVAL", envs.interval.to_string())
            .env("HEAT_FAIL_PID", envs.fail_pid.to_string())
            .spawn()
            .expect("Failed to execute failure.sh");
    } else {
        eprintln!("No failure script provided");
    }
}
