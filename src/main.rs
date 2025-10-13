use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
};
fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let Ok(listener) = TcpListener::bind(addr) else {
        print!("Error: TcpListener binding failed.");
        return;
    };
    for thread in listener.incoming() {
        std::thread::spawn(move || {
            println!("handle created");
            let mut buf = [0; 512];
            match thread {
                Ok(mut stream) => {
                    let Ok(msg) = stream.read(&mut buf) else {
                        print!("Error: reading of incoming packet failed.");
                        return;
                    };
                    let msg = msg.to_ne_bytes();
                    let (header, body) = split_http(&msg).unwrap();
                    let header_lines = split_header(header);
                    println!("Message received: {:?}, {:?}", header_lines, body);
                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!".as_bytes();
                    stream.write(response).ok();
                }
                Err(e) => print!("This TcpStream errored: {e}"),
            }
        });
    }
}

/// splits a http message into header and body. TODO: Should return Result instead.
fn split_http(msg: &[u8]) -> Result<(&[u8], &[u8]), &'static str> {
    // define pattern to search for
    let pos = msg.windows(4).position(|window| window == b"\r\n\r\n");
    match pos {
        Some(val) => {
            let header = &msg[..val];
            let body = &msg[val + 4..];
            Ok((header, body))
        }
        None => Err("parsing error."),
    }
}

fn split_header(header: &[u8]) {
    let header_text = String::from_utf8_lossy(header);
    let mut lines: Vec<String> = Vec::new();
    for line in header_text.lines() {
        lines.push(line.to_string());
    }
}
