use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

pub type SmtpResult<T> = std::result::Result<T, SmtpError>;

#[derive(Debug)]
enum Error {
    HttpError(String),
    Utf8Error(String),
    StringError(String),
    SmtpError(String),
    IoError(String)
}


#[derive(Debug)]
pub enum SmtpError {
    TcpError(String),
    IoError(String),
    TcpWriteError(String),
    TcpReadError(String),
}

// main Error impl

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
            Error::Utf8Error(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
            Error::StringError(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
            Error::SmtpError(error_msg) => write!(f, "Error: SmtpEror: {error_msg}"),
            Error::IoError(error_msg) => write!(f, "Error: IoError: {error_msg}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e.to_string())
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

// SmrpError impl

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
