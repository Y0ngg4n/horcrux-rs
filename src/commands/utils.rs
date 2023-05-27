// use std::io::{Read, Result};
// use aes::Aes256;
// use aes::cipher::generic_array::GenericArray;
// use aes::cipher::{BlockCipher, StreamCipher, KeyInit};
// use chacha20poly1305::XChaCha20Poly1305;
// use rand::{RngCore, thread_rng};


// fn encrypt_small_file(
//     filepath: &str,
//     dist: &str,
//     key: &[u8; 32],
//     nonce: &[u8; 24],
// ) -> Result<(), std::io::Error> {
//     let cipher = XChaCha20Poly1305::new(key.into());

//     let file_data = std::fs::read(filepath)?;

//     let encrypted_file = cipher
//         .encrypt(nonce.into(), file_data.as_ref())
//         .map_err(|err| err!("Encrypting small file: {}", err))?;

//     std::fs::write(&dist, encrypted_file)?;

//     Ok(())
// }

// pub fn crypto_reader<R: Read>(r: R, key: &[u8]) -> Result<impl Read> {
//     let block_cipher = Aes256::new(GenericArray::from_slice(key));
//     let iv = generate_iv(block_cipher);

//     let mut stream_cipher = aes::stream_cipher::ctr::Ctr8::new(&block_cipher, GenericArray::from_slice(&iv));
//     aes::
//     Ok(CryptoReader {
//         reader: r,
//         stream_cipher,
//     })
// }

// fn generate_iv(block_size: usize) -> Vec<u8> {
//     let mut iv = vec![0; block_size];
//     thread_rng().fill_bytes(&mut iv);
//     iv
// }

// struct CryptoReader<R: Read> {
//     reader: R,
//     stream_cipher: aes::stream_cipher::ctr::Ctr8<Aes256>,
// }

// impl<R: Read> Read for CryptoReader<R> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         let read_bytes = self.reader.read(buf)?;

//         self.stream_cipher.apply_keystream(&mut buf[..read_bytes]);

//         Ok(read_bytes)
//     }
// }
