use std::io::{Read, Result};
use aes::Aes256;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{NewBlockCipher, StreamCipher};
use rand::{RngCore, thread_rng};

pub fn crypto_reader<R: Read>(r: R, key: &[u8]) -> Result<impl Read> {
    let block_cipher = Aes256::new(GenericArray::from_slice(key));
    let iv = generate_iv(block_cipher.block_size());

    let mut stream_cipher = aes::stream_cipher::ctr::Ctr8::new(&block_cipher, GenericArray::from_slice(&iv));

    Ok(CryptoReader {
        reader: r,
        stream_cipher,
    })
}

fn generate_iv(block_size: usize) -> Vec<u8> {
    let mut iv = vec![0; block_size];
    thread_rng().fill_bytes(&mut iv);
    iv
}

struct CryptoReader<R: Read> {
    reader: R,
    stream_cipher: aes::stream_cipher::ctr::Ctr8<Aes256>,
}

impl<R: Read> Read for CryptoReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_bytes = self.reader.read(buf)?;

        self.stream_cipher.apply_keystream(&mut buf[..read_bytes]);

        Ok(read_bytes)
    }
}
