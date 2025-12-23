pub mod errors;

use clap::{Parser, ValueEnum};
use parser::{Transaction, errors::ParsingError};
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[arg(long)]
    pub input: PathBuf,

    #[arg(long)]
    pub input_format: Format,

    #[arg(long)]
    pub output_format: Format,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Format {
    Csv,
    Txt,
    Bin,
}

pub fn get_transactions(
    input: PathBuf,
    input_format: Format,
) -> Result<Vec<Transaction>, ParsingError> {
    let file = std::fs::File::open(input)?;
    let mut reader = BufReader::new(file);
    let transactions = match input_format {
        Format::Csv => parser::Parser::read_from_csv(&mut reader),
        Format::Txt => parser::Parser::read_from_txt(&mut reader),
        Format::Bin => parser::Parser::read_from_bin(&mut reader),
    };
    transactions
}
