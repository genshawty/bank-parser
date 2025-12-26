//! CSV format parsing and writing for transactions.
//!
//! This module provides functionality to read and write transaction data in CSV format
//! with proper header validation and field encoding.

use crate::errors::{ParsingError, TransactionError};
use crate::{Parser, Transaction, TransactionBuilder};

const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

fn split_line(line: &str) -> Vec<String> {
    line.split(",")
        .map(|s| s.trim_matches('"').to_string())
        .collect()
}

impl Parser {
    /// Reads and parses transaction records from a CSV format.
    ///
    /// This function reads CSV data with the expected header format:
    /// `TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION`
    ///
    /// # Arguments
    ///
    /// * `r` - A mutable reference to any type implementing `BufRead` (e.g., `BufReader`, `Cursor`)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Transaction>)` - A vector of successfully parsed transactions
    /// * `Err(ParsingError)` - An error if the header is incorrect, the format is invalid, or any transaction fails to parse
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use parser::Parser;
    /// use std::fs::File;
    /// use std::io::BufReader;
    ///
    /// let file = File::open("transactions.csv").expect("Unable to open file");
    /// let mut reader = BufReader::new(file);
    /// let transactions = Parser::read_from_csv(&mut reader).expect("Failed to parse CSV");
    /// println!("Read {} transactions", transactions.len());
    /// ```
    pub fn read_from_csv<R: std::io::BufRead>(r: &mut R) -> Result<Vec<Transaction>, ParsingError> {
        let mut txes = Vec::new();
        let mut buf = String::new();
        // r.read_to_string(&mut buf)?;
        r.read_line(&mut buf)?;
        // let rows: Vec<&str> = buf.split("\n").collect();
        if buf.trim() != CSV_HEADER {
            eprintln!("{:?}", buf);
            return Err(ParsingError::IncorrectHeader);
        }
        loop {
            buf.clear();
            let n = r.read_line(&mut buf)?;
            if n == 0 {
                break; // EOF - no more data to read
            }
            if buf.trim().is_empty() {
                continue; // Empty line - skip it
            }
            let cells = split_line(&buf.trim());
            let tx = Transaction::try_from_csv_row(cells)?;
            txes.push(tx);
        }
        Ok(txes)
    }

    /// Writes a collection of transactions to CSV format.
    ///
    /// This function writes transactions with the CSV header:
    /// `TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION`
    /// Each transaction is written on a separate line with the description field enclosed in quotes.
    ///
    /// # Arguments
    ///
    /// * `w` - A mutable reference to any type implementing `Write` (e.g., `File`, `Vec<u8>`, `Cursor`)
    /// * `txes` - A vector of `Transaction` objects to write
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all transactions are successfully written
    /// * `Err(std::io::Error)` - If any write operation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use parser::{Parser, Transaction, TxType, Status};
    /// use std::fs::File;
    ///
    /// let transactions = vec![
    ///     Transaction {
    ///         tx_id: 1,
    ///         tx_type: TxType::Deposit,
    ///         from_user_id: 0,
    ///         to_user_id: 100,
    ///         amount: 5000,
    ///         timestamp: 1234567890,
    ///         status: Status::Success,
    ///         description: "Test deposit".to_string(),
    ///     }
    /// ];
    /// let mut file = File::create("output.csv").expect("Unable to create file");
    /// Parser::write_to_csv(&mut file, transactions).expect("Failed to write CSV");
    /// ```
    pub fn write_to_csv<W: std::io::Write>(
        w: &mut W,
        txes: Vec<Transaction>,
    ) -> std::io::Result<()> {
        write!(w, "{}\n", CSV_HEADER)?;
        for tx in txes {
            write!(w, "{}\n", tx.to_csv_row())?;
        }
        Ok(())
    }
}

