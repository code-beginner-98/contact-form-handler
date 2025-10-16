use std::{
    collections::{HashMap, VecDeque},
    fmt,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream}
};

const MAX_HEADER_SIZE: usize = 512 * 8;

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let Ok(listener) = TcpListener::bind(addr) else {
        print!("Error: TcpListener binding failed.");
        return;
    };

    // Spawn new thread for incoming connection
    for thread in listener.incoming() {
        std::thread::spawn(move || {
            println!("handle created");
            let mut buf = [0; 512];
            let mut msg = Vec::new();

            match thread {
                Ok(mut stream) => {
                    // read incoming data into msg, until "\r\n\r\n" pattern is detected in msg or until msg hits size limit
                    let mut pos: Option<usize> = None;
                    while pos == None {
                        let Ok(_msg_size) = stream.read(&mut buf) else {
                            // Send 400 Bad Request
                            println!("Error: reading of incoming packet failed.");
                            bad_request(&stream);
                            return;
                        };

                        // Check for empty line pattern.
                        pos = check_http_header(&buf);

                        // write buf into msg and check msg length
                        msg.append(&mut buf.to_vec());
                        if msg.len() > MAX_HEADER_SIZE {
                            // Send 400 Bad Request
                            println!("Error: header max size exceeded, terminating connection.");
                            bad_request(&stream);
                            return;
                        }
                    }

                    // Pattern found, trim front of message, move to header vec, construct header
                    // trim msg and construct header
                    let Ok((header, trail)) = split_header_from(msg) else {
                        // Send 400 Bad Request
                        println!("Error: header content corrupted.");
                        bad_request(&stream);
                        return;
                    };

                    // Read content length
                    let content_length =
                        header
                        .content
                        .get("Content-Length")
                        .unwrap() // TODO: Error handling
                        .parse::<usize>()
                        .unwrap() // TODO: Error handling
                    ;

                    // store trailing data in msg
                    msg = trail;

                    // read from buf until msg > content length
                    while msg.len() < content_length {
                        let Ok(_msg_size) = stream.read(&mut buf) else {
                            // Send 400 Bad Request
                            println!("Error: reading of incoming packet failed.");
                            bad_request(&stream);
                            return;
                        };

                        msg.append(&mut buf.to_vec());
                    }

                    // extract body information
                    let Ok(contact_data) = split_body_from(msg) else {
                        // Send 400 Bad Request
                        println!("Error: reading of body failed.");
                        bad_request(&stream);
                        return;
                    };

                    // construct smtp from body fields
                    let smtp_message = SmtpMessage::from(contact_data);

                    // Send success response website
                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!".as_bytes();
                    stream.write(response).ok(); // TODO: Handle incomplete writes
                }
                Err(e) => print!("This TcpStream errored: {e}"),
            }
        });
    }
}

/// Takes a http-conform header buffer and returns the position of the Empty Line "\r\n\r\n", if present.
fn check_http_header(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

/// Takes A HTTP-conform msg and returns the header and trailing content after the empty line.
fn split_header_from(msg: Vec<u8>) -> Result<(HttpHeader, Vec<u8>), &'static str> {
    let pos = msg.windows(4).position(|window| window == b"\r\n\r\n");
    let mut header = Vec::new();
    let mut trail = Vec::new();

    match pos {
        Some(val) => {
            header = msg[..val].to_vec();
            trail = msg[val + 4..].to_vec();
        }

        None => {
            let e = "Error: Message got corrupted at runtime.";
            return Err(e);
        }
    }

    // form header struct
    let Ok((start_line, content)) = split_header(header) else {
        let e = "Error: Message got corrupted at runtime";
        return Err(e);
    };

    let http_header = HttpHeader {
        start_line: start_line,
        content: content,
    };

    Ok((http_header, trail))
}

/// Takes in header slice and turns them into a key-value map for each line
fn split_header(header: Vec<u8>) -> Result<(Vec<String>, HashMap<String, String>), Error> {
    // convert header bytes to text
    let header_text = String::from_utf8(header)?;

    // collect into Vector. Note that .split().collect() is easier on the eyes than "for line in header_text {lines.push()}""
    // Checks for empty lines created by split(), since last line will certainly end in "\r\n", creating an empty line in the vec.
    let lines = header_text
        .split("\r\n")
        .filter(|f| !f.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // VecDeques allow pop_front(), which is better than Vec::remove(), which is O(n).
    let mut lines = VecDeque::from(lines);
    let start_line = lines.pop_front().unwrap(); // TODO: Error handling

    let start_vec = start_line
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // extract key-value pairs from the remaining lines
    let mut header_map = HashMap::new();
    for line in lines.iter() {
        let (key, value) = line.split_once(": ").unwrap();

        header_map.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok((start_vec, header_map))
}

fn split_body_from(msg: Vec<u8>) -> Result<HttpMessage, Error> {
    let body = String::from_utf8(msg)?;
    let pairs = body
        .split("&")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut mail_map = HashMap::new();
    for pair in pairs {
        let (key, value) = pair.split_once("=").unwrap(); // TODO: Error Handling
        mail_map.insert(key.trim().to_string(), value.trim().to_string());
    }

    // construct HttpMessage
    let http_message = HttpMessage::from(mail_map)?;

    Ok(http_message)
}

struct HttpHeader {
    start_line: Vec<String>,
    content: HashMap<String, String>,
}

struct HttpMessage {
    from_mail: String,
    surname: String,
    last_name: String,
    subject: String,
    content: String,
}

impl HttpMessage {
    fn from(mail_map: HashMap<String, String>) -> Result<HttpMessage, Error> {
        let (from_mail, surname, last_name, subject, content) = (
            mail_map.get("email").ok_or("missing email field")?.to_string(),
            mail_map.get("surname").ok_or("missing surname field")?.to_string(),
            mail_map.get("last_name").ok_or("missing last_name field")?.to_string(),
            mail_map.get("subject").ok_or("missing subject field")?.to_string(),
            mail_map.get("content").ok_or("missing content field")?.to_string(),
        );

        // read map into fields
        // TODO: Contact form variable names, structure, fields? This is kind of a placeholder for now.
        Ok(
            HttpMessage {
                from_mail: from_mail,
                surname: surname,
                last_name: last_name,
                subject: subject,
                content: content,
            }
        )
    }
}

#[derive(Debug)]
enum Error {
    HttpError(String),
    Utf8Error(String),
    StringError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
            Error::Utf8Error(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
            Error::StringError(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error
{
    fn from(e: std::string::FromUtf8Error) -> Self
    {
        Error::Utf8Error(e.to_string())
    }
}

impl From<&str> for Error
{
    fn from(e: &str) -> Self
    {
        Error::StringError(e.to_string())
    }
}

struct SmtpMessage {}

impl SmtpMessage {
    fn from(http_message: HttpMessage) -> Self
    {
        
    }
}

fn bad_request(mut stream: &TcpStream) {
    let response =
        "HTTP/101 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: 64\r\n\r\nError: Bad Request, Message: Request could not be read properly."
            .as_bytes();
    stream.write(response).ok(); // TODO: Handle incomplete writes
}
