mod cli;
mod executor;


use clap::Parser;

use crate::cli::Args;
use crate::executor::{execute_command, execute_failure_script};
use libc;
use crate::stat::Mode;

use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use nix::sys::signal::{Signal, self};
use nix::unistd::Pid;
use nix::errno::Errno;
use nix::Result;
use std::path::Path;
use nix::unistd::mkfifo;
use std::fs;
// use std::process::Command; 
use std::io::Write;
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
    
    let pipe_path = Path::new("/tmp/rust_pipe");
    let _ = fs::remove_file(pipe_path);
     // Create a named pipe (FIFO)
    let mode = Mode::from_bits_truncate(libc::S_IRUSR  | libc::S_IWUSR | libc::S_IRGRP  | libc::S_IROTH );
    match mkfifo(pipe_path, mode) {
        Ok(()) => println!("Named pipe created successfully at {:?}", pipe_path),
        Err(e) => eprintln!("Failed to create named pipe: {}", e),
    }

    loop {
        let start = SystemTime::now();
        let output = execute_command(&args);
        let unix_time = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exit_code = output.status.code().unwrap_or(1);

        // when command failed
        if !output.status.success() {
            if args.failure_script.is_some(){
                println!("failure_script is wrote!!");
                execute_failure_script(pipe_path, "./failure.sh");
                let mut pipe = fs::OpenOptions::new().write(true).open(pipe_path).expect("Failed to open the named pipe");
                writeln!(pipe, "{} {}", exit_code, unix_time).expect("Failed to write to the named pipe");
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
