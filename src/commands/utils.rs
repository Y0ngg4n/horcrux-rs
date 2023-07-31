use chacha20poly1305::{
    aead::{stream::{self}, Error}, 
    XChaCha20Poly1305, KeyInit
};
use std::{ fs::File, io::{Read, Write}, fmt};

pub fn encrypt_file(
    source: &mut File,
    destination: &mut File,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), std::io::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_slice().into());
    const BUFFER_LENGTH: usize = 500;
    let mut buffer = [0u8; BUFFER_LENGTH];

    loop {
        let read_count = source.read(&mut buffer)?;
        if read_count == BUFFER_LENGTH {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err:Error| (err)).unwrap();
            destination.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err:Error| (err)).unwrap();
            destination.write(&ciphertext)?;
            break;
        }
    }
    Ok(())
}


pub fn decrypt_file(
    encrypted_source: &mut File,
    destination: &mut File,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), std::io::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.into());

    const BUFFER_LENGTH: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LENGTH];

    loop {
        let read_count = encrypted_source.read(&mut buffer)?;
        if read_count == BUFFER_LENGTH {
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|err:Error| (err)).unwrap();
            destination.write(&plaintext)?;
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|err:Error| (err)).unwrap();
            destination.write(&plaintext)?;
            break;
        }
    }
    drop(destination);
    Ok(())
}

pub enum CliError {
    IOError(std::io::Error),
    ParseError(serde_json::Error),
    InputError,
    DecryptionError,
    EncryptionError,
    CryptographyError
}




impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::ParseError(f) => write!(f, "Custom Error 1 occurred."),
            CliError::InputError => write!(f, "Custom Error 1 occurred."),
            CliError::CryptographyError => write!(f, "Custom Error 2 occurred."),
            // Handle more error variants here
        }
    }
}