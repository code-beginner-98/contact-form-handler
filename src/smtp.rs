use std::net::{SocketAddr, TcpListener};
pub struct Smtp {
    port: SocketAddr,
}

#[derive(Debug)]
pub enum SmtpError {
    TcpError(String),
    IoError(String),
}

impl From<&str> for SmtpError
{
    fn from(e: &str) -> Self
    {
        SmtpError::TcpError(e.to_string())
    }
}

impl From<std::io::Error> for SmtpError
{
    fn from(e: std::io::Error) -> Self
    {
        SmtpError::IoError(e.to_string())
    }
}
impl Smtp {

    /// binds to a free port to start a smtp connection.
    /// Returns smtp struct, which includes a std::net::TcpStream inside,
    /// which is used to communicate.
    /// Smtp::bind_to_port lets you have control over the port.
    pub fn bind() -> Result<Smtp, SmtpError> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        Ok(Self { port: listener.local_addr()? })
    }
}
