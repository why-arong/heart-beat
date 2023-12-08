mod cli;
mod executor;


use clap::Parser;

use crate::cli::Args;
use crate::executor::{execute_command, execute_failure_script, FailureScriptEnv};
use libc;

use nix::sys::signal::{self, Signal};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH, Instant};
// use nix::sys::signal::{Signal, self};
use nix::unistd::Pid;
use nix::errno::Errno;
use nix::Result;
use nix::sys::stat;


fn send_signal(pid: u32, signal_name: &str) -> Result<()> {
    let signal = match signal_name {
        "USR1" => Signal::SIGUSR1,
        "HUP" => Signal::SIGHUP,
        "INT" => Signal::SIGINT, 
        _ => return Err(nix::Error::from(Errno::EINVAL)),
    };
    signal::kill(Pid::from_raw(pid as i32), signal)
}


fn main() {
    let args = Args::parse();
    let interval = args.interval;
    let mut failure_count = 0;
    let mut last_recovery_time = Instant::now();

    loop {
        let start = SystemTime::now();
        let (output, fail_pid) = execute_command(&args);
        let unix_time = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exit_code = output.status.code().unwrap_or(1);

        // when command failed
        if !output.status.success() {
            failure_count += 1;
            if args.failure_script.is_some(){
                println!("failure_script is wrote!!");
                let failure_env = FailureScriptEnv{
                    exit_code,
                    unix_time,
                    interval,
                    fail_pid,
                };
                execute_failure_script( "./failure.sh", failure_env);
            }

            if args.pid.is_some(){
                println!("pid is wrote!!"); 
                let pid = args.pid.unwrap();
                let signal = args.signal.as_deref().unwrap_or("HUP");
                if let Err(e) = send_signal(pid, signal) {
                    eprintln!("Failed to send signal: {}", e);
                    return;
                }
            }
        } else {
            failure_count = 0;

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
