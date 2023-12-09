mod cli;
mod executor;


use clap::Parser;

use crate::cli::Args;
use crate::executor::{execute_command, execute_failure_script, FailureScriptEnv};

use nix::sys::signal::*;
use nix::sys::signal;
use nix::unistd::Pid;
use nix::errno::Errno;
use nix::Result;
use nix::unistd::alarm;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};


static ALARM_RECEIVED: AtomicBool = AtomicBool::new(false);

extern fn signal_handler(_: nix::libc::c_int) { 
    ALARM_RECEIVED.store(true, Ordering::SeqCst);
}

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

    let sa = SigAction::new(
        SigHandler::Handler(signal_handler),
        SaFlags::SA_RESTART,
        SigSet::empty()
    );
    unsafe {
        sigaction(Signal::SIGALRM, &sa).expect("Failed to set signal handler");
    }
    let args = Args::parse();
    let interval = args.interval;
    // let mut failure_count = 0;
    // let mut last_recovery_time = Instant::now();
    alarm::set(interval as u32);
    let mut old_mask = SigSet::empty();
    let mut mask = SigSet::empty();
    mask.add(Signal::SIGALRM);

    loop {
        // Block SIGALRM
        let _ = sigprocmask(SigmaskHow::SIG_BLOCK, Some(&mask), Some(&mut old_mask));
        if ALARM_RECEIVED.load(Ordering::SeqCst) {
            ALARM_RECEIVED.store(false, Ordering::SeqCst);
            alarm::set(interval as u32);
            let start = SystemTime::now();
            
            let (output, fail_pid) = execute_command(&args);
            let unix_time = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
            let exit_code = output.status.code().unwrap_or(1);

            // when command failed
            if !output.status.success() {
                // failure_count += 1;
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
                    // failure_count = 0;

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
            }
            // Unblock SIGALRM
            let _ = sigprocmask(SigmaskHow::SIG_UNBLOCK, Some(&mask), None);
            
    }
}
