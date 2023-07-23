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
//https://docs.rs/aead/latest/aead/stream/struct.Encryptor.html#method.encrypt_next_in_place

use chacha20poly1305::{
    aead::{stream::{self, EncryptorBE32, NewStream}, Aead, Error, AeadCore, AeadInPlace}, 
    XChaCha20Poly1305, XNonce, Key, KeyInit
};
use std::{
    fs::{File},
    io::{Read, Write},
};


pub fn encrypt_file(
    source: &mut File,
    destination: &mut File,
    key: &Key,
    nonce: &XNonce,
) -> Result<(), std::io::Error> {
    let aead = XChaCha20Poly1305::new(&key);
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
    key: &Key,
    nonce: &XNonce,
) -> Result<(), std::io::Error> {
    let aead = XChaCha20Poly1305::new(&key);
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_slice().into());

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




