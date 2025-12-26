//! Error types for transaction parsing operations.
//!
//! This module defines the error types that can occur during transaction parsing
//! and processing across different data formats.

use std::{array::TryFromSliceError, error::Error, fmt, io};

/// Error type for invalid transaction type or status parsing.
///
/// This error occurs when attempting to parse a string or byte value into
/// a [`TxType`](crate::TxType) that doesn't match
/// any valid variant.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use parser::TxType;
///
/// let result = TxType::from_str("invalid");
/// assert!(result.is_err());
/// ```
#[derive(Debug)]
pub struct TxTypeParseError(pub String);

impl fmt::Display for TxTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid transaction type: '{}'", self.0)
    }
}

impl Error for TxTypeParseError {}

/// Errors that occur when processing individual transaction data.
///
/// This error type represents various validation and parsing issues that can
/// occur when reading or constructing a transaction from raw data.
#[derive(Debug)]
pub enum TransactionError {
    /// The overall data format is invalid or unrecognized.
    InvalidDataFormat,

    /// The number of amount-related arguments is incorrect.
    ///
    /// Contains the actual number of arguments received.
    InvalidAmountArguments(usize),

    /// A specific field contains corrupted or invalid data.
    ///
    /// Contains the field name and the invalid value that was encountered.
    CorruptedField(String, String),

    /// A required field is missing from the transaction data.
    ///
    /// Contains the name of the missing field.
    MissingField(String),
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::InvalidDataFormat => {
                write!(f, "Invalid data format in transaction")
            }
            TransactionError::InvalidAmountArguments(count) => {
                write!(f, "Invalid number of amount arguments: got {}", count)
            }
            TransactionError::CorruptedField(field_name, given_value) => {
                write!(
                    f,
                    "Corrupted field '{}': invalid value '{}'",
                    field_name, given_value
                )
            }
            TransactionError::MissingField(field_name) => {
                write!(f, "Missing required field: '{}'", field_name)
            }
        }
    }
}

impl Error for TransactionError {}

/// Top-level error type for parsing transaction files.
///
/// This is the main error type returned by parsing operations and can wrap
/// lower-level errors from I/O operations or individual transaction processing.
#[derive(Debug)]
pub enum ParsingError {
    /// The file header is incorrect or missing.
    ///
    /// This typically means csv header or bin header of data is incorrect
    IncorrectHeader,

    /// An error occurred while processing a transaction.
    ///
    /// Wraps a [`TransactionError`] from individual transaction validation.
    TransactionError(TransactionError),

    /// An I/O error occurred while reading the file.
    ///
    /// Wraps the underlying [`std::io::Error`].
    Io(io::Error),

    /// The data format is invalid.
    ///
    /// This can occur when data packing to the csv/txt/bin was wrong
    InvalidDataFormat,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::IncorrectHeader => {
                write!(f, "Incorrect file header")
            }
            ParsingError::TransactionError(e) => {
                write!(f, "Transaction parsing error: {}", e)
            }
            ParsingError::Io(e) => {
                write!(f, "I/O error: {}", e)
            }
            ParsingError::InvalidDataFormat => {
                write!(f, "Invalid data format")
            }
        }
    }
}

impl Error for ParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParsingError::TransactionError(e) => Some(e),
            ParsingError::Io(e) => Some(e),
            _ => None,
        }
    }
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
