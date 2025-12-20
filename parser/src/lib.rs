pub mod csv_format;
pub mod errors;
use std::fmt;
use std::str::FromStr;

use crate::errors::TxTypeParseError;

#[derive(Debug, Clone)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
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
}
