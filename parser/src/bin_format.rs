use crate::errors::{ParsingError, TransactionError};
use crate::{Parser, Transaction, TransactionBuilder, TxType};

impl Parser {
    pub fn read_from_bin<R: std::io::BufRead>(r: &mut R) -> Result<Vec<Transaction>, ParsingError> {
        let mut txes = Vec::new();
        let mut header = [0; 8];
        while let Ok(_) = r.read_exact(&mut header) {
            if header[..4] != *b"YPBN" {
                return Err(ParsingError::InvalidDataFormat);
            }
            let data_size = u32::from_be_bytes(header[4..8].try_into()?);
            let mut data = vec![0u8; data_size as usize];
            r.read_exact(&mut data)?;
            let tx = Transaction::try_from_bin(&data)?;
            txes.push(tx);
        }
        Ok(txes)
    }

    pub fn write_to_bin<W: std::io::Write>(
        w: &mut W,
        txes: Vec<Transaction>,
    ) -> std::io::Result<()> {
        for (i, tx) in txes.iter().enumerate() {
            write!(w, "# Record {} ({})\n", i, tx.tx_type)?;
            write!(w, "{}\n", tx.to_bin())?;
        }
        Ok(())
    }
}

impl Transaction {
    pub fn try_from_bin(row: &Vec<u8>) -> Result<Self, TransactionError> {
        let mut builder = TransactionBuilder::new();
        builder.tx_id_byte(&row[..8])?;
        builder.tx_type_byte(&row[8..9])?;
        builder.from_user_id_byte(&row[9..17])?;
        builder.to_user_id_byte(&row[17..25])?;
        builder.amount_byte(&row[25..33])?;
        builder.timestamp_byte(&row[33..41])?;
        builder.status_byte(&row[41..42])?;
        builder.description_byte(&row[42..])?;

        builder.build()
    }

    fn to_bin(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Status, TxType};
    use std::io::BufReader;

    #[test]
    fn test_try_from_bin_deposit() {
        // Create a binary record for a deposit transaction
        let mut data = Vec::new();

        // TX_ID: 1000 (8 bytes, big-endian u64)
        data.extend_from_slice(&1000u64.to_be_bytes());

        // TX_TYPE: 0 = DEPOSIT (1 byte)
        data.push(0);

        // FROM_USER_ID: 0 for deposit (8 bytes)
        data.extend_from_slice(&0u64.to_be_bytes());

        // TO_USER_ID: 12345 (8 bytes)
        data.extend_from_slice(&12345u64.to_be_bytes());

        // AMOUNT: 500 (8 bytes, signed i64)
        data.extend_from_slice(&500i64.to_be_bytes());

        // TIMESTAMP: 1640000000000 (8 bytes)
        data.extend_from_slice(&1640000000000u64.to_be_bytes());

        // STATUS: 0 = SUCCESS (1 byte)
        data.push(0);

        // DESCRIPTION: "Test deposit" (4 bytes length + UTF-8 string)
        let desc = "Test deposit";
        data.extend_from_slice(&(desc.len() as u32).to_be_bytes());
        data.extend_from_slice(desc.as_bytes());

        let tx = Transaction::try_from_bin(&data).unwrap();

        assert_eq!(tx.tx_id, 1000);
        assert_eq!(tx.tx_type, TxType::Deposit);
        assert_eq!(tx.from_user_id, 0);
        assert_eq!(tx.to_user_id, 12345);
        assert_eq!(tx.amount, 500);
        assert_eq!(tx.timestamp, 1640000000000);
        assert_eq!(tx.status, Status::Success);
        assert_eq!(tx.description, "Test deposit");
    }

    #[test]
    fn test_try_from_bin_transfer() {
        let mut data = Vec::new();

        data.extend_from_slice(&2000u64.to_be_bytes());
        data.push(1); // TRANSFER
        data.extend_from_slice(&11111u64.to_be_bytes());
        data.extend_from_slice(&22222u64.to_be_bytes());
        data.extend_from_slice(&300i64.to_be_bytes());
        data.extend_from_slice(&1640000001000u64.to_be_bytes());
        data.push(1); // FAILURE

        let desc = "Failed transfer";
        data.extend_from_slice(&(desc.len() as u32).to_be_bytes());
        data.extend_from_slice(desc.as_bytes());

        let tx = Transaction::try_from_bin(&data).unwrap();

        assert_eq!(tx.tx_id, 2000);
        assert_eq!(tx.tx_type, TxType::Transfer);
        assert_eq!(tx.from_user_id, 11111);
        assert_eq!(tx.to_user_id, 22222);
        assert_eq!(tx.amount, 300);
        assert_eq!(tx.status, Status::Failure);
    }

    #[test]
    fn test_try_from_bin_withdrawal() {
        let mut data = Vec::new();

        data.extend_from_slice(&3000u64.to_be_bytes());
        data.push(2); // WITHDRAWAL
        data.extend_from_slice(&99999u64.to_be_bytes());
        data.extend_from_slice(&0u64.to_be_bytes());
        data.extend_from_slice(&(-150i64).to_be_bytes()); // Negative amount
        data.extend_from_slice(&1640000002000u64.to_be_bytes());
        data.push(2); // PENDING

        let desc = "ATM withdrawal";
        data.extend_from_slice(&(desc.len() as u32).to_be_bytes());
        data.extend_from_slice(desc.as_bytes());

        let tx = Transaction::try_from_bin(&data).unwrap();

        assert_eq!(tx.tx_id, 3000);
        assert_eq!(tx.tx_type, TxType::Withdrawal);
        assert_eq!(tx.from_user_id, 99999);
        assert_eq!(tx.to_user_id, 0);
        assert_eq!(tx.amount, 150); // Converted to absolute value
        assert_eq!(tx.status, Status::Pending);
    }

    #[test]
    fn test_try_from_bin_empty_description() {
        let mut data = Vec::new();

        data.extend_from_slice(&4000u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&0u64.to_be_bytes());
        data.extend_from_slice(&55555u64.to_be_bytes());
        data.extend_from_slice(&100i64.to_be_bytes());
        data.extend_from_slice(&1640000003000u64.to_be_bytes());
        data.push(0);

        // Empty description
        data.extend_from_slice(&0u32.to_be_bytes());

        let tx = Transaction::try_from_bin(&data).unwrap();

        assert_eq!(tx.description, "");
    }

    #[test]
    fn test_read_from_bin_file() {
        let file = std::fs::File::open("data/records_example.bin").unwrap();
        let mut reader = BufReader::new(file);

        let txes = Parser::read_from_bin(&mut reader).unwrap();

        // Check that we read multiple transactions
        assert!(txes.len() > 0, "Should read at least one transaction");

        // Print first transaction for debugging
        let first = &txes[0];
        println!("First transaction:");
        println!("  tx_id: {}", first.tx_id);
        println!("  tx_type: {:?}", first.tx_type);
        println!("  from_user_id: {}", first.from_user_id);
        println!("  to_user_id: {}", first.to_user_id);
        println!("  amount: {}", first.amount);
        println!("  timestamp: {}", first.timestamp);
        println!("  status: {:?}", first.status);
        println!("  description: {}", first.description);
        println!(
            "Successfully read {} transactions from binary file",
            txes.len()
        );

        // Basic validations
        assert!(first.tx_id > 0);
        assert!(first.timestamp > 0);
    }

}
