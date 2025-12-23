use crate::errors::TransactionError;
use crate::{Status, Transaction, TxType};
use std::str::FromStr;

#[derive(Debug)]
pub struct TransactionBuilder {
    tx_id: Option<u64>,
    tx_type: Option<TxType>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<i64>,
    timestamp: Option<u64>,
    status: Option<Status>,
    description: Option<String>,
}

impl TransactionBuilder {
    /// Creates a new empty TransactionBuilder.
    ///
    /// All fields are initialized to `None` and must be set before calling `build()`.
    ///
    /// # Returns
    ///
    /// A new `TransactionBuilder` instance with all fields unset
    ///
    /// # Examples
    ///
    /// ```
    /// use parser::TransactionBuilder;
    ///
    /// let mut builder = TransactionBuilder::new();
    /// ```
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

    /// Sets the transaction ID from a string value.
    ///
    /// Parses the string as a u64 and stores it in the builder.
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the transaction ID
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string cannot be parsed as a valid u64
    pub fn tx_id_str(&mut self, val: String) -> Result<(), TransactionError> {
        let tx_id = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("tx_id".to_string(), val.clone()))?;
        self.tx_id = Some(tx_id);
        Ok(())
    }

    /// Sets the transaction ID from binary data.
    ///
    /// Decodes an 8-byte big-endian u64 from the byte slice.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the transaction ID (must be exactly 8 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 8 bytes
    pub fn tx_id_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        let tx_id = u64::from_be_bytes(val.try_into().map_err(|_| {
            TransactionError::CorruptedField(
                "tx_id".to_string(),
                format!("expected 8 bytes, got {}", val.len()),
            )
        })?);
        self.tx_id = Some(tx_id);
        Ok(())
    }

    /// Sets the transaction type from a string value.
    ///
    /// Parses strings like "deposit", "withdrawal", or "transfer" (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the transaction type
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string is not a valid transaction type
    pub fn tx_type_str(&mut self, val: String) -> Result<(), TransactionError> {
        let tx_type = TxType::from_str(&val)
            .map_err(|_| TransactionError::CorruptedField("tx_type".to_string(), val.clone()))?;
        self.tx_type = Some(tx_type);
        Ok(())
    }

    /// Sets the transaction type from binary data.
    ///
    /// Decodes a 1-byte value: 0=Deposit, 1=Transfer, 2=Withdrawal.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the transaction type (must be exactly 1 byte)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 1 byte or contains an invalid value
    pub fn tx_type_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        if val.len() != 1 {
            return Err(TransactionError::CorruptedField(
                "tx_type".to_string(),
                format!("expected 1 byte, got {}", val.len()),
            ));
        }
        let tx_type = TxType::from_u8(val[0]).map_err(|_| {
            TransactionError::CorruptedField("tx_type".to_string(), format!("{}", val[0]))
        })?;
        self.tx_type = Some(tx_type);
        Ok(())
    }

    /// Sets the source user ID from a string value.
    ///
    /// Parses the string as a u64 representing the user initiating the transaction.
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the from_user_id
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string cannot be parsed as a valid u64
    pub fn from_user_id_str(&mut self, val: String) -> Result<(), TransactionError> {
        let from_user_id = val.parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("from_user_id".to_string(), val.clone())
        })?;
        self.from_user_id = Some(from_user_id);
        Ok(())
    }

    /// Sets the source user ID from binary data.
    ///
    /// Decodes an 8-byte big-endian u64 from the byte slice.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the from_user_id (must be exactly 8 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 8 bytes
    pub fn from_user_id_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        let from_user_id = u64::from_be_bytes(val.try_into().map_err(|_| {
            TransactionError::CorruptedField(
                "from_user_id".to_string(),
                format!("expected 8 bytes, got {}", val.len()),
            )
        })?);
        self.from_user_id = Some(from_user_id);
        Ok(())
    }

    /// Sets the destination user ID from a string value.
    ///
    /// Parses the string as a u64 representing the user receiving the transaction.
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the to_user_id
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string cannot be parsed as a valid u64
    pub fn to_user_id_str(&mut self, val: String) -> Result<(), TransactionError> {
        let to_user_id = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("to_user_id".to_string(), val.clone()))?;
        self.to_user_id = Some(to_user_id);
        Ok(())
    }

    /// Sets the destination user ID from binary data.
    ///
    /// Decodes an 8-byte big-endian u64 from the byte slice.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the to_user_id (must be exactly 8 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 8 bytes
    pub fn to_user_id_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        let to_user_id = u64::from_be_bytes(val.try_into().map_err(|_| {
            TransactionError::CorruptedField(
                "to_user_id".to_string(),
                format!("expected 8 bytes, got {}", val.len()),
            )
        })?);
        self.to_user_id = Some(to_user_id);
        Ok(())
    }

    /// Sets the transaction amount from a string value.
    ///
    /// Parses the string as a u64 representing the transaction amount (always positive).
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the amount
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string cannot be parsed as a valid u64
    pub fn amount_str(&mut self, val: String) -> Result<(), TransactionError> {
        let amount = val
            .parse::<i64>()
            .map_err(|_| TransactionError::CorruptedField("amount".to_string(), val.clone()))?;
        self.amount = Some(amount);
        Ok(())
    }

    /// Sets the transaction amount from binary data.
    ///
    /// Decodes an 8-byte big-endian i64 and converts it to absolute value (u64).
    /// This handles negative amounts in binary format (e.g., for withdrawals).
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the amount (must be exactly 8 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 8 bytes
    pub fn amount_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        let amount = i64::from_be_bytes(val.try_into().map_err(|_| {
            TransactionError::CorruptedField(
                "amount".to_string(),
                format!("expected 8 bytes, got {}", val.len()),
            )
        })?);
        // in other formats amount is >=0
        // so here i convert in to also be u64
        self.amount = Some(amount);
        Ok(())
    }

    /// Sets the transaction timestamp from a string value.
    ///
    /// Parses the string as a u64 representing the Unix timestamp in milliseconds.
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the timestamp
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string cannot be parsed as a valid u64
    pub fn timestamp_str(&mut self, val: String) -> Result<(), TransactionError> {
        let timestamp = val
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("timestamp".to_string(), val.clone()))?;
        self.timestamp = Some(timestamp);
        Ok(())
    }

    /// Sets the transaction timestamp from binary data.
    ///
    /// Decodes an 8-byte big-endian u64 from the byte slice.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the timestamp (must be exactly 8 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 8 bytes
    pub fn timestamp_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        let timestamp = u64::from_be_bytes(val.try_into().map_err(|_| {
            TransactionError::CorruptedField(
                "timestamp".to_string(),
                format!("expected 8 bytes, got {}", val.len()),
            )
        })?);
        self.timestamp = Some(timestamp);
        Ok(())
    }

    /// Sets the transaction status from a string value.
    ///
    /// Parses strings like "success", "failure", or "pending" (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `val` - String representation of the transaction status
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully parsed and set
    /// * `Err(TransactionError)` - If the string is not a valid status
    pub fn status_str(&mut self, val: String) -> Result<(), TransactionError> {
        let status = Status::from_str(&val)
            .map_err(|_| TransactionError::CorruptedField("tx_status".to_string(), val.clone()))?;
        self.status = Some(status);
        Ok(())
    }

    /// Sets the transaction status from binary data.
    ///
    /// Decodes a 1-byte value: 0=Success, 1=Failure, 2=Pending.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the status (must be exactly 1 byte)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the byte slice is not exactly 1 byte or contains an invalid value
    pub fn status_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        if val.len() != 1 {
            return Err(TransactionError::CorruptedField(
                "status".to_string(),
                format!("expected 1 byte, got {}", val.len()),
            ));
        }
        let status = Status::from_u8(val[0]).map_err(|_| {
            TransactionError::CorruptedField("status".to_string(), format!("{}", val[0]))
        })?;
        self.status = Some(status);
        Ok(())
    }

    /// Sets the transaction description from a string value.
    ///
    /// Automatically trims surrounding quotes if present.
    ///
    /// # Arguments
    ///
    /// * `val` - String containing the transaction description
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Always succeeds
    pub fn description_str(&mut self, val: String) -> Result<(), TransactionError> {
        let description = val.trim_matches('"').to_string();
        self.description = Some(description);
        Ok(())
    }

    /// Sets the transaction description from binary data.
    ///
    /// Decodes a length-prefixed UTF-8 string. The first 4 bytes contain the length
    /// as a big-endian u32, followed by that many bytes of UTF-8 text.
    ///
    /// # Arguments
    ///
    /// * `val` - Byte slice containing the length prefix and description text
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the value was successfully decoded and set
    /// * `Err(TransactionError)` - If the data is malformed or not valid UTF-8
    pub fn description_byte(&mut self, val: &[u8]) -> Result<(), TransactionError> {
        let desc_len = u32::from_be_bytes(val[..4].try_into().map_err(|_| {
            TransactionError::CorruptedField(
                "desc_len".to_string(),
                format!("expected 4 bytes, got {}", val.len()),
            )
        })?);
        if desc_len == 0 {
            return self.description_str("".to_string());
        }
        let description = str::from_utf8(&val[4..]).map_err(|_| {
            TransactionError::CorruptedField(
                "desc_len".to_string(),
                format!("expected 4 bytes, got {}", val.len()),
            )
        })?;
        self.description = Some(description.to_string());
        Ok(())
    }

    /// Consumes the builder and constructs a Transaction.
    ///
    /// All fields must be set before calling this method, otherwise it will return an error
    /// indicating which field is missing.
    ///
    /// # Returns
    ///
    /// * `Ok(Transaction)` - If all fields are set and the transaction is successfully built
    /// * `Err(TransactionError)` - If any required field is missing
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use parser::TransactionBuilder;
    ///
    /// let mut builder = TransactionBuilder::new();
    /// builder.tx_id_str("123".to_string()).unwrap();
    /// builder.tx_type_str("deposit".to_string()).unwrap();
    /// builder.from_user_id_str("0".to_string()).unwrap();
    /// builder.to_user_id_str("100".to_string()).unwrap();
    /// builder.amount_str("5000".to_string()).unwrap();
    /// builder.timestamp_str("1234567890".to_string()).unwrap();
    /// builder.status_str("success".to_string()).unwrap();
    /// builder.description_str("Test".to_string()).unwrap();
    ///
    /// let transaction = builder.build().expect("Failed to build transaction");
    /// ```
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
