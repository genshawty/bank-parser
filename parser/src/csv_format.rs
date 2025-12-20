use std::cell;
use std::str::FromStr;

use crate::errors::{ParsingError, TransactionError};
use crate::{Parser, Status, Transaction, TxType};

const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

fn split_line(line: &str) -> Vec<String> {
    line.split(",")
        .map(|s| s.trim_matches('"').to_string())
        .collect()
}

impl Parser {
    pub fn read_from_csv<R: std::io::BufRead>(r: &mut R) -> Result<Vec<Transaction>, ParsingError> {
        let mut txes = Vec::new();
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        let rows: Vec<&str> = buf.split("\n").collect();
        if rows[0] != CSV_HEADER {
            eprintln!("{:?}", rows[0]);
            return Err(ParsingError::IncorrectHeader);
        }
        for row in &rows[1..] {
            if row.is_empty() {
                continue;
            };
            let cells = split_line(row);
            // println!("{:?}", cells);
            let tx = Transaction::try_from_csv_row(cells)?;
            txes.push(tx);
        }
        Ok(txes)
    }
}

impl Transaction {
    fn try_from_csv_row(cells: Vec<String>) -> Result<Self, TransactionError> {
        if cells.len() != 8 {
            return Err(TransactionError::InvalidAmountArguments(cells.len()));
        }
        let tx_id = cells[0].parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("tx_id".to_string(), cells[0].clone())
        })?;
        let tx_type = TxType::from_str(&cells[1]).map_err(|_| {
            TransactionError::CorruptedField("tx_type".to_string(), cells[1].clone())
        })?;
        let from_user_id = cells[2].parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("from_user_id".to_string(), cells[2].clone())
        })?;
        let to_user_id = cells[3].parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("to_user_id".to_string(), cells[3].clone())
        })?;
        let amount = cells[4].parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("amount".to_string(), cells[4].clone())
        })?;
        let timestamp = cells[5].parse::<u64>().map_err(|_| {
            TransactionError::CorruptedField("timestamp".to_string(), cells[5].clone())
        })?;
        let status = Status::from_str(&cells[6]).map_err(|_| {
            TransactionError::CorruptedField("tx_status".to_string(), cells[6].clone())
        })?;
        let description = &cells[7];

        Ok(Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description: description.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn test_read_from_csv() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/records_example.csv");
        // println!("{}", d.display());
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
            ParsingError::IncorrectHeader => {},
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
        let cells = vec![
            "1".to_string(),
            "deposit".to_string(),
            "100".to_string(),
        ];
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
            },
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
            },
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
            },
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
            },
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
            },
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
            },
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
            },
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
        let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n";
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
            },
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
}
