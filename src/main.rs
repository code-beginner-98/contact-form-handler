use std::{
    collections::HashMap, io::{Read, Write}, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}
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
            let mut msg = [0; 512];
            // TODO: Handle incomplete requests/read to buffer, wait for more
            match thread {
                Ok(mut stream) => {
                    let Ok(_msg_size) = stream.read(&mut msg) else {
                        print!("Error: reading of incoming packet failed.");
                        return;
                    };
                    // print!("Message: {}", String::from_utf8_lossy(&msg));
                    let (header, body) = split_http(&msg);
                    // println!("header: {header:?}");
                    // println!("body:   {body:?}\n");
                    let header_map = split_header(header);
                    let mail_map = match body {
                        None => HashMap::new(),
                        Some(body_text) => split_body(body_text)
                    };
                    let keys = ["Connection", "Cache-Control", "Host", "Content-Length"];
                    for key in keys {
                        match header_map.get(key) {
                            Some(value) => println!("{key}: {value}"),
                            None => println!("{key}: no value associated.")
                        };
                    };
                    println!("mail map: {mail_map:?}");
                    // TODO: Check Body Length
                    // Send response
                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!".as_bytes();
                    // TODO: Handle incomplete writes
                    stream.write(response).ok();
                }
                Err(e) => print!("This TcpStream errored: {e}"),
            }
        });
    }
}

/// splits a http message into header and body. If no "\r\n\r\n" delimiter can be found, instead returns the message and a None.
fn split_http(msg: &[u8]) -> (&[u8], Option<&[u8]>) {
    // define pattern to search for
    //println!("parsing msg: {msg:?}");
    let pos = msg.windows(4).position(|window| window == b"\r\n\r\n");
    match pos {
        Some(val) => {
            let header = &msg[..val];
            let body = &msg[val + 4..];
            (header, Some(body))
        }
        None => (msg, None)
    }
}

/// Takes in header slice and turns them into a key-value map for each line
fn split_header(header: &[u8]) -> HashMap<String, String> {
    // convert header bytes to text
    let header_text = String::from_utf8_lossy(header);
    // collect into Vector. Note that .split().collect() is easier n the eyes than for line in header_text {lines.push()}
    // Checks for empty lines created by split(), since last line will certainly end in "\r\n", creating an empty line in the vec.
    let lines =
        header_text
        .split("\r\n")
        .filter(|f| !f.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        ;
    // extract key-value pairs from the 2nd line onwards at ": "
    let mut header_map = HashMap::new();
    for line in lines.iter().skip(1) {
        // println!("current line mapping: \"{line}\"");
        let (key, value) = line.split_once(": ").unwrap();
        header_map.insert(key.trim().to_string(), value.trim().to_string());
    }
    header_map
}

fn split_body(body: &[u8]) -> HashMap<String, String> {
    let body_text = String::from_utf8_lossy(body);
    let pairs =
        body_text
        .split("&")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut mail_map = HashMap::new();
    for pair in pairs {
        let (key, value) = pair.split_once("=").unwrap();
        mail_map.insert(key.trim().to_string(), value.trim().to_string());
    }
    mail_map
}