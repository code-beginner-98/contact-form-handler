use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}};
pub struct SmtpClient {
    stream: TcpStream,
    tls: AuthenticationMethod,
}
pub struct SmtpMessage
{
    to: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    content: Option<String>,
}

#[derive(PartialEq)]
pub enum AuthenticationMethod
{
    Tls,
    NoTls,
}
#[derive(Debug)]
pub enum SmtpError {
    TcpError(String),
    IoError(String),
    TcpWriteError(String),
    TcpReadError(String),
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
                stream: stream,
                tls: AuthenticationMethod::Tls,
            }
        )
        }
        else { Err(SmtpError::TcpError(String::from("smtp server didn't respond correctly"))) }
    }

    /// Performs the initial smtp handshake. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn handshake(&mut self) -> Result<(), SmtpError>
    {
        let msg = "EHLO rustclient.local";
        self.write_line(msg)?;
        self.expect_line("250").ok_or(SmtpError::TcpReadError("Error: server sent unexpected response at: ".to_string() + msg))?;
        
        Ok(())
    }

    /// Writes a line to the client's inner tcp stream.
    pub fn write_line<S>(&mut self, msg: S) -> Result<(), SmtpError>
    where S: ToString
    {
        let buf = msg.to_string();
        let buf_bytes = buf.as_bytes();
        self.stream.write(buf_bytes)?;
        Ok(())
    }

    /// Reads from connection and checks for reply code. Returns a None if the code isn't found and the
    /// content of the message, if it is. Use Option::ok_or() to convert to a Result.
    pub fn expect_line<S>(&mut self, s:S) -> Option<[u8;512]>
    where S: ToString
    {
        let comp = s.to_string();
        let comp_bytes = comp.as_bytes();
        let mut buf = [0; 512];
        self.stream.read(&mut buf);
        if &buf[0..=2] == comp_bytes
        {
            return Some(buf);
        }
        else 
        {
            return None;
        }
    }

    /// Performs the initial smtp handshake as well as the STARTTLS call and authentication,
    /// using username and password fields. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn handshake_tls(&mut self) -> Result<(), SmtpError>
    {
        let msg = "EHLO rustclient.local";
        self.write_line(msg)?;
        self.expect_line("250").ok_or(SmtpError::TcpReadError("Error: server sent unexpected response at: ".to_string() + msg))?;

        let msg= "STARTTLS";
        self.write_line(msg)?;
        self.expect_line("220").ok_or(SmtpError::TcpReadError("Error: server sent unexpected response at: ".to_string() + msg))?;
        
        self.tls_auth()?;
        Ok(())
    }

    /// Performs the tls authentication, following a STARTTLS call. This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn tls_auth(&self) -> Result<(), SmtpError>
    {
        Ok(())
    }

    /// Performs message sending over smtp.
    /// This function shouldn't be used directly,
    /// as it is only partial. If using, the developer must ensure that the socket
    /// is either free'd after or a message is sent over the connection.
    pub fn send_msg(&self) -> Result<(), SmtpError>
    {
        Ok(())
    }

    /// Performs a full client-server roundtrip, including
    /// handshake, authentication and message sending.
    /// smtp::AuthenticationMethod::Tls and ::NoTls can be used to indicate
    /// whether the connection should use the STARTTLS call
    pub fn send_email<A>(&mut self, addr: A, msg: SmtpMessage) -> Result<(), SmtpError>
    where A: ToSocketAddrs
    {
        let port = self.bind_to_server_addr(addr)?;
        if self.tls == AuthenticationMethod::Tls
        {
            self.handshake_tls();
        }
        else
        {
            self.handshake()?;
        }
        self.tls_auth()?;
        self.send_msg()?;
        Ok(())
    }
}
