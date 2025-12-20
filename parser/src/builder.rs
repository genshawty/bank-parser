use crate::errors::TransactionError;
use crate::{Status, Transaction, TxType};
use std::str::FromStr;

#[derive(Debug)]
pub struct TransactionBuilder {
    tx_id: Option<u64>,
    tx_type: Option<TxType>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<u64>,
    timestamp: Option<u64>,
    status: Option<Status>,
    description: Option<String>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder {
            tx_id: None,
            tx_type: None,
            from_user_id: None,
            to_user_id: None,
            amount: None,
            timestamp: None,
            status: None,
            description: None,
        }
    }

    pub fn tx_id_str(&mut self, val: String) -> Result<(), TransactionError> {
        let tx_id = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("tx_id".to_string(), val.clone()))?;
        self.tx_id = Some(tx_id);
        Ok(())
    }

    pub fn tx_type_str(&mut self, val: String) -> Result<(), TransactionError> {
        let tx_type = TxType::from_str(&val)
            .map_err(|_| TransactionError::CorruptedField("tx_type".to_string(), val.clone()))?;
        self.tx_type = Some(tx_type);
        Ok(())
    }

    pub fn from_user_id_str(&mut self, val: String) -> Result<(), TransactionError> {
        let from_user_id = val.parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("from_user_id".to_string(), val.clone())
        })?;
        self.from_user_id = Some(from_user_id);
        Ok(())
    }

    pub fn to_user_id_str(&mut self, val: String) -> Result<(), TransactionError> {
        let to_user_id = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("to_user_id".to_string(), val.clone()))?;
        self.to_user_id = Some(to_user_id);
        Ok(())
    }

    pub fn amount_str(&mut self, val: String) -> Result<(), TransactionError> {
        let amount = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("amount".to_string(), val.clone()))?;
        self.amount = Some(amount);
        Ok(())
    }

    pub fn timestamp_str(&mut self, val: String) -> Result<(), TransactionError> {
        let timestamp = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("timestamp".to_string(), val.clone()))?;
        self.timestamp = Some(timestamp);
        Ok(())
    }

    pub fn status_str(&mut self, val: String) -> Result<(), TransactionError> {
        let status = Status::from_str(&val)
            .map_err(|_| TransactionError::CorruptedField("tx_status".to_string(), val.clone()))?;
        self.status = Some(status);
        Ok(())
    }

    pub fn description_str(&mut self, val: String) -> Result<(), TransactionError> {
        self.description = Some(val);
        Ok(())
    }

    pub fn build(self) -> Result<Transaction, TransactionError> {
        Ok(Transaction {
            tx_id: self
                .tx_id
                .ok_or_else(|| TransactionError::MissingField("tx_id".to_string()))?,
            tx_type: self
                .tx_type
                .ok_or_else(|| TransactionError::MissingField("tx_type".to_string()))?,
            from_user_id: self
                .from_user_id
                .ok_or_else(|| TransactionError::MissingField("from_user_id".to_string()))?,
            to_user_id: self
                .to_user_id
                .ok_or_else(|| TransactionError::MissingField("to_user_id".to_string()))?,
            amount: self
                .amount
                .ok_or_else(|| TransactionError::MissingField("amount".to_string()))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| TransactionError::MissingField("timestamp".to_string()))?,
            status: self
                .status
                .ok_or_else(|| TransactionError::MissingField("status".to_string()))?,
            description: self
                .description
                .ok_or_else(|| TransactionError::MissingField("description".to_string()))?,
        })
    }
}
