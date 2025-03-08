use std::env;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn dnslookup(domain: &str) {
    match (domain,80).to_socket_addrs() {
        Ok(iter) => {
            for addr in iter {
                println!("{} {} {}",domain, if addr.is_ipv6() { "IPv6" } else { "IPv4" }, addr);
            }
        }
        Err(e) => println!("DNS lookup failed: {}", e),
    }
}

fn seqcom() {
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage {} <domain>", args[0]);
        return;
    }

    let domain = &args[1];
    dnslookup(domain);
}
