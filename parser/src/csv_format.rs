use std::cell;

use crate::errors::{ParsingError, TransactionError};
use crate::{Parser, Transaction};

const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

fn split_line(line: &str) -> Vec<&str> {
    line.split(",").collect()
}

impl Parser {
    pub fn read_from_csv<R: std::io::BufRead>(r: &mut R) -> Result<Vec<Transaction>, ParsingError> {
        let mut txes = Vec::new();
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        let rows: Vec<&str> = buf.split(",\n").collect();
        if rows[0] != CSV_HEADER {
            return Err(ParsingError::IncorrectHeader);
        }
        for row in rows {
            let cells = split_line(row);
            let tx = Transaction::try_from_csv_row(cells)?;
            txes.push(tx);
        }
        Ok(txes)
    }
}

impl Transaction {
    fn try_from_csv_row(cells: Vec<&str>) -> Result<Self, TransactionError> {
        let tx_id = cells[0]
            .parse::<u64>()
            .map_err(|_| TransactionError::CorruptedField("tx_id".to_string()))?;
        let tx_type = cells[1].to_string();

        Ok(Transaction {})
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_read_from_csv() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("data/records_example.csv");
        println!("{}", d.display());
        let file = fs::File::open(d).expect("file could not be opened");
        let mut reader = io::BufReader::new(file);
        let txes = Parser::read_from_csv(&mut reader).expect("reading from csv gone wrong");
    }
}
