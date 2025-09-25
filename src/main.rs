use clap::Parser;
use futures::stream::{self, StreamExt};
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(name = "portscan")]
#[command(about = "Simple async port scanner (TCP)")]
struct Args {
    /// Hostname or IP to scan (e.g. example.com or 192.168.1.10)
    host: String,

    /// Start port (inclusive)
    #[arg(short, long, default_value_t = 1)]
    start: u16,

    /// End port (inclusive)
    #[arg(short, long, default_value_t = 1024)]
    end: u16,

    /// Concurrency: how many ports to check in parallel
    #[arg(short, long, default_value_t = 100)]
    concurrency: usize,

    /// Timeout per connection attempt in milliseconds
    #[arg(short, long, default_value_t = 300)]
    timeout_ms: u64,
}

async fn scan_port(host: &str, port: u16, timeout_ms: u64) -> Option<u16> {
    let addr = format!("{}:{}", host, port);

    // First try resolving the hostname quickly; if it fails, return None.
    // If host is an IP, ToSocketAddrs will succeed fast.
    let addrs = match addr.to_socket_addrs() {
        Ok(iter) => iter.collect::<Vec<_>>(),
        Err(_) => return None,
    };

    // Try each resolved address until one connects
    for sock in addrs {
        let conn = timeout(Duration::from_millis(timeout_ms), TcpStream::connect(sock)).await;
        match conn {
            Ok(Ok(_stream)) => {
                // Connected successfully
                return Some(port);
            }
            _ => {
                // timed out or failed, try next resolved addr (if any)
            }
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.start > args.end {
        eprintln!("start port must be <= end port");
        std::process::exit(1);
    }

    let ports: Vec<u16> = (args.start..=args.end).collect();

    println!(
        "Scanning {} from {} to {} (concurrency={}, timeout={}ms)...",
        args.host, args.start, args.end, args.concurrency, args.timeout_ms
    );

    // Use a stream with buffer_unordered to limit concurrency
    let results = stream::iter(ports)
        .map(|port| {
            let host = args.host.clone();
            let timeout_ms = args.timeout_ms;
            async move {
                let open = scan_port(&host, port, timeout_ms).await;
                (port, open.is_some())
            }
        })
        .buffer_unordered(args.concurrency)
        .collect::<Vec<_>>()
        .await;

    // collect and print open ports in order
    let mut open_ports: Vec<u16> = results
        .into_iter()
        .filter_map(|(port, is_open)| if is_open { Some(port) } else { None })
        .collect();

    open_ports.sort_unstable();

    if open_ports.is_empty() {
        println!("No open TCP ports found in range.");
    } else {
        println!("Open TCP ports:");
        for p in open_ports {
            println!("  - {}", p);
        }
    }
}
