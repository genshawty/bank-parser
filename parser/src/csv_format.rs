use std::cell;

use crate::errors::{ParsingError, TransactionError};
use crate::{Parser, Transaction};

fn split_line(line: &str) -> Vec<&str> {
    line.split(",").collect()
}

impl Parser {
    pub fn read_from_csv<R: std::io::BufRead>(r: &mut R) -> Result<Vec<Transaction>, ParsingError> {
        let mut txes = Vec::new();
        let mut buf = String::new();
        r.read_to_string(&mut buf)?;
        let rows = buf.split(",\n");
        let mut has_started = false;
        for row in rows {
            let cells = split_line(row);
            // searching for 'Дебет' in 5th cell
            if !has_started {
                if cells.len() > 4 && cells[4] == "Дебет" {
                    has_started = true;
                }
                continue;
            }
            if cells[1] == "" {
                break;
            }
            let tx = Transaction::try_from_csv_row(cells)?;
            txes.push(tx);
        }
        Ok(txes)
    }
}

impl Transaction {
    fn try_from_csv_row(cells: Vec<&str>) -> Result<Self, TransactionError> {
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
