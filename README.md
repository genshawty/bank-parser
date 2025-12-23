# YPBank Parsing Project

A Rust-based system for parsing, converting, and comparing banking transaction records across multiple file formats.

## Project Structure

### `parser/`
Core parsing library that handles transaction records in three formats:
- **CSV**: Comma-separated values with header row
- **TXT**: Text blocks with key-value pairs separated by double newlines
- **BIN**: Binary format with YPBN magic header and big-endian encoding

The library provides type-safe transaction handling, validation, and a builder pattern for constructing transactions.

### `converter/`
Command-line tools built on top of the parser library:
- **ypbank_converter**: Converts transaction files between formats (CSV ↔ TXT ↔ BIN)
- **comparer**: Compares two transaction files of any format and identifies differences

## Quick Start

```bash
# Convert TXT to CSV
cd converter
cargo run --bin ypbank_converter -- --input data/records.txt --input-format txt --output-format csv

# Compare two files
cargo run --bin comparer -- --file1 data/file1.csv --format1 csv --file2 data/file2.bin --format2 bin
```

## Transaction Format

Each transaction contains:
- Transaction ID (u64)
- Type (Deposit, Transfer, Withdrawal)
- From/To User IDs (u64)
- Amount (u64)
- Timestamp (u64)
- Status (Success, Failure, Pending)
- Description (String)

## Testing

Run tests for both crates:
```bash
# Parser tests
cd parser && cargo test

# Converter tests
cd converter && cargo test
```