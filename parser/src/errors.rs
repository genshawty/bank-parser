use std::{array::TryFromSliceError, fmt, io};

// todo: display and error
#[derive(Debug)]
pub struct TxTypeParseError(pub String);

// todo: display and error
#[derive(Debug)]
pub enum TransactionError {
    InvalidDataFormat,
    InvalidAmountArguments(usize),
    CorruptedField(String, String),
    MissingField(String),
}

// todo: display and error
#[derive(Debug)]
pub enum ParsingError {
    IncorrectHeader,
    TransactionError(TransactionError),
    Io(io::Error),
    InvalidDataFormat,
}

impl From<io::Error> for ParsingError {
    fn from(e: io::Error) -> Self {
        ParsingError::Io(e)
    }
}

impl From<TransactionError> for ParsingError {
    fn from(e: TransactionError) -> Self {
        ParsingError::TransactionError(e)
    }
}

impl From<TryFromSliceError> for ParsingError {
    fn from(_: TryFromSliceError) -> Self {
        ParsingError::InvalidDataFormat
    }
}
