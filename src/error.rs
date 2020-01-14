use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidAddress,
    InvalidStorageKey,
    InternalError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            InvalidAddress => write!(f, "Invalid address"),
            InvalidStorageKey => write!(f, "Invalid storage key"),
            InternalError(ref e) => write!(f, "Internal Error {}", e),
        }
    }
}
