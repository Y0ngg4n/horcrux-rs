use std::{fmt, io::LineWriter, path::PathBuf, ops::RangeInclusive};

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

const FRAGMENT_RANGE: RangeInclusive<usize> = 1..=255;

pub fn shards_in_range(s: &str) -> Result<u8, String> {
    let shards: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a number."))?;
    if FRAGMENT_RANGE.contains(&shards) {
        Ok(shards as u8)
    } else {
        Err(format!(
            "shards must be between {} and {}.",
            FRAGMENT_RANGE.start(),
            FRAGMENT_RANGE.end()
        ))
    }
}


pub fn is_qualified_path(p: &str) -> Result<PathBuf, String> {
    let path: PathBuf = p
        .parse()
        .map_err(|_| format!("`{p}` is not a path."))?;
    if !path.is_file() {
        Ok(path)
    } else {
        Err(format!(
            "{} is not a path.",
            path.to_string_lossy()
        ))
    }
}

pub fn is_qualified_file(f: &str) -> Result<PathBuf, String> {
    let file: PathBuf = f
        .parse()
        .map_err(|_| format!("`{f}` is not a file."))?;
    if file.is_file() && !file.is_dir() && !file.is_symlink() {
        Ok(file)
    } else {
        Err(format!(
            "{} is not a file.",
            file.to_string_lossy()
        ))
    }
}
