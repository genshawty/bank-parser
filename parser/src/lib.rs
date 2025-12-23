pub mod bin_format;
pub mod builder;
pub mod csv_format;
pub mod errors;
pub mod txt_format;
use builder::TransactionBuilder;
use std::fmt;
use std::str::FromStr;

use crate::errors::TxTypeParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

impl FromStr for TxType {
    type Err = TxTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "deposit" => Ok(Self::Deposit),
            "withdrawal" => Ok(Self::Withdrawal),
            "transfer" => Ok(Self::Transfer),
            _ => Err(TxTypeParseError(s.to_string())),
        }
    }
}

impl TxType {
    /// Converts a byte value to a transaction type.
    ///
    /// # Arguments
    /// * `byte` - The byte value to convert (0 = Deposit, 1 = Transfer, 2 = Withdrawal)
    ///
    /// # Returns
    /// * `Ok(TxType)` - The corresponding transaction type
    /// * `Err(TxTypeParseError)` - If the byte value is invalid (> 2)
    ///
    /// # Examples
    /// ```
    /// use parser::TxType;
    ///
    /// assert_eq!(TxType::from_u8(0).unwrap(), TxType::Deposit);
    /// assert_eq!(TxType::from_u8(1).unwrap(), TxType::Transfer);
    /// assert_eq!(TxType::from_u8(2).unwrap(), TxType::Withdrawal);
    /// assert!(TxType::from_u8(3).is_err());
    /// ```
    pub fn from_u8(byte: u8) -> Result<Self, TxTypeParseError> {
        match byte {
            0 => Ok(TxType::Deposit),
            1 => Ok(TxType::Transfer),
            2 => Ok(TxType::Withdrawal),
            _ => Err(TxTypeParseError(format!("invalid byte value: {}", byte))),
        }
    }

    /// Converts a transaction type to its byte representation.
    ///
    /// # Returns
    /// The byte value (0 = Deposit, 1 = Transfer, 2 = Withdrawal)
    ///
    /// # Examples
    /// ```
    /// use parser::TxType;
    ///
    /// assert_eq!(TxType::Deposit.to_u8(), 0);
    /// assert_eq!(TxType::Transfer.to_u8(), 1);
    /// assert_eq!(TxType::Withdrawal.to_u8(), 2);
    /// ```
    pub fn to_u8(&self) -> u8 {
        match self {
            TxType::Deposit => 0,
            TxType::Transfer => 1,
            TxType::Withdrawal => 2,
        }
    }
}

impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TxType::Deposit => "DEPOSIT",
            TxType::Transfer => "TRANSFER",
            TxType::Withdrawal => "WITHDRAWAL",
        };
        f.write_str(s)
    }
}

impl FromStr for Status {
    type Err = TxTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "success" => Ok(Self::Success),
            "failure" => Ok(Self::Failure),
            "pending" => Ok(Self::Pending),
            _ => Err(TxTypeParseError(s.to_string())),
        }
    }
}

impl Status {
    /// Converts a byte value to a transaction status.
    ///
    /// # Arguments
    /// * `byte` - The byte value to convert (0 = Success, 1 = Failure, 2 = Pending)
    ///
    /// # Returns
    /// * `Ok(Status)` - The corresponding status
    /// * `Err(TxTypeParseError)` - If the byte value is invalid (> 2)
    ///
    /// # Examples
    /// ```
    /// use parser::Status;
    ///
    /// assert_eq!(Status::from_u8(0).unwrap(), Status::Success);
    /// assert_eq!(Status::from_u8(1).unwrap(), Status::Failure);
    /// assert_eq!(Status::from_u8(2).unwrap(), Status::Pending);
    /// assert!(Status::from_u8(3).is_err());
    /// ```
    pub fn from_u8(byte: u8) -> Result<Self, TxTypeParseError> {
        match byte {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(TxTypeParseError(format!("invalid byte value: {}", byte))),
        }
    }

    /// Converts a status to its byte representation.
    ///
    /// # Returns
    /// The byte value (0 = Success, 1 = Failure, 2 = Pending)
    ///
    /// # Examples
    /// ```
    /// use parser::Status;
    ///
    /// assert_eq!(Status::Success.to_u8(), 0);
    /// assert_eq!(Status::Failure.to_u8(), 1);
    /// assert_eq!(Status::Pending.to_u8(), 2);
    /// ```
    pub fn to_u8(&self) -> u8 {
        match self {
            Status::Success => 0,
            Status::Failure => 1,
            Status::Pending => 2,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Success => "success",
            Status::Failure => "failure",
            Status::Pending => "pending",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    tx_id: u64,
    tx_type: TxType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    status: Status,
    description: String,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_txt_block())
    }
}

pub struct Parser {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txtype_from_str_valid() {
        assert!(matches!(
            "deposit".parse::<TxType>().unwrap(),
            TxType::Deposit
        ));
        assert!(matches!(
            "Deposit".parse::<TxType>().unwrap(),
            TxType::Deposit
        ));
        assert!(matches!(
            "DEPOSIT".parse::<TxType>().unwrap(),
            TxType::Deposit
        ));
        assert!(matches!(
            "  deposit  ".parse::<TxType>().unwrap(),
            TxType::Deposit
        ));

