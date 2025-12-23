#[derive(Debug)]
pub enum InputError {
    InputExtensionMismatch(String, String),
    InputIsNotFile,
    InputFileNotExists,
}

#[derive(Debug)]
pub enum ComparerArgsError {
    File1ExtensionMismatch(String, String),
    File1IsNotFile,
    File1NotExists,

    File2ExtensionMismatch(String, String),
    File2IsNotFile,
    File2NotExists,
}
