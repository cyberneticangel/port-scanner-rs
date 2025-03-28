use std::net::TcpStream;
use std::time::Duration;
use clap::Parser;

#[derive(Parser)]

struct Cli {
    // target host (IP or Domain)
    host:String,

    // Port range start (inclusive), default value is 1
    #[arg(default_value_t = 1)]
    start_port:u16,

    // Port range end (inclusive), default value is 1024
    #[arg(default_value_t = 1024)]
    end_port:u16,

    // Timeout in milliseconds, default value is 500
    #[arg(short, long, default_value_t = 500)]
    timeout:u64,
}

fn scan_port(host:&str, port:u16, timeout:u64) -> bool {
    let addr = format!("{}:{}", host, port); // addr is a string like "127.0.0.1:80"
    if let Ok(_) = TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(timeout)) { // .parse() converts it to a SocketAddr (IP + Port)
        true
    }
    else {
        false
        }
    }

fn main() {
    let args = Cli::parse();

    println!("Scanning {} (ports {}-{})[+]", args.host, args.start_port, args.end_port);
    for port in args.start_port..=args.end_port {
        if scan_port(&args.host, port, args.timeout) {
            println!("Port {} is open", port);
        }
    }
}
