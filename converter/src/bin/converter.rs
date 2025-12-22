// ypbank_converter \
//   --input <input_file> \
//   --input-format <format> \
//   --output-format <format> \
//   > output_file.txt
use clap::{Parser, ValueEnum};
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
    println!("{:?}", args);
}
