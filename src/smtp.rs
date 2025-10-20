use std::{io::Read, net::{SocketAddr, TcpListener, ToSocketAddrs}};
pub struct SmtpClient {
    server_addr: SocketAddr,
    tls: AuthenticationMethod,
}
pub struct SmtpMessage
{
    to: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    content: Option<String>,
}

pub enum AuthenticationMethod
{
    Tls,
    NoTls,
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

impl SmtpClient {

    /// binds to a server_addr to start a smtp connection.
    /// Returns smtp struct, which includes a std::net::TcpStream inside,
    /// which is used to communicate.
    pub fn bind_to_server_addr<T>(&self, addr: T) -> Result<SmtpClient, SmtpError>
    where T: ToSocketAddrs
    {
        let mut stream = std::net::TcpStream::connect(addr)?;
        let mut buf = [0; 512];
        stream.read(&mut buf);

        if buf[0..=2] == [2,2,0]
        {
        Ok(Self
            {
                server_addr: stream.local_addr()?,
                tls: AuthenticationMethod::Tls,
            }
        )
        }
        else { Err(SmtpError::TcpError(String::from("smtp server didn't respond correctly"))) }
    }

    /// Performs the initial smtp handshake. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn handshake(&self) -> Result<_, SmtpError>
    {

    }

    /// Performs the initial smtp handshake as well as the STARTTLS call and authentication,
    /// using username and password fields. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn handshake_tls(&self) -> Result<_, Error>
    {

    }

    /// Performs the tls authentication, following a STARTTLS call. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn tls_auth(&self) -> Result<_, SmtpError>
    {

    }

    /// Performs message sending over smtp.
    /// This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn send_msg(&self) -> Result<_, SmtpError>
    {

    }

    /// Performs a full client-server roundtrip, including
    /// handshake, authentication and message sending.
    /// smtp::AuthenticationMethod::Tls and ::NoTls can be used to indicate
    /// whether the connection should use the STARTTLS call
    pub fn send_email(&mut self, msg: SmtpMessage) -> Result<(), SmtpError>
    {
        let port = self.bind()?;
        self.handshake()?;
        self.tls_auth()?;
        self.send_msg()?;
        Ok(())
    }
}
