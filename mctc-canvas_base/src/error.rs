use std::{error::Error, fmt::Display, string::FromUtf8Error, sync::Arc};

pub type PResult<T> = Result<T, PError>;

#[derive(Debug, Clone)]
pub enum StringEncoding {
    Utf8,
}

impl Display for StringEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringEncoding::Utf8 => write!(f, "utf8"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PError {
    Io(Arc<std::io::Error>),
    InvalidString(StringEncoding),
    InvalidTypeId(u64),
    InvalidLength(usize, usize),
    InvalidValue(u64),
}

impl Error for PError {}

impl Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PError::Io(e) => e.fmt(f),
            PError::InvalidString(enc) => write!(f, "invalid string ({enc})"),
            PError::InvalidTypeId(id) => write!(f, "invalid id (0x{id:X})"),
            PError::InvalidLength(len, exp) => write!(f, "unexpected length (found {len}, expected {exp})"),
            PError::InvalidValue(val) => write!(f, "unexpected value (found {val})"),
        }
    }
}


impl From<std::io::Error> for PError {
    fn from(value: std::io::Error) -> Self {
        PError::Io(Arc::new(value))
    }
}

impl From<FromUtf8Error> for PError {
    fn from(_value: FromUtf8Error) -> Self {
        PError::InvalidString(StringEncoding::Utf8)
    }
}