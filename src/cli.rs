use clap::Parser;


/// Simple heartbeat command-line app
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Interval in seconds between checks
    #[clap(short, long, value_parser)]
    pub interval: u64,

    /// Shell script to execute
    #[clap(short = 's', long, value_parser)]
    pub script: Option<String>,

    /// The command to execute
    #[clap(value_parser, trailing_var_arg = true)]
    pub command: Vec<String>,

    #[clap(long, value_parser)]
    pub pid: Option<u32>,
    
    #[clap(long, value_parser)]
    pub signal: Option<String>,

    #[clap(long = "faill", value_parser)]
    pub failure_script: Option<String>,
}