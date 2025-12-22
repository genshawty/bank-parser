// ypbank_converter \
//   --input <input_file> \
//   --input-format <format> \
//   --output-format <format> \
//   > output_file.txt
use clap::{Parser, ValueEnum};
use parser::Transaction;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    input: PathBuf,

    #[arg(long)]
    input_format: Format,

    #[arg(long)]
    output_format: Format,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Format {
    Csv,
    Txt,
    Bin,
}

fn main() {
    let args = Cli::parse();

    if !args.input.exists() {
        eprintln!("Input file not found: {:?}", args.input);
        return;
    } else {
        match args.input.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                if ext != format!("{:?}", args.input_format).to_ascii_lowercase() {
                    eprintln!("Input format does not match input file format");
                    return;
                }
            }
            None => {
                eprintln!("Input file is not a file");
                return;
            }
        }
    }

    let transactions = match args.input_format {
        Format::Csv => {
            parser::Parser::read_from_csv(r)
        }
    }
}