        assert!(matches!(
            "withdrawal".parse::<TxType>().unwrap(),
            TxType::Withdrawal
        ));
        assert!(matches!(
            "Withdrawal".parse::<TxType>().unwrap(),
            TxType::Withdrawal
        ));
        assert!(matches!(
            "WITHDRAWAL".parse::<TxType>().unwrap(),
            TxType::Withdrawal
        ));

        assert!(matches!(
            "transfer".parse::<TxType>().unwrap(),
            TxType::Transfer
        ));
        assert!(matches!(
            "Transfer".parse::<TxType>().unwrap(),
            TxType::Transfer
        ));
        assert!(matches!(
            "TRANSFER".parse::<TxType>().unwrap(),
            TxType::Transfer
        ));
    }

    #[test]
    fn test_txtype_from_str_invalid() {
        assert!("invalid".parse::<TxType>().is_err());
        assert!("".parse::<TxType>().is_err());
        assert!("depos".parse::<TxType>().is_err());
        assert!("withdrawall".parse::<TxType>().is_err());
    }

    #[test]
    fn test_txtype_display() {
        assert_eq!(TxType::Deposit.to_string(), "DEPOSIT");
        assert_eq!(TxType::Withdrawal.to_string(), "WITHDRAWAL");
        assert_eq!(TxType::Transfer.to_string(), "TRANSFER");
    }

    #[test]
    fn test_status_from_str_valid() {
        assert!(matches!(
            "success".parse::<Status>().unwrap(),
            Status::Success
        ));
        assert!(matches!(
            "Success".parse::<Status>().unwrap(),
            Status::Success
        ));
        assert!(matches!(
            "SUCCESS".parse::<Status>().unwrap(),
            Status::Success
        ));
        assert!(matches!(
            "  success  ".parse::<Status>().unwrap(),
            Status::Success
        ));

        assert!(matches!(
            "failure".parse::<Status>().unwrap(),
            Status::Failure
        ));
        assert!(matches!(
            "Failure".parse::<Status>().unwrap(),
            Status::Failure
        ));
        assert!(matches!(
            "FAILURE".parse::<Status>().unwrap(),
            Status::Failure
        ));

        assert!(matches!(
            "pending".parse::<Status>().unwrap(),
            Status::Pending
        ));
        assert!(matches!(
            "Pending".parse::<Status>().unwrap(),
            Status::Pending
        ));
        assert!(matches!(
            "PENDING".parse::<Status>().unwrap(),
            Status::Pending
        ));
    }

    #[test]
    fn test_status_from_str_invalid() {
        assert!("invalid".parse::<Status>().is_err());
        assert!("".parse::<Status>().is_err());
        assert!("sucess".parse::<Status>().is_err());
        assert!("fail".parse::<Status>().is_err());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Success.to_string(), "success");
        assert_eq!(Status::Failure.to_string(), "failure");
        assert_eq!(Status::Pending.to_string(), "pending");
    }

    #[test]
    fn test_txtype_from_u8_valid() {
        assert_eq!(TxType::from_u8(0).unwrap(), TxType::Deposit);
        assert_eq!(TxType::from_u8(1).unwrap(), TxType::Transfer);
        assert_eq!(TxType::from_u8(2).unwrap(), TxType::Withdrawal);
    }

    #[test]
    fn test_txtype_from_u8_invalid() {
        assert!(TxType::from_u8(3).is_err());
        assert!(TxType::from_u8(99).is_err());
        assert!(TxType::from_u8(255).is_err());
    }

    #[test]
    fn test_txtype_to_u8() {
        assert_eq!(TxType::Deposit.to_u8(), 0);
        assert_eq!(TxType::Transfer.to_u8(), 1);
        assert_eq!(TxType::Withdrawal.to_u8(), 2);
    }

    #[test]
    fn test_status_from_u8_valid() {
        assert_eq!(Status::from_u8(0).unwrap(), Status::Success);
        assert_eq!(Status::from_u8(1).unwrap(), Status::Failure);
        assert_eq!(Status::from_u8(2).unwrap(), Status::Pending);
    }

    #[test]
    fn test_status_from_u8_invalid() {
        assert!(Status::from_u8(3).is_err());
        assert!(Status::from_u8(99).is_err());
        assert!(Status::from_u8(255).is_err());
    }

    #[test]
    fn test_status_to_u8() {
        assert_eq!(Status::Success.to_u8(), 0);
        assert_eq!(Status::Failure.to_u8(), 1);
        assert_eq!(Status::Pending.to_u8(), 2);
    }

    #[test]
    fn test_txtype_u8_roundtrip() {
        assert_eq!(
            TxType::from_u8(TxType::Deposit.to_u8()).unwrap(),
            TxType::Deposit
        );
        assert_eq!(
            TxType::from_u8(TxType::Transfer.to_u8()).unwrap(),
            TxType::Transfer
        );
        assert_eq!(
            TxType::from_u8(TxType::Withdrawal.to_u8()).unwrap(),
            TxType::Withdrawal
        );
    }

    #[test]
    fn test_status_u8_roundtrip() {
        assert_eq!(
            Status::from_u8(Status::Success.to_u8()).unwrap(),
            Status::Success
        );
        assert_eq!(
            Status::from_u8(Status::Failure.to_u8()).unwrap(),
            Status::Failure
        );
        assert_eq!(
            Status::from_u8(Status::Pending.to_u8()).unwrap(),
            Status::Pending
        );
    }
}
