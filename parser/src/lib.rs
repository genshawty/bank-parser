pub mod csv_format;
pub mod errors;
use std::fmt;
use std::str::FromStr;

use crate::errors::TxTypeParseError;

#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

#[derive(Debug)]
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
            "transfer" => Ok(Self::Withdrawal),
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
}
