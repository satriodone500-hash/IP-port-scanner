use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct ScanResult {
    pub open_ports: Vec<u16>,
    pub total_scanned: usize,
    pub duration: Duration,
}

pub fn resolve_target(target: &str) -> Result<IpAddr, String> {
    let target = target.trim();
    if target.is_empty() {
        return Err("Target cannot be empty".to_string());
    }

    if let Ok(ip) = target.parse::<IpAddr>() {
        return Ok(ip);
    }

    let socket_str = format!("{}:80", target);
    match socket_str.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.find(|a| a.is_ipv4()) {
                Ok(addr.ip())
            } else if let Some(addr) = addrs.next() {
                Ok(addr.ip())
            } else {
                Err(format!("Could not resolve target hostname '{}'", target))
            }
        }
        Err(err) => Err(format!(
            "Failed to resolve target '{}': {}",
            target, err
        )),
    }
}

pub fn scan_ports(
    target_ip: IpAddr,
    mut ports: Vec<u16>,
    num_threads: usize,
    timeout: Duration,
) -> ScanResult {
    let total_scanned = ports.len();
    let thread_count = if num_threads == 0 {
        1
    } else {
        num_threads.min(total_scanned).max(1)
    };

    ports.reverse();
    let ports_queue = Arc::new(Mutex::new(ports));
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::with_capacity(thread_count);
    let start_time = Instant::now();

    for _ in 0..thread_count {
        let queue = Arc::clone(&ports_queue);
        let sender = tx.clone();

        let handle = thread::spawn(move || {
            loop {
                let port = {
                    let mut lock = match queue.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => poisoned.into_inner(),
                    };
                    lock.pop()
                };

                let Some(port) = port else {
                    break;
                };

                let socket_addr = SocketAddr::new(target_ip, port);
                if TcpStream::connect_timeout(&socket_addr, timeout).is_ok() {
                    let _ = sender.send(port);
                }
            }
        });

        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        let _ = handle.join();
    }

    let mut open_ports: Vec<u16> = rx.iter().collect();
    open_ports.sort_unstable();

    let duration = start_time.elapsed();

    ScanResult {
        open_ports,
        total_scanned,
        duration,
    }
}
