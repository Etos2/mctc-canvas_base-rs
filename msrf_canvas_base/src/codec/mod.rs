use std::{fmt::Display, str::Utf8Error};

mod v0_0;

// TODO: Deduplicate common record errors
#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedType(u16),
    InvalidValueLength,
    InvalidUTF8(Utf8Error),
    InvalidField(Vec<u8>)
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedType(id) => write!(f, "unexpected type {id}"),
            Error::InvalidValueLength => write!(f, "value too small"),
            Error::InvalidUTF8(e) => e.fmt(f),
            Error::InvalidField(culprit) => write!(f, "invalid data in record ({culprit:x?})"),
        }
    }
}

impl From<usize> for Error {
    fn from(_value: usize) -> Self {
        Error::InvalidValueLength
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Error::InvalidUTF8(value)
    }
}