use std::env;
use std::io::{Error, ErrorKind, Result};
use std::net::UdpSocket;

fn get_server_info(server_ip: &str, server_port: u16) -> Result<(u8, Vec<String>)> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.set_nonblocking(true)?;

    let request: [u8; 25] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0x54,
        0x53, 0x6F, 0x75, 0x72, 0x63,
        0x65, 0x20, 0x45, 0x6E, 0x67,
        0x69, 0x6E, 0x65, 0x20, 0x51,
        0x75, 0x65, 0x72, 0x79, 0x00
    ];

    sock.send_to(&request, format!("{}:{}", server_ip, server_port))?;

    let mut buf = [0u8; 4096];
    let (size, _) = sock.recv_from(&mut buf)?;

    let response = &buf[..size];
    let response_str = String::from_utf8_lossy(response);
    let response_parts: Vec<&str> = response_str.split("\x00\x00\x01").collect();

    if response_parts.len() > 1 {
        let tickrate = response_parts[1].as_bytes()[1];
        let stats = response_parts[1][7..].split('\x00').map(|s| s.to_string()).collect();

        Ok((tickrate, stats))
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Invalid response format"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: ./cs2-server-metrics <server_ip> <server_port>");
        return;
    }

    let server_ip = &args[1];
    let server_port: u16 = match args[2].parse() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid server port");
            return;
        }
    };

    match get_server_info(server_ip, server_port) {
        Ok((tickrate, stats)) => {
            println!("Tickrate: {}", tickrate);
            println!("Networking stats: {:?}", stats);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}