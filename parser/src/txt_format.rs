use crate::errors::{ParsingError, TransactionError};
use crate::{Parser, Status, Transaction, TransactionBuilder, TxType};

impl Parser {
    pub fn read_from_txt<R: std::io::BufRead>(r: &mut R) -> Result<Vec<Transaction>, ParsingError> {
        let mut txes = Vec::new();
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        let blocks = buf.split("\n\n");
        for block in blocks {
            if block.is_empty() {
                continue;
            };
            let tx = Transaction::try_from_txt_block(block.to_string())?;
            txes.push(tx);
        }
        Ok(txes)
    }

    pub fn write_to_txt<W: std::io::Write>(
        w: &mut W,
        txes: Vec<Transaction>,
    ) -> std::io::Result<()> {
        for (i, tx) in txes.iter().enumerate() {
            write!(w, "# Record {} ({})\n", i, tx.tx_type)?;
            write!(w, "{}\n", tx.to_txt_block())?;
        }
        Ok(())
    }
}

impl Transaction {
    fn try_from_txt_block(block: String) -> Result<Self, TransactionError> {
        let mut builder = TransactionBuilder::new();
        for row in block.split("\n") {
            if row.is_empty() || row.starts_with("#") {
                continue;
            }
            let values: Vec<&str> = row.split(": ").collect();
            if values.len() != 2 {
                return Err(TransactionError::InvalidDataFormat);
            }
            let key = values[0].trim().to_ascii_lowercase();
            let value = values[1].trim().to_string();

            match key.as_str() {
                "tx_id" => builder.tx_id_str(value)?,
                "tx_type" => builder.tx_type_str(value)?,
                "from_user_id" => builder.from_user_id_str(value)?,
                "to_user_id" => builder.to_user_id_str(value)?,
                "amount" => builder.amount_str(value)?,
                "timestamp" => builder.timestamp_str(value)?,
                "status" => builder.status_str(value)?,
                "description" => builder.description_str(value)?,
                _ => {} // Ignore unknown fields
            }
        }

        builder.build()
    }

    fn to_txt_block(&self) -> String {
        format!(
            "TX_ID: {}\n\
            TX_TYPE: {}\n\
            FROM_USER_ID: {}\n\
            TO_USER_ID: {}\n\
            AMOUNT: {}\n\
            TIMESTAMP: {}\n\
            STATUS: {}\n\
            DESCRIPTION: \"{}\"\n",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn test_read_from_txt_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/records_example.txt");
        let file = fs::File::open(d).expect("file could not be opened");
        let mut reader = std::io::BufReader::new(file);
        let txes = Parser::read_from_txt(&mut reader).expect("reading from csv gone wrong");
        assert!(txes.len() == 1000)
    }

    #[test]
    fn test_try_from_txt_block_basic() {
        let block = "TX_ID: 1234567890123456\n\
                     TX_TYPE: DEPOSIT\n\
                     FROM_USER_ID: 0\n\
                     TO_USER_ID: 9876543210987654\n\
                     AMOUNT: 10000\n\
                     TIMESTAMP: 1633036800000\n\
                     STATUS: SUCCESS\n\
                     DESCRIPTION: \"Terminal deposit\"";

        let result = Transaction::try_from_txt_block(block.to_string());
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.tx_id, 1234567890123456);
        assert_eq!(tx.from_user_id, 0);
        assert_eq!(tx.to_user_id, 9876543210987654);
        assert_eq!(tx.amount, 10000);
        assert_eq!(tx.timestamp, 1633036800000);
        assert_eq!(tx.description, "Terminal deposit");
    }

