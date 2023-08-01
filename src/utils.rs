use std::{fmt, io::LineWriter};

#[derive(Debug,)]
pub enum CliError {
    IOError,
    ParseError,
    InputError,
    DecryptionError,
    EncryptionError,
    CryptographyError
}


impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::IOError => write!(f, "Custom Error 2 occurred."),
            CliError::ParseError => write!(f, "Custom Error 1 occurred."),
            CliError::InputError => write!(f, "Custom Error 1 occurred."),
            CliError::DecryptionError => write!(f, "Custom Error 2 occurred."),
            CliError::EncryptionError => write!(f, "Custom Error 2 occurred."),
            CliError::CryptographyError => write!(f, "Custom Error 2 occurred."),
        }
    }
}

// pub fn handle_std_in() -> Vec<u8> {
// }