impl Transaction {
    fn try_from_csv_row(cells: Vec<String>) -> Result<Self, TransactionError> {
        if cells.len() != 8 {
            return Err(TransactionError::InvalidAmountArguments(cells.len()));
        }

        let mut builder = TransactionBuilder::new();
        builder.tx_id_str(cells[0].clone())?;
        builder.tx_type_str(cells[1].clone())?;
        builder.from_user_id_str(cells[2].clone())?;
        builder.to_user_id_str(cells[3].clone())?;
        builder.amount_str(cells[4].clone())?;
        builder.timestamp_str(cells[5].clone())?;
        builder.status_str(cells[6].clone())?;
        builder.description_str(cells[7].clone())?;

        builder.build()
    }

    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},\"{}\"",
            self.tx_id,
            self.tx_type.clone(),
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status.clone(),
            self.description.clone()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    use crate::{Status, TxType};

    #[test]
    fn test_read_from_csv() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/records_example.csv");
        let file = fs::File::open(d).expect("file could not be opened");
        let mut reader = io::BufReader::new(file);
        let txes = Parser::read_from_csv(&mut reader).expect("reading from csv gone wrong");
        assert!(txes.len() == 1000)
    }

    #[test]
    fn test_parser_single_transaction_with_cursor() {
        // Note: DESCRIPTION field is enclosed in quotes as per CSV format
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,deposit,100,200,5000,1234567890,success,\"Initial deposit\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 1);
    }

    #[test]
    fn test_parser_multiple_transactions_with_cursor() {
        // Note: DESCRIPTION fields are enclosed in quotes
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,deposit,100,200,5000,1234567890,success,\"Initial deposit\"\n\
                        2,withdrawal,200,100,3000,1234567891,success,\"Withdrawal to account\"\n\
                        3,transfer,200,300,1500,1234567892,pending,\"Transfer funds\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 3);
    }

    #[test]
    fn test_parser_empty_lines_with_cursor() {
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,deposit,100,200,5000,1234567890,success,\"First\"\n\
                        \n\
                        2,withdrawal,200,100,3000,1234567891,success,\"Second\"\n\
                        \n\
                        3,transfer,200,300,1500,1234567892,pending,\"Third\"\n";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 3);
    }

    #[test]
    fn test_parser_incorrect_header() {
        let csv_data = "WRONG,HEADER,FORMAT\n\
                        1,deposit,100,200,5000,1234567890,success,\"Test\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParsingError::IncorrectHeader => {}
            _ => panic!("Expected IncorrectHeader error"),
        }
    }

    #[test]
    fn test_parser_case_insensitive_types() {
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,DEPOSIT,100,200,5000,1234567890,SUCCESS,\"Upper case\"\n\
                        2,Withdrawal,200,100,3000,1234567891,Failure,\"Mixed case\"\n\
                        3,transfer,200,300,1500,1234567892,PENDING,\"Lower case\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 3);
    }

    #[test]
    fn test_parser_description_with_quotes() {
        // Test that quotes are properly trimmed from DESCRIPTION field
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,deposit,100,200,5000,1234567890,success,\"Record number 1\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 1);
        // Description should have quotes removed by trim_matches('"')
    }

    #[test]
    fn test_transaction_invalid_field_count() {
        let cells = vec!["1".to_string(), "deposit".to_string(), "100".to_string()];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::InvalidAmountArguments(count) => assert_eq!(count, 3),
            _ => panic!("Expected InvalidAmountArguments error"),
        }
    }

    #[test]
    fn test_transaction_invalid_tx_id() {
        let cells = vec![
            "not_a_number".to_string(),
            "deposit".to_string(),
            "100".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "success".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "tx_id");
                assert_eq!(value, "not_a_number");
            }
            _ => panic!("Expected CorruptedField error for tx_id"),
        }
    }

    #[test]
    fn test_transaction_invalid_tx_type() {
        let cells = vec![
            "1".to_string(),
            "invalid_type".to_string(),
            "100".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "success".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "tx_type");
                assert_eq!(value, "invalid_type");
            }
            _ => panic!("Expected CorruptedField error for tx_type"),
        }
    }

    #[test]
    fn test_transaction_invalid_from_user_id() {
        let cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "invalid".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "success".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "from_user_id");
                assert_eq!(value, "invalid");
            }
            _ => panic!("Expected CorruptedField error for from_user_id"),
        }
    }

    #[test]
    fn test_transaction_invalid_to_user_id() {
        let cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "100".to_string(),
            "invalid".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "success".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "to_user_id");
                assert_eq!(value, "invalid");
            }
            _ => panic!("Expected CorruptedField error for to_user_id"),
        }
    }

    #[test]
    fn test_transaction_invalid_amount() {
        let cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "100".to_string(),
            "200".to_string(),
            "invalid".to_string(),
            "1234567890".to_string(),
            "success".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "amount");
                assert_eq!(value, "invalid");
            }
            _ => panic!("Expected CorruptedField error for amount"),
        }
    }

    #[test]
    fn test_transaction_invalid_timestamp() {
        let cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "100".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "invalid".to_string(),
            "success".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "timestamp");
                assert_eq!(value, "invalid");
            }
            _ => panic!("Expected CorruptedField error for timestamp"),
        }
    }

    #[test]
    fn test_transaction_invalid_status() {
        let cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "100".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "invalid_status".to_string(),
            "Test".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);

        assert!(result.is_err());
        match result.unwrap_err() {
            TransactionError::CorruptedField(field, value) => {
                assert_eq!(field, "tx_status");
                assert_eq!(value, "invalid_status");
            }
            _ => panic!("Expected CorruptedField error for status"),
        }
    }

    #[test]
    fn test_transaction_valid_all_types() {
        let deposit_cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "100".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "success".to_string(),
            "Deposit test".to_string(),
        ];
        let deposit_result = Transaction::try_from_csv_row(deposit_cells);
        assert!(deposit_result.is_ok());

        let withdrawal_cells = vec![
            "2".to_string(),
            "withdrawal".to_string(),
            "200".to_string(),
            "100".to_string(),
            "3000".to_string(),
            "1234567891".to_string(),
            "failure".to_string(),
            "Withdrawal test".to_string(),
        ];
        let withdrawal_result = Transaction::try_from_csv_row(withdrawal_cells);
        assert!(withdrawal_result.is_ok());

        let transfer_cells = vec![
            "3".to_string(),
            "transfer".to_string(),
            "200".to_string(),
            "300".to_string(),
            "1500".to_string(),
            "1234567892".to_string(),
            "pending".to_string(),
            "Transfer test".to_string(),
        ];
        let transfer_result = Transaction::try_from_csv_row(transfer_cells);
        assert!(transfer_result.is_ok());
    }

    #[test]
    fn test_transaction_with_whitespace() {
        let cells = vec![
            "1".to_string(),
            "  deposit  ".to_string(),
            "100".to_string(),
            "200".to_string(),
            "5000".to_string(),
            "1234567890".to_string(),
            "  success  ".to_string(),
            "Test with spaces".to_string(),
        ];
        let result = Transaction::try_from_csv_row(cells);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_only_header() {
        let csv_data =
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 0);
    }

    #[test]
    fn test_parser_malformed_row_in_middle() {
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,deposit,100,200,5000,1234567890,success,\"First\"\n\
                        2,invalid_type,200,100,3000,1234567891,success,\"Second\"\n\
                        3,transfer,200,300,1500,1234567892,pending,\"Third\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_err());
        match result.unwrap_err() {
            ParsingError::TransactionError(TransactionError::CorruptedField(field, _)) => {
                assert_eq!(field, "tx_type");
            }
            _ => panic!("Expected TransactionError::CorruptedField"),
        }
    }

    #[test]
    fn test_parser_with_real_format() {
        // Test with format matching the actual CSV file format
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"\n\
                        1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"\n\
                        1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,SUCCESS,\"Record number 3\"";
        let mut cursor = Cursor::new(csv_data);
        let result = Parser::read_from_csv(&mut cursor);

        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 3);
    }

    #[test]
    fn test_split_line_with_quotes() {
        // Test the split_line function directly to ensure quotes are properly handled
        let line = "1,deposit,100,200,5000,1234567890,success,\"Test description\"";
        let fields = split_line(line);

        assert_eq!(fields.len(), 8);
        assert_eq!(fields[7], "Test description"); // Quotes should be removed
    }

    #[test]
    fn test_split_line_without_quotes() {
        let line = "1,deposit,100,200,5000,1234567890,success,Test";
        let fields = split_line(line);

        assert_eq!(fields.len(), 8);
        assert_eq!(fields[7], "Test");
    }

    #[test]
    fn test_to_csv_row_basic() {
        let transaction = Transaction {
            tx_id: 1,
            tx_type: TxType::Deposit,
            from_user_id: 100,
            to_user_id: 200,
            amount: 5000,
            timestamp: 1234567890,
            status: Status::Success,
            description: "Test deposit".to_string(),
        };

        let csv_row = transaction.to_csv_row();
        assert_eq!(
            csv_row,
            "1,DEPOSIT,100,200,5000,1234567890,success,\"Test deposit\""
        );
    }

    #[test]
    fn test_to_csv_row_empty_description() {
        let transaction = Transaction {
            tx_id: 42,
            tx_type: TxType::Withdrawal,
            from_user_id: 500,
            to_user_id: 600,
            amount: 2500,
            timestamp: 9876543210,
            status: Status::Failure,
            description: "".to_string(),
        };

        let csv_row = transaction.to_csv_row();
        assert_eq!(
            csv_row,
            "42,WITHDRAWAL,500,600,2500,9876543210,failure,\"\""
        );
    }

    #[test]
    fn test_to_csv_row_description_with_special_chars() {
        let transaction = Transaction {
            tx_id: 123,
            tx_type: TxType::Transfer,
            from_user_id: 100,
            to_user_id: 200,
            amount: 5000,
            timestamp: 1234567890,
            status: Status::Pending,
            description: "Description with, commas and \"quotes\"".to_string(),
        };

        let csv_row = transaction.to_csv_row();
        assert_eq!(
            csv_row,
            "123,TRANSFER,100,200,5000,1234567890,pending,\"Description with, commas and \"quotes\"\""
        );
    }

    #[test]
    fn test_to_csv_row_roundtrip() {
        // Test that to_csv_row output can be parsed back with try_from_csv_row
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

        let csv_row = original.to_csv_row();
        let cells = split_line(&csv_row);
        let parsed = Transaction::try_from_csv_row(cells).unwrap();

        assert_eq!(original.tx_id, parsed.tx_id);
        assert_eq!(original.from_user_id, parsed.from_user_id);
        assert_eq!(original.to_user_id, parsed.to_user_id);
        assert_eq!(original.amount, parsed.amount);
        assert_eq!(original.timestamp, parsed.timestamp);
        assert_eq!(original.description, parsed.description);
    }

    #[test]
    fn test_write_to_csv_single_transaction() {
        let mut buffer = Cursor::new(Vec::new());
        let transactions = vec![Transaction {
            tx_id: 1,
            tx_type: TxType::Deposit,
            from_user_id: 100,
            to_user_id: 200,
            amount: 5000,
            timestamp: 1234567890,
            status: Status::Success,
            description: "Test deposit".to_string(),
        }];

        let result = Parser::write_to_csv(&mut buffer, transactions);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        let expected = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
                        1,DEPOSIT,100,200,5000,1234567890,success,\"Test deposit\"\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_write_to_csv_multiple_transactions() {
        let mut buffer = Cursor::new(Vec::new());
        let transactions = vec![
            Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 100,
                to_user_id: 200,
                amount: 5000,
                timestamp: 1234567890,
                status: Status::Success,
                description: "First".to_string(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Withdrawal,
                from_user_id: 200,
                to_user_id: 100,
                amount: 3000,
                timestamp: 1234567891,
                status: Status::Failure,
                description: "Second".to_string(),
            },
            Transaction {
                tx_id: 3,
                tx_type: TxType::Transfer,
                from_user_id: 100,
                to_user_id: 300,
                amount: 1500,
                timestamp: 1234567892,
                status: Status::Pending,
                description: "Third".to_string(),
            },
        ];

        let result = Parser::write_to_csv(&mut buffer, transactions);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        let lines: Vec<&str> = output.split('\n').collect();

        assert_eq!(
            lines[0],
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
        );
        assert_eq!(
            lines[1],
            "1,DEPOSIT,100,200,5000,1234567890,success,\"First\""
        );
        assert_eq!(
            lines[2],
            "2,WITHDRAWAL,200,100,3000,1234567891,failure,\"Second\""
        );
        assert_eq!(
            lines[3],
            "3,TRANSFER,100,300,1500,1234567892,pending,\"Third\""
        );
    }

    #[test]
    fn test_write_to_csv_empty_transactions() {
        let mut buffer = Cursor::new(Vec::new());
        let transactions: Vec<Transaction> = vec![];

        let result = Parser::write_to_csv(&mut buffer, transactions);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(
            output,
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n"
        );
    }

    #[test]
    fn test_write_then_read_roundtrip() {
        // Write transactions to CSV, then read them back
        let original_transactions = vec![
            Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 100,
                to_user_id: 200,
                amount: 5000,
                timestamp: 1234567890,
                status: Status::Success,
                description: "First transaction".to_string(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Transfer,
                from_user_id: 200,
                to_user_id: 300,
                amount: 2500,
                timestamp: 1234567891,
                status: Status::Pending,
                description: "Second transaction".to_string(),
            },
        ];

        // Write to buffer
        let mut write_buffer = Cursor::new(Vec::new());
        Parser::write_to_csv(&mut write_buffer, original_transactions).unwrap();

        // Read back from buffer
        let mut read_buffer = Cursor::new(write_buffer.into_inner());
        let parsed_transactions = Parser::read_from_csv(&mut read_buffer).unwrap();

        assert_eq!(parsed_transactions.len(), 2);
        assert_eq!(parsed_transactions[0].tx_id, 1);
        assert_eq!(parsed_transactions[0].description, "First transaction");
        assert_eq!(parsed_transactions[1].tx_id, 2);
        assert_eq!(parsed_transactions[1].description, "Second transaction");
    }
}
