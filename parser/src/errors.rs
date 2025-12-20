use std::{fmt, io};

#[derive(Debug)]
pub enum TransactionError {
    InvalidAmountArguments,
    CorruptedField(String),
}

#[derive(Debug)]
pub enum ParsingError {
    TransactionError(TransactionError),
    Io(io::Error),
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
