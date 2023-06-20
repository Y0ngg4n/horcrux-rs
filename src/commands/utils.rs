// use chacha20poly1305::aead::{Aead, NewAead};
// use chacha20poly1305::XChaCha20Poly1305;
// use std::io::{Read, Result};

// macro_rules! crypto_reader {
//     ($file:expr, $key:expr) => {
//         {
//             let file = $file;
//             let key = $key;
//             let cipher = XChaCha20Poly1305::new(key.into());
        
//             // Wrap the reader with the cipher stream reader
//             let reader = Box::new(CryptoReader {
//                 cipher,
//                 reader: Box::new(file),
//             });
        
//             reader
//         }
//     };
// }

// struct CryptoReader<R> {
//     cipher: XChaCha20Poly1305,
//     reader: Box<R>,
// }

// impl<R: Read> Read for CryptoReader<R> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         let len = self.reader.read(buf)?;
//         let ciphertext = &mut buf[..len];
//         let nonce = chacha20poly1305::XNonce::from_slice(&[0u8; 24]);
//         let ct: Vec<u8> = ciphertext.into();
//         self.cipher.decrypt(nonce, ct);
//         Ok(len)
//     }
// }


use chacha20poly1305::{
    aead::{stream, Aead, NewAead, Error, AeadCore, Key}, 
    XChaCha20Poly1305, Nonce
};
use rand::{rngs::OsRng, RngCore};
use std::{
    fs::{self, File},
    io::{Read, Write, BufReader, self}, path::PathBuf,
};


const TWO_GB: i64 = 2 * 1024 * 1024 * 1024;

pub fn encrypt_small_file(
    filepath: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<Vec<u8>, std::io::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(filepath)?;
    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| ("UH OH"))
        .unwrap();

    Ok(encrypted_file)
}


pub fn decrypt_small_file(
    encrypted_file: &mut File,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<Vec<u8>, std::io::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let mut file_data = Vec::new();
    encrypted_file.read_to_end(&mut file_data)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err:Error| (err)).unwrap();
    Ok(decrypted_file)
}


fn encrypt_large_file(
    source_file_path: &str,
    dist_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), std::io::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());
    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(dist_file_path)?;

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err:Error| (err)).unwrap();
            dist_file.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err:Error| (err)).unwrap();
            dist_file.write(&ciphertext)?;
            break;
        }
    }

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