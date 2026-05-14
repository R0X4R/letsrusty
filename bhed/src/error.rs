use std::io;

#[derive(Debug)]
pub enum BhedError {
    Io(io::Error),
    InvalidSeparator(String),
}

impl From<io::Error> for BhedError {
    fn from(err: io::Error) -> Self {
        BhedError::Io(err)
    }
}

impl std::fmt::Display for BhedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BhedError::Io(err) => write!(f, "IO error: {}", err),
            BhedError::InvalidSeparator(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for BhedError {}

pub type Result<T> = std::result::Result<T, BhedError>;
