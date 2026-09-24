use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-port-scanner", about = "Simple TCP Port Scanner in Rust")]
pub struct Cli {
    #[arg(help = "Target IPv4 address or hostname")]
    pub target: String,

    #[arg(
        short = 'p',
        long = "ports",
        default_value = "1-1024",
        help = "Ports to scan (e.g. 80, 22,80,443, 1-1000)"
    )]
    pub ports: String,

    #[arg(
        short = 't',
        long = "threads",
        default_value_t = 100,
        help = "Number of worker threads"
    )]
    pub threads: usize,

    #[arg(
        long = "timeout",
        default_value_t = 1000,
        help = "Connection timeout in milliseconds"
    )]
    pub timeout_ms: u64,
}
