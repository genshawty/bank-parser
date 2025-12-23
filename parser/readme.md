# Parser

A Rust library for parsing banking transaction records in multiple formats.

## Features

- Support for multiple input formats (CSV, TXT, BIN)
- Structured record parsing and validation
- Type-safe transaction record handling

## Components

- **CSV Format**: Parse and write transactions in CSV format
- **TXT Format**: Parse and write transactions in text block format with key-value pairs
- **BIN Format**: Parse and write transactions in binary format with YPBN magic header
- **Builder**: Fluent builder pattern for constructing transactions safely
- **Errors**: Comprehensive error types with Display and Error trait implementations

## Todo

- [x] Documentation
- [ ] CSV/TXT parsing by lines, not by file
- [x] Display and Error trait for errors