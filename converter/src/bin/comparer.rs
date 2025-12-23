use clap::Parser;
use parser::{Parser as ParserTrait, Transaction};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use ypbank_converter::errors::ComparerArgsError;
use ypbank_converter::{Format, get_transactions};

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[arg(long)]
    pub file1: PathBuf,

    #[arg(long)]
    pub file2: PathBuf,

    #[arg(long)]
    pub format1: Format,

    #[arg(long)]
    pub format2: Format,
}

fn validate_args(args: Cli) -> Result<(), ComparerArgsError> {
    if !args.file1.exists() {
        return Err(ComparerArgsError::File1NotExists);
    } else if !args.file2.exists() {
        return Err(ComparerArgsError::File2NotExists);
    } else {
        match args.file1.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                if ext != format!("{:?}", args.format1).to_ascii_lowercase() {
                    return Err(ComparerArgsError::File1ExtensionMismatch(
                        ext.to_string(),
                        format!("{:?}", args.format1).to_ascii_lowercase(),
                    ));
                }
            }
            None => return Err(ComparerArgsError::File1IsNotFile),
        }

        match args.file2.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                if ext != format!("{:?}", args.format2).to_ascii_lowercase() {
                    return Err(ComparerArgsError::File2ExtensionMismatch(
                        ext.to_string(),
                        format!("{:?}", args.format2).to_ascii_lowercase(),
                    ));
                }
            }
            None => return Err(ComparerArgsError::File2IsNotFile),
        }
    }
    Ok(())
}

/// Returns the first mismatched transaction between two vectors.
///
/// Compares transactions element-wise and returns the first mismatch found:
/// - Uses `zip` to pair up elements from both vectors up to the shorter length
/// - `find` stops at the first pair where left != right
/// - If all paired elements match, checks if one vector is longer and returns the first extra element
///
/// Returns `None` if both vectors match perfectly (same length and all elements equal).
fn first_mismatch_transaction(
    tx_left: &Vec<Transaction>,
    tx_right: &Vec<Transaction>,
) -> Option<Transaction> {
    // Check pairs from both vectors for the first mismatch
    tx_left
        .iter()
        .zip(tx_right.iter())
        .find(|(left, right)| left != right)
        .map(|(left, _)| left.clone())
        // If no mismatch in common range, check for length difference
        .or_else(|| {
            let min_len = tx_left.len().min(tx_right.len());
            // Return first extra element from left if it's longer
            tx_left
                .get(min_len)
                .cloned()
                // Or return first extra element from right if it's longer
                .or_else(|| tx_right.get(min_len).cloned())
        })
}

fn main() {
    let args = Cli::parse();
    validate_args(args.clone()).expect("invalid arguments");

    let transactions_left = match get_transactions(args.file1, args.format1) {
        Ok(tx) => tx,
        Err(e) => panic!("error parsing transactions from file 1: {}", e),
    };

    let transactions_right = match get_transactions(args.file2, args.format2) {
        Ok(tx) => tx,
        Err(e) => panic!("error parsing transactions from file 1: {}", e),
    };
    let diff = first_mismatch_transaction(&transactions_left, &transactions_right);
    match diff {
        Some(tx) => {
            println!("Files are different, first mismatch: {}", tx)
        }
        None => {
            println!("Files are the same")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::{Status, TxType};

    /// Helper function to create a test transaction
    fn create_transaction(
        id: u64,
        tx_type: TxType,
        from: u64,
        to: u64,
        amount: u64,
        timestamp: u64,
        status: Status,
        desc: &str,
    ) -> Transaction {
        let mut builder = parser::builder::TransactionBuilder::new();
        builder.tx_id_str(id.to_string()).unwrap();
        builder.tx_type_str(format!("{}", tx_type)).unwrap();
        builder.from_user_id_str(from.to_string()).unwrap();
        builder.to_user_id_str(to.to_string()).unwrap();
        builder.amount_str(amount.to_string()).unwrap();
        builder.timestamp_str(timestamp.to_string()).unwrap();
        builder.status_str(format!("{}", status)).unwrap();
        builder.description_str(desc.to_string()).unwrap();
        builder.build().unwrap()
    }

    #[test]
    fn test_first_mismatch_empty_vectors() {
        let left: Vec<Transaction> = vec![];
        let right: Vec<Transaction> = vec![];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_none());
    }

    #[test]
    fn test_first_mismatch_identical_single() {
        let tx = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test",
        );
        let left = vec![tx.clone()];
        let right = vec![tx.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_none());
    }

    #[test]
    fn test_first_mismatch_identical_multiple() {
        let tx1 = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test1",
        );
        let tx2 = create_transaction(
            2,
            TxType::Transfer,
            200,
            300,
            2000,
            123457,
            Status::Pending,
            "test2",
        );
        let tx3 = create_transaction(
            3,
            TxType::Withdrawal,
            300,
            400,
            3000,
            123458,
            Status::Failure,
            "test3",
        );

        let left = vec![tx1.clone(), tx2.clone(), tx3.clone()];
        let right = vec![tx1.clone(), tx2.clone(), tx3.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_none());
    }

    #[test]
    fn test_first_mismatch_different_first_element() {
        let tx1 = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test1",
        );
        let tx2 = create_transaction(
            2,
            TxType::Transfer,
            200,
            300,
            2000,
            123457,
            Status::Pending,
            "test2",
        );

        let left = vec![tx1.clone()];
        let right = vec![tx2.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), tx1);
    }

    #[test]
    fn test_first_mismatch_different_second_element() {
        let tx1 = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test1",
        );
        let tx2 = create_transaction(
            2,
            TxType::Transfer,
            200,
            300,
            2000,
            123457,
            Status::Pending,
            "test2",
        );
        let tx3 = create_transaction(
            3,
            TxType::Withdrawal,
            300,
            400,
            3000,
            123458,
            Status::Failure,
            "test3",
        );

        let left = vec![tx1.clone(), tx2.clone()];
        let right = vec![tx1.clone(), tx3.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_some());
        // Should return the mismatched element from left (tx2)
        assert_eq!(result.unwrap(), tx2);
    }

    #[test]
    fn test_first_mismatch_left_longer() {
        let tx1 = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test1",
        );
        let tx2 = create_transaction(
            2,
            TxType::Transfer,
            200,
            300,
            2000,
            123457,
            Status::Pending,
            "test2",
        );

        let left = vec![tx1.clone(), tx2.clone()];
        let right = vec![tx1.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_some());
        // Should return the first extra element from left (tx2)
        assert_eq!(result.unwrap(), tx2);
    }

    #[test]
    fn test_first_mismatch_right_longer() {
        let tx1 = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test1",
        );
        let tx2 = create_transaction(
            2,
            TxType::Transfer,
            200,
            300,
            2000,
            123457,
            Status::Pending,
            "test2",
        );

        let left = vec![tx1.clone()];
        let right = vec![tx1.clone(), tx2.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_some());
        // Should return the first extra element from right (tx2)
        assert_eq!(result.unwrap(), tx2);
    }

    #[test]
    fn test_first_mismatch_left_empty_right_has_elements() {
        let tx = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test",
        );

        let left: Vec<Transaction> = vec![];
        let right = vec![tx.clone()];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), tx);
    }

    #[test]
    fn test_first_mismatch_right_empty_left_has_elements() {
        let tx = create_transaction(
            1,
            TxType::Deposit,
            100,
            200,
            1000,
            123456,
            Status::Success,
            "test",
        );

        let left = vec![tx.clone()];
        let right: Vec<Transaction> = vec![];

        let result = first_mismatch_transaction(&left, &right);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), tx);
    }
}