    #[test]
    fn test_try_from_txt_block_with_comments() {
        let block = "# This is a comment\n\
                     TX_ID: 123\n\
                     TX_TYPE: TRANSFER\n\
                     FROM_USER_ID: 100\n\
                     TO_USER_ID: 200\n\
                     AMOUNT: 500\n\
                     TIMESTAMP: 1633036800000\n\
                     STATUS: PENDING\n\
                     DESCRIPTION: \"Test transfer\"";

        let result = Transaction::try_from_txt_block(block.to_string());
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.tx_id, 123);
        assert_eq!(tx.description, "Test transfer");
    }

    #[test]
    fn test_try_from_txt_block_fields_any_order() {
        let block = "DESCRIPTION: \"Out of order\"\n\
                     AMOUNT: 999\n\
                     STATUS: FAILURE\n\
                     TX_ID: 456\n\
                     TIMESTAMP: 1633036800000\n\
                     TX_TYPE: WITHDRAWAL\n\
                     TO_USER_ID: 0\n\
                     FROM_USER_ID: 789";

        let result = Transaction::try_from_txt_block(block.to_string());
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.tx_id, 456);
        assert_eq!(tx.amount, 999);
        assert_eq!(tx.description, "Out of order");
    }

    #[test]
    fn test_try_from_txt_block_case_insensitive_keys() {
        let block = "tx_id: 100\n\
                     Tx_Type: deposit\n\
                     from_user_id: 0\n\
                     TO_USER_ID: 200\n\
                     AmOuNt: 500\n\
                     TIMESTAMP: 1633036800000\n\
                     status: success\n\
                     description: \"Mixed case\"";

        let result = Transaction::try_from_txt_block(block.to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_from_txt_block_missing_field() {
        let block = "TX_ID: 123\n\
                     TX_TYPE: DEPOSIT\n\
                     FROM_USER_ID: 0\n\
                     TO_USER_ID: 200\n\
                     AMOUNT: 500\n\
                     TIMESTAMP: 1633036800000\n\
                     STATUS: SUCCESS";

        let result = Transaction::try_from_txt_block(block.to_string());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TransactionError::MissingField(_)
        ));
    }

    #[test]
    fn test_try_from_txt_block_invalid_format() {
        let block = "TX_ID: 123\n\
                     INVALID LINE WITHOUT COLON\n\
                     TX_TYPE: DEPOSIT";

        let result = Transaction::try_from_txt_block(block.to_string());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TransactionError::InvalidDataFormat
        ));
    }

    #[test]
    fn test_to_txt_block_basic() {
        let tx = Transaction {
            tx_id: 1234567890123456,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 9876543210987654,
            amount: 10000,
            timestamp: 1633036800000,
            status: Status::Success,
            description: "Terminal deposit".to_string(),
        };

        let block = tx.to_txt_block();
        assert!(block.contains("TX_ID: 1234567890123456"));
        assert!(block.contains("TX_TYPE: DEPOSIT"));
        assert!(block.contains("FROM_USER_ID: 0"));
        assert!(block.contains("TO_USER_ID: 9876543210987654"));
        assert!(block.contains("AMOUNT: 10000"));
        assert!(block.contains("TIMESTAMP: 1633036800000"));
        assert!(block.contains("STATUS: success"));
        assert!(block.contains("DESCRIPTION: \"Terminal deposit\""));
    }

    #[test]
    fn test_txt_block_roundtrip() {
        let original = Transaction {
            tx_id: 999,
            tx_type: TxType::Transfer,
            from_user_id: 1000,
            to_user_id: 2000,
            amount: 15000,
            timestamp: 1111111111,
            status: Status::Pending,
            description: "Roundtrip test".to_string(),
        };

        let block = original.to_txt_block();
        let parsed = Transaction::try_from_txt_block(block).unwrap();

        assert_eq!(original.tx_id, parsed.tx_id);
        assert_eq!(original.from_user_id, parsed.from_user_id);
        assert_eq!(original.to_user_id, parsed.to_user_id);
        assert_eq!(original.amount, parsed.amount);
        assert_eq!(original.timestamp, parsed.timestamp);
        assert_eq!(original.description, parsed.description);
    }

    #[test]
    fn test_read_from_txt_multiple_blocks() {
        let txt_data = "TX_ID: 1\n\
                        TX_TYPE: DEPOSIT\n\
                        FROM_USER_ID: 0\n\
                        TO_USER_ID: 100\n\
                        AMOUNT: 1000\n\
                        TIMESTAMP: 1633036800000\n\
                        STATUS: SUCCESS\n\
                        DESCRIPTION: \"First\"\n\
                        \n\
                        TX_ID: 2\n\
                        TX_TYPE: WITHDRAWAL\n\
                        FROM_USER_ID: 200\n\
                        TO_USER_ID: 0\n\
                        AMOUNT: 500\n\
                        TIMESTAMP: 1633036900000\n\
                        STATUS: FAILURE\n\
                        DESCRIPTION: \"Second\"";

        let mut cursor = Cursor::new(txt_data);
        let result = Parser::read_from_txt(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].tx_id, 1);
        assert_eq!(transactions[1].tx_id, 2);
    }

    #[test]
    fn test_write_to_txt_single_transaction() {
        let mut buffer = Cursor::new(Vec::new());
        let transactions = vec![Transaction {
            tx_id: 1,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 100,
            amount: 1000,
            timestamp: 1633036800000,
            status: Status::Success,
            description: "Test deposit".to_string(),
        }];

        let result = Parser::write_to_txt(&mut buffer, transactions);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("# Record 0 (DEPOSIT)"));
        assert!(output.contains("TX_ID: 1"));
        assert!(output.contains("AMOUNT: 1000"));
        assert!(output.contains("DESCRIPTION: \"Test deposit\""));
    }

    #[test]
    fn test_write_to_txt_multiple_transactions() {
        let mut buffer = Cursor::new(Vec::new());
        let transactions = vec![
            Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 100,
                amount: 1000,
                timestamp: 1633036800000,
                status: Status::Success,
                description: "First".to_string(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Withdrawal,
                from_user_id: 200,
                to_user_id: 0,
                amount: 500,
                timestamp: 1633036900000,
                status: Status::Failure,
                description: "Second".to_string(),
            },
        ];

        let result = Parser::write_to_txt(&mut buffer, transactions);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer.into_inner()).unwrap();

        // Check for comments
        assert!(output.contains("# Record 0 (DEPOSIT)"));
        assert!(output.contains("# Record 1 (WITHDRAWAL)"));

        // Check that blocks are separated by empty line
        assert!(output.contains("\n\n"));

        // Split by double newline to verify blocks are separated
        let blocks: Vec<&str> = output.split("\n\n").filter(|s| !s.is_empty()).collect();
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_write_to_txt_empty_transactions() {
        let mut buffer = Cursor::new(Vec::new());
        let transactions: Vec<Transaction> = vec![];

        let result = Parser::write_to_txt(&mut buffer, transactions);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_write_then_read_txt_roundtrip() {
        let original_transactions = vec![
            Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 100,
                amount: 5000,
                timestamp: 1633036800000,
                status: Status::Success,
                description: "First transaction".to_string(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Transfer,
                from_user_id: 200,
                to_user_id: 300,
                amount: 2500,
                timestamp: 1633036900000,
                status: Status::Pending,
                description: "Second transaction".to_string(),
            },
        ];

        // Write to buffer
        let mut write_buffer = Cursor::new(Vec::new());
        Parser::write_to_txt(&mut write_buffer, original_transactions).unwrap();

        // Read back from buffer
        let mut read_buffer = Cursor::new(write_buffer.into_inner());
        let parsed_transactions = Parser::read_from_txt(&mut read_buffer).unwrap();

        assert_eq!(parsed_transactions.len(), 2);
        assert_eq!(parsed_transactions[0].tx_id, 1);
        assert_eq!(parsed_transactions[0].description, "First transaction");
        assert_eq!(parsed_transactions[1].tx_id, 2);
        assert_eq!(parsed_transactions[1].description, "Second transaction");
    }
}
