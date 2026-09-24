mod cli;
mod ports;
mod scanner;

use clap::Parser;
use cli::Cli;
use ports::parse_ports;
use scanner::{resolve_target, scan_ports};
use std::process;
use std::time::Duration;

fn main() {
    let args = Cli::parse();

    let target_ip = match resolve_target(&args.target) {
        Ok(ip) => ip,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let port_list = match parse_ports(&args.ports) {
        Ok(ports) => ports,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    println!("Rust Port Scanner\n");
    println!("Target: {}", args.target);
    println!("Ports: {}\n", args.ports);

    let timeout = Duration::from_millis(args.timeout_ms);
    let result = scan_ports(target_ip, port_list, args.threads, timeout);

    for port in &result.open_ports {
        println!("{:<6} OPEN", port);
    }

    if !result.open_ports.is_empty() {
        println!();
    }

    println!("Scan completed in {:.2}s\n", result.duration.as_secs_f64());
    println!("Scanned: {}", result.total_scanned);
    println!("Open: {}", result.open_ports.len());
}
