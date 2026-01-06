//! Error types for transaction parsing operations.
//!
//! This module defines the error types that can occur during transaction parsing
//! and processing across different data formats.

use std::{array::TryFromSliceError, io};
use thiserror::Error;

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
#[derive(Debug, Error)]
#[error("Invalid transaction type: '{0}'")]
pub struct TxTypeParseError(pub String);

/// Errors that occur when processing individual transaction data.
///
/// This error type represents various validation and parsing issues that can
/// occur when reading or constructing a transaction from raw data.
#[derive(Debug, Error)]
pub enum TransactionError {
    /// The overall data format is invalid or unrecognized.
    #[error("Invalid data format in transaction")]
    InvalidDataFormat,

    /// The number of amount-related arguments is incorrect.
    /// Contains the actual number of arguments received.
    #[error("Invalid number of amount arguments: got {0}")]
    InvalidAmountArguments(usize),

    /// A specific field contains corrupted or invalid data.
    ///
    /// Contains the field name and the invalid value that was encountered.
    #[error("Corrupted field '{0}': invalid value '{1}'")]
    CorruptedField(String, String),

    /// A required field is missing from the transaction data.
    ///
    /// Contains the name of the missing field.
    #[error("Missing required field: '{0}'")]
    MissingField(String),
}

/// Top-level error type for parsing transaction files.
///
/// This is the main error type returned by parsing operations and can wrap
/// lower-level errors from I/O operations or individual transaction processing.
#[derive(Debug, Error)]
pub enum ParsingError {
    /// The file header is incorrect or missing.
    ///
    /// This typically means csv header or bin header of data is incorrect
    #[error("Incorrect file header")]
    IncorrectHeader,

    /// An error occurred while processing a transaction.
    ///
    /// Wraps a [`TransactionError`] from individual transaction validation.
    #[error("Transaction parsing error: {0}")]
    TransactionError(#[from] TransactionError),

    /// An I/O error occurred while reading the file.
    ///
    /// Wraps the underlying [`std::io::Error`].
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// The data format is invalid.
    ///
    /// This can occur when data packing to the csv/txt/bin was wrong
    #[error("Invalid data format")]
    InvalidDataFormat,
}

impl From<TryFromSliceError> for ParsingError {
    fn from(_: TryFromSliceError) -> Self {
        ParsingError::InvalidDataFormat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx_type_parse_error_display() {
        let error = TxTypeParseError("invalid_type".to_string());
        assert_eq!(
            format!("{}", error),
            "Invalid transaction type: 'invalid_type'"
        );
    }

    #[test]
    fn test_transaction_error_display() {
        let error1 = TransactionError::InvalidDataFormat;
        assert_eq!(format!("{}", error1), "Invalid data format in transaction");

        let error2 = TransactionError::InvalidAmountArguments(5);
        assert_eq!(
            format!("{}", error2),
            "Invalid number of amount arguments: got 5"
        );

        let error3 = TransactionError::CorruptedField("amount".to_string(), "abc".to_string());
        assert_eq!(
            format!("{}", error3),
            "Corrupted field 'amount': invalid value 'abc'"
        );

        let error4 = TransactionError::MissingField("tx_id".to_string());
        assert_eq!(format!("{}", error4), "Missing required field: 'tx_id'");
    }

    #[test]
    fn test_parsing_error_display() {
        let error1 = ParsingError::IncorrectHeader;
        assert_eq!(format!("{}", error1), "Incorrect file header");

        let error2 = ParsingError::InvalidDataFormat;
        assert_eq!(format!("{}", error2), "Invalid data format");

        let transaction_err = TransactionError::MissingField("tx_id".to_string());
        let error3 = ParsingError::TransactionError(transaction_err);
        assert_eq!(
            format!("{}", error3),
            "Transaction parsing error: Missing required field: 'tx_id'"
        );
    }

    #[test]
    fn test_parsing_error_from_transaction_error() {
        let transaction_err = TransactionError::InvalidAmountArguments(3);
        let parsing_err: ParsingError = transaction_err.into();
        assert_eq!(
            format!("{}", parsing_err),
            "Transaction parsing error: Invalid number of amount arguments: got 3"
        );
    }

    #[test]
    fn test_parsing_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let parsing_err: ParsingError = io_err.into();
        assert!(format!("{}", parsing_err).starts_with("I/O error:"));
        assert!(format!("{}", parsing_err).contains("file not found"));
    }
}
