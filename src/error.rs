pub mod error {
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
}
