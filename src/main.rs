use clap::Parser;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target host
    host: String,

    /// Target port
    port: u16,

    /// Ping interval in milliseconds (0 for fast ping)
    #[arg(short = 'i', default_value_t = 1000)]
    interval: u64,

    /// Number of pings (default 4). If it ends with 's' (e.g., '10s'), it specifies duration in seconds.
    #[arg(short = 'n', default_value = "4")]
    count_or_duration: String,

    /// Force IPv4
    #[arg(short = '4', default_value_t = false)]
    ipv4: bool,

    /// Force IPv6
    #[arg(short = '6', default_value_t = false)]
    ipv6: bool,

    /// Warmup seconds
    #[arg(short = 'w', default_value_t = 1)]
    warmup: u64,
}

struct Stats {
    sent: u32,
    received: u32,
    min_rtt: Duration,
    max_rtt: Duration,
    total_rtt: Duration,
}

impl Stats {
    fn new() -> Self {
        Self {
            sent: 0,
            received: 0,
            min_rtt: Duration::from_secs(999999),
            max_rtt: Duration::from_secs(0),
            total_rtt: Duration::from_secs(0),
        }
    }

    fn update(&mut self, rtt: Duration) {
        self.sent += 1;
        self.received += 1;
        if rtt < self.min_rtt {
            self.min_rtt = rtt;
        }
        if rtt > self.max_rtt {
            self.max_rtt = rtt;
        }
        self.total_rtt += rtt;
    }

    fn record_loss(&mut self) {
        self.sent += 1;
    }

    fn print_summary(&self, host: &str, port: u16) {
        println!("\n--- {} tcp ping statistics ---", host);
        let loss = if self.sent > 0 {
            (self.sent - self.received) as f64 / self.sent as f64 * 100.0
        } else {
            0.0
        };
        println!(
            "{} packets transmitted, {} received, {:.1}% packet loss",
            self.sent, self.received, loss
        );
        if self.received > 0 {
            let avg = self.total_rtt / self.received;
            println!(
                "rtt min/avg/max = {:?}/{:?}/{:?}",
                self.min_rtt, avg, self.max_rtt
            );
        }
    }
}

fn resolve_address(host: &str, port: u16, ipv4: bool, ipv6: bool) -> io::Result<SocketAddr> {
    let addrs = (host, port).to_socket_addrs()?;
    for addr in addrs {
        if ipv4 && !addr.is_ipv4() {
            continue;
        }
        if ipv6 && !addr.is_ipv6() {
            continue;
        }
        return Ok(addr);
    }
    Err(io::Error::new(
        io::ErrorKind::AddrNotAvailable,
        "No suitable address found",
    ))
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let addr = resolve_address(&args.host, args.port, args.ipv4, args.ipv6)?;
    println!("rtcping {} ({}) port {}", args.host, addr.ip(), args.port);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut stats = Stats::new();
    
    // Warmup
    if args.warmup > 0 {
        println!("Warming up for {} seconds...", args.warmup);
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(args.warmup) && running.load(Ordering::SeqCst) {
            let _ = TcpStream::connect_timeout(&addr, Duration::from_secs(1));
            thread::sleep(Duration::from_millis(100));
        }
    }

    let is_duration = args.count_or_duration.ends_with('s');
    let limit = if is_duration {
        args.count_or_duration[..args.count_or_duration.len()-1].parse::<u64>().unwrap_or(4)
    } else {
        args.count_or_duration.parse::<u64>().unwrap_or(4)
    };

    let start_time = Instant::now();
    let mut count = 0;

    while running.load(Ordering::SeqCst) {
        if is_duration {
            if start_time.elapsed() >= Duration::from_secs(limit) {
                break;
            }
        } else {
            if count >= limit {
                break;
            }
        }

        let start = Instant::now();
        match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
            Ok(_) => {
                let rtt = start.elapsed();
                stats.update(rtt);
                println!(
                    "Connected to {}:{} time={:?}",
                    args.host, args.port, rtt
                );
            }
            Err(e) => {
                stats.record_loss();
                println!("Connect to {}:{} failed: {}", args.host, args.port, e);
            }
        }

        count += 1;
        
        if args.interval > 0 {
            thread::sleep(Duration::from_millis(args.interval));
        }
    }

    stats.print_summary(&args.host, args.port);

    Ok(())
}
