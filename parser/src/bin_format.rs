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
        for tx in txes.iter() {
            let data = tx.to_bin();
            // Write header: YPBN magic + record size
            w.write_all(b"YPBN")?;
            w.write_all(&(data.len() as u32).to_be_bytes())?;
            // Write body
            w.write_all(&data)?;
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

    fn to_bin(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.tx_id.to_be_bytes());
        data.push(self.tx_type.to_u8());
        data.extend_from_slice(&self.from_user_id.to_be_bytes());
        data.extend_from_slice(&self.to_user_id.to_be_bytes());
        // Convert u64 amount to i64 with proper sign based on tx_type
        // ATM NOT CONVERTING
        let amount_signed = match self.tx_type {
            TxType::Withdrawal => (self.amount as i64), // SHOULD be negative by specs, but now it isn't.
            _ => self.amount as i64,                    // Deposit or transfer: positive
        };
        data.extend_from_slice(&amount_signed.to_be_bytes());
        data.extend_from_slice(&self.timestamp.to_be_bytes());
        data.push(self.status.to_u8());
        let desc_bytes = self.description.as_bytes();
        data.extend_from_slice(&(desc_bytes.len() as u32).to_be_bytes());
        data.extend_from_slice(desc_bytes);

        data
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

    #[test]
    fn test_write_to_bin_single_transaction() {
        use std::io::Cursor;

        let tx = Transaction {
            tx_id: 1000,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 12345,
            amount: 500,
            timestamp: 1640000000000,
            status: Status::Success,
            description: "Test deposit".to_string(),
        };

        let mut buffer = Vec::new();
        Parser::write_to_bin(&mut buffer, vec![tx]).unwrap();

        // Verify the binary format
        assert_eq!(&buffer[0..4], b"YPBN"); // Magic
        let size = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        assert!(size > 0);

        // Read it back
        let mut cursor = Cursor::new(buffer);
        let txes = Parser::read_from_bin(&mut cursor).unwrap();
        assert_eq!(txes.len(), 1);
        assert_eq!(txes[0].tx_id, 1000);
        assert_eq!(txes[0].amount, 500);
    }

    #[test]
    fn test_write_to_bin_withdrawal_negative_amount() {
        use std::io::Cursor;

        // Create a withdrawal transaction (amount should be written as negative)
        let tx = Transaction {
            tx_id: 3000,
            tx_type: TxType::Withdrawal,
            from_user_id: 99999,
            to_user_id: 0,
            amount: 150,
            timestamp: 1640000002000,
            status: Status::Pending,
            description: "ATM withdrawal".to_string(),
        };

        let mut buffer = Vec::new();
        Parser::write_to_bin(&mut buffer, vec![tx]).unwrap();

        // Read the amount field directly from the binary (should be negative)
        // Skip header (8 bytes) + tx_id (8) + tx_type (1) + from_user_id (8) + to_user_id (8) = 33 bytes
        let amount_bytes = [
            buffer[33], buffer[34], buffer[35], buffer[36], buffer[37], buffer[38], buffer[39],
            buffer[40],
        ];
        let amount_signed = i64::from_be_bytes(amount_bytes);
        assert_eq!(amount_signed, -150); // Should be negative in binary

        // Read it back - should parse correctly
        let mut cursor = Cursor::new(buffer);
        let txes = Parser::read_from_bin(&mut cursor).unwrap();
        assert_eq!(txes.len(), 1);
        assert_eq!(txes[0].tx_type, TxType::Withdrawal);
        assert_eq!(txes[0].amount, 150); // Parsed as absolute value
    }

    #[test]
    fn test_write_then_read_bin_roundtrip() {
        use std::io::Cursor;

        let txes = vec![
            Transaction {
                tx_id: 1000,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 12345,
                amount: 500,
                timestamp: 1640000000000,
                status: Status::Success,
                description: "Deposit".to_string(),
            },
            Transaction {
                tx_id: 2000,
                tx_type: TxType::Transfer,
                from_user_id: 11111,
                to_user_id: 22222,
                amount: 300,
                timestamp: 1640000001000,
                status: Status::Failure,
                description: "Transfer".to_string(),
            },
            Transaction {
                tx_id: 3000,
                tx_type: TxType::Withdrawal,
                from_user_id: 99999,
                to_user_id: 0,
                amount: 150,
                timestamp: 1640000002000,
                status: Status::Pending,
                description: "".to_string(),
            },
        ];

        let mut buffer = Vec::new();
        Parser::write_to_bin(&mut buffer, txes.clone()).unwrap();

        let mut cursor = Cursor::new(buffer);
        let read_txes = Parser::read_from_bin(&mut cursor).unwrap();

        assert_eq!(read_txes.len(), 3);
        for (original, parsed) in txes.iter().zip(read_txes.iter()) {
            assert_eq!(original.tx_id, parsed.tx_id);
            assert_eq!(original.tx_type, parsed.tx_type);
            assert_eq!(original.from_user_id, parsed.from_user_id);
            assert_eq!(original.to_user_id, parsed.to_user_id);
            assert_eq!(original.amount, parsed.amount);
            assert_eq!(original.timestamp, parsed.timestamp);
            assert_eq!(original.status, parsed.status);
            assert_eq!(original.description, parsed.description);
        }
    }

    #[test]
    fn test_analyze_amount_sign_by_tx_type() {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open("data/records_example.bin").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut offset = 0;
        let mut record_num = 0;

        println!("\n=== Analyzing amount sign by transaction type ===");

        while offset + 8 <= buffer.len() {
            // Check magic
            if &buffer[offset..offset + 4] != b"YPBN" {
                break;
            }

            // Read record size
            let size = u32::from_be_bytes([
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]) as usize;

            offset += 8; // Skip header

            if offset + size > buffer.len() {
                break;
            }

            // Parse fields
            let tx_type_byte = buffer[offset + 8];
            let tx_type = match tx_type_byte {
                0 => "DEPOSIT",
                1 => "TRANSFER",
                2 => "WITHDRAWAL",
                _ => "UNKNOWN",
            };

            // Read amount as signed i64 (at offset 25 from start of data)
            let amount_bytes = [
                buffer[offset + 25],
                buffer[offset + 26],
                buffer[offset + 27],
                buffer[offset + 28],
                buffer[offset + 29],
                buffer[offset + 30],
                buffer[offset + 31],
                buffer[offset + 32],
            ];
            let amount_signed = i64::from_be_bytes(amount_bytes);

            if amount_signed < 0 {
                println!(
                    "Record {}: {} - amount: {} (NEGATIVE)",
                    record_num, tx_type, amount_signed
                );
            }

            offset += size;
            record_num += 1;
        }

        println!("Total records analyzed: {}", record_num);
    }
}
