use std::{array::TryFromSliceError, error::Error, fmt, io};

#[derive(Debug)]
pub struct TxTypeParseError(pub String);

impl fmt::Display for TxTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid transaction type: '{}'", self.0)
    }
}

impl Error for TxTypeParseError {}

#[derive(Debug)]
pub enum TransactionError {
    InvalidDataFormat,
    InvalidAmountArguments(usize),
    CorruptedField(String, String),
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

#[derive(Debug)]
pub enum ParsingError {
    IncorrectHeader,
    TransactionError(TransactionError),
    Io(io::Error),
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
