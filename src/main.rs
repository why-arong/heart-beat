mod cli;
mod executor;


use clap::Parser;

use crate::cli::Args;
use crate::executor::execute_command;

use std::thread;
use std::time::{Duration, SystemTime};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use nix::errno::Errno;


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
        let output = execute_command(&args);
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
