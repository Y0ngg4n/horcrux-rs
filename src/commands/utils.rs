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
    aead::{stream, Aead, NewAead, Error},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
use std::{
    fs::{self, File},
    io::{Read, Write, BufReader, self}, path::PathBuf,
};

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
