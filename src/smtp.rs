use std::net::{SocketAddr, TcpListener, ToSocketAddrs};
pub struct Smtp {
    port: SocketAddr,
    to: Option<String>,
    from: Option<String>,
    content: Option<String>,
    tls: AuthenticationMethod,
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

impl Smtp {

    /// binds to a free port to start a smtp connection.
    /// Returns smtp struct, which includes a std::net::TcpStream inside,
    /// which is used to communicate.
    /// Smtp::bind_to_port lets you have control over the port.
    pub fn bind() -> Result<Smtp, SmtpError> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        Ok(Self
            {
                port: listener.local_addr()?,
                to: None,
                from: None,
                content: None,
                tls: AuthenticationMethod::Tls,
            }
        )
    }

    /// binds to a free port to start a smtp connection.
    /// Returns smtp struct, which includes a std::net::TcpStream inside,
    /// which is used to communicate.
    /// Smtp::bind_to_port lets you have control over the port.
    pub fn bind_to_port<T>(addr: T) -> Result<Smtp, SmtpError>
    where T: ToSocketAddrs
    {
        let listener = TcpListener::bind(addr)?;
        Ok(Self
            {
                port: listener.local_addr()?,
                to: None,
                from: None,
                content: None,
                tls: AuthenticationMethod::Tls,
            }
        )
    }

    /// Performs the initial smtp handshake. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn handshake(&Self) -> Result<_, SmtpError>
    {

    }

    /// Performs the initial smtp handshake as well as the STARTTLS call and authentication,
    /// using username and password fields. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn handshake_tls(&Self) -> Result<_, Error>
    {

    }

    /// Performs the tls authentication, following a STARTTLS call. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn tls_auth(&Self) -> Result<_, SmtpError>
    {

    }

    /// Performs message sending over smtp.
    /// This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn send_msg(&Self) -> Result<_, SmtpError>
    {

    }

    /// Performs a full client-server roundtrip, including
    /// handshake, authentication and message sending.
    /// smtp::AuthenticationMethod::Tls and ::NoTls can be used to indicate
    /// whether the connection should use the STARTTLS call
    pub fn send_email(&Self) -> Result<_, SmtpError>
    {

    }
}
