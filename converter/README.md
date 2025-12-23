# YPBank Converter

A command-line tool for converting banking transaction records between different formats.

## Usage

Convert transaction files between CSV, TXT, and BIN formats:

```bash
cargo run --bin ypbank_converter -- --input <FILE> --input-format <FORMAT> --output-format <FORMAT>
```

### Arguments

- `--input`: Path to the input file
- `--input-format`: Input format (csv, txt, or bin)
- `--output-format`: Output format (csv, txt, or bin)

### Examples

```bash
# Convert TXT to CSV
cargo run --bin ypbank_converter -- --input data/records.txt --input-format txt --output-format csv

# Convert BIN to TXT
cargo run --bin ypbank_converter -- --input data/records.bin --input-format bin --output-format txt
```

## Comparer Tool

Compare two transaction files regardless of their format:

```bash
cargo run --bin comparer -- --file1 <FILE1> --format1 <FORMAT1> --file2 <FILE2> --format2 <FORMAT2>
```

The comparer identifies the first mismatched transaction between files or confirms they are identical.