pub mod error {

    use std::fmt;
    #[derive(Debug)]
    pub enum SmtpError {
        SmtpError(String),
        TcpError(String),
        IoError(String),
        TcpWriteError(String),
        TcpReadError(String),
    }

    impl From<&str> for SmtpError {
        fn from(e: &str) -> Self {
            SmtpError::TcpError(e.to_string())
        }
    }

    impl From<std::io::Error> for SmtpError {
        fn from(e: std::io::Error) -> Self {
            SmtpError::IoError(e.to_string())
        }
    }

    #[derive(Debug)]
    pub enum Error {
        HttpError(String),
        Utf8Error(String),
        StringError(String),
        SmtpError(String),
    }

    impl std::error::Error for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::HttpError(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
                Error::Utf8Error(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
                Error::StringError(error_msg) => write!(f, "Error: HttpError: {error_msg}"),
                Error::SmtpError(error_msg) => write!(f, "Error: SmtpEror: {error_msg}"),
            }
        }
    }

    impl From<std::string::FromUtf8Error> for Error {
        fn from(e: std::string::FromUtf8Error) -> Self {
            Error::Utf8Error(e.to_string())
        }
    }

    impl From<&str> for Error {
        fn from(e: &str) -> Self {
            Error::StringError(e.to_string())
        }
    }
}