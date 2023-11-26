use clap::Parser;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use nix::errno::Errno;


/// Simple heartbeat command-line app
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Interval in seconds between checks
    #[clap(short, long, value_parser)]
    interval: u64,

    /// Shell script to execute
    #[clap(short = 's', long, value_parser)]
    script: Option<String>,

    /// The command to execute
    #[clap(value_parser, trailing_var_arg = true)]
    command: Vec<String>,

    #[clap(long, value_parser)]
    pid: Option<u32>,

    #[clap(long, value_parser)]
    signal: Option<String>,
}

fn send_signal(pid: u32, signal_name: &str) -> nix::Result<()> {
    let signal = match signal_name {
        "USR1" => Signal::SIGUSR1,
        "HUP" => Signal::SIGHUP,
        _ => return Err(nix::Error::from(Errno::EINVAL)),
    };
    kill(Pid::from_raw(pid as i32), signal)
}

fn main() {
    let args = Args::parse();

    let interval = args.interval;

    loop {
        let start = SystemTime::now();
        let output = if let Some(script_path) = &args.script {
            // Check if the script is executable
            if !is_executable(script_path) {
                eprintln!("Error: The script '{}' is not executable.", script_path);
                return;
            }
            // Execute the script directly
            Command::new(script_path)
                .output()
                .expect("Failed to execute script")
        } else {
            // Execute the command
            Command::new("sh")
                .arg("-c")
                .arg(args.command.join(" "))
                .output()
                .expect("Failed to execute command")
        };
        if !output.status.success() && args.pid.is_some() {
            let pid = args.pid.unwrap();
            let signal = args.signal.as_deref().unwrap_or("HUP");
            if let Err(e) = send_signal(pid, signal) {
                eprintln!("Failed to send signal: {}", e);
                return;
            }
        }
        let status = match output.status.code() {
            Some(0) => String::from("OK"),
            Some(code) => format!("Failed: Exit Code {}", code),
            None => String::from("Failed: Unknown Error"),
        };

        println!(
            "{}: {}",
            start.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            status
        );

        thread::sleep(Duration::from_secs(interval));
    }
}



fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}