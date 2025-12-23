use clap::Parser;
use parser::Parser as ParserTrait;
use std::io::{BufWriter, Write};
use ypbank_converter::errors::InputError;
use ypbank_converter::{Cli, Format, get_transactions};

fn validate_args(args: Cli) -> Result<(), InputError> {
    if !args.input.exists() {
        return Err(InputError::InputFileNotExists);
    } else {
        match args.input.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                if ext != format!("{:?}", args.input_format).to_ascii_lowercase() {
                    return Err(InputError::InputExtensionMismatch(
                        ext.to_string(),
                        format!("{:?}", args.input_format).to_ascii_lowercase(),
                    ));
                }
            }
            None => return Err(InputError::InputIsNotFile),
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    validate_args(args.clone()).expect("invalid arguments");

    let transactions = match get_transactions(args.input, args.input_format) {
        Ok(tx) => tx,
        Err(e) => panic!("error parsing transactions: {}", e),
    };

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    match args.output_format {
        Format::Csv => ParserTrait::write_to_csv(&mut writer, transactions),
        Format::Txt => ParserTrait::write_to_txt(&mut writer, transactions),
        Format::Bin => ParserTrait::write_to_bin(&mut writer, transactions),
    }
    .expect("error writing output");

    writer.flush().expect("error flushing output");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_args_file_not_exists() {
        let args = Cli {
            input: PathBuf::from("nonexistent_file.txt"),
            input_format: Format::Txt,
            output_format: Format::Csv,
        };

        let result = validate_args(args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InputError::InputFileNotExists
        ));
    }

    #[test]
    fn test_validate_args_extension_mismatch() {
        // Create a temporary file with .csv extension
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_file.csv");
        std::fs::write(&temp_file, "test").unwrap();

        let args = Cli {
            input: temp_file.clone(),
            input_format: Format::Txt, // Format doesn't match extension
            output_format: Format::Csv,
        };

        let result = validate_args(args);

        // Cleanup
        std::fs::remove_file(&temp_file).ok();

        assert!(result.is_err());
        if let Err(InputError::InputExtensionMismatch(ext, expected)) = result {
            assert_eq!(ext, "csv");
            assert_eq!(expected, "txt");
        } else {
            panic!("Expected InputExtensionMismatch error");
        }
    }

    #[test]
    fn test_validate_args_valid_csv() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_valid.csv");
        std::fs::write(&temp_file, "test").unwrap();

        let args = Cli {
            input: temp_file.clone(),
            input_format: Format::Csv,
            output_format: Format::Txt,
        };

        let result = validate_args(args);

        // Cleanup
        std::fs::remove_file(&temp_file).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_args_valid_txt() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_valid.txt");
        std::fs::write(&temp_file, "test").unwrap();

        let args = Cli {
            input: temp_file.clone(),
            input_format: Format::Txt,
            output_format: Format::Bin,
        };

        let result = validate_args(args);

        // Cleanup
        std::fs::remove_file(&temp_file).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_args_valid_bin() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_valid.bin");
        std::fs::write(&temp_file, "test").unwrap();

        let args = Cli {
            input: temp_file.clone(),
            input_format: Format::Bin,
            output_format: Format::Csv,
        };

        let result = validate_args(args);

        // Cleanup
        std::fs::remove_file(&temp_file).ok();

        assert!(result.is_ok());
    }
}
