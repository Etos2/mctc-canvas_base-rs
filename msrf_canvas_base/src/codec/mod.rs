use std::{fmt::Display, io::Write, str::Utf8Error};

use msrf::error::IoError;

use crate::CanvasMeta;

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
// pub trait RawSerialiser {
//     fn write_source_add<W: Write>(&self, rec: &SourceAdd, wtr: W) -> Result<(), IoError<DesError>>;
//     fn write_source_remove<W: Write>(
//         &self,
//         rec: &SourceRemove,
//         wtr: W,
//     ) -> Result<(), IoError<DesError>>;
// }

pub trait RawSerialiser {
    fn write_canvas_meta<W: Write>(&self, rec: CanvasMeta, wtr: W) -> Result<(), IoError<Error>>;
}