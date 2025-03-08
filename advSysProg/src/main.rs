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

fn seqcon(domain: &str) {
    let addrs: Vec<_> = match (domain, 80).to_socket_addrs() {
        Ok(iter) => iter.collect(),
        Err(e) => {
            println!("DNS lookup failed: {}", e);
            return;
        }
    };

    if addrs.is_empty() {
        println!("No IP addresses were found for domain {}", domain);
        return;
    }

    for addr in addrs {
        //match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                println!("Connected to {}", addr);
                let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", domain);
                stream.write_all(request.as_bytes()).unwrap();
                
                let mut response = String::new();
                stream.read_to_string(&mut response).unwrap();
                println!("Response:\n{}", response);
                return; 
            }
            Err(_) => continue,
        }
    }

    println!("Failed to establish connection");


}

fn concon(domain: &str) {
    let addrs: Vec<_> = match (domain, 80).to_socket_addrs() {
        Ok(iter) => iter.collect(),
        Err(e) => {
            println!("DNS lookup failed: {}", e);
            return;
        }
    };

    if addrs.is_empty() {
        println!("No IP addresses were found for domain {}", domain);
        return;
    }

    let (tx,rx) = mpsc::channel();

    for addr in addrs {
        let txClone = tx.clone();
        thread::spawn(move || {
            //match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
            match TcpStream::connect(&addr) {
                Ok(stream) => {
                    println!("Connected to {}", addr);
                    txClone.send(stream).ok(); 
                }
                Err(_) => {},
            }
        });
    }

    drop(tx);

    if let Ok(mut stream) = rx.recv() {
        let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", domain);
        stream.write_all(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        println!("Response:\n{}", response);
    } else {
        println!("Failed to establish connection to any IP address.");
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage {} <domain>", args[0]);
        return;
    }

    let domain = &args[1];

    //dnslookup(domain);

    //seqcon(domain);

    concon(domain);
}
