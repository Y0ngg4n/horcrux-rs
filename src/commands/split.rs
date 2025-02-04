use super::horcrux::{formatted_header, HorcruxHeader};
use crate::crypto::encrypt_file;
use chacha20poly1305::aead::OsRng;
use clap::builder::OsStr;
use rand::RngCore;
use sharks::{Share, Sharks};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, LineWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn split(
    source: &PathBuf,
    destination: &PathBuf,
    total: u8,
    threshold: u8,
) -> Result<(), anyhow::Error> {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    let crypto_shark = Sharks(threshold);

    //Break up key, nonce into same number of n fragments
    let key_dealer = crypto_shark.dealer(key.as_slice());
    let key_fragments: Vec<Share> = key_dealer.take(total as usize).collect();

    let nonce_dealer = crypto_shark.dealer(nonce.as_slice());
    let nonce_fragments: Vec<Share> = nonce_dealer.take(total as usize).collect();

    let timestamp = SystemTime::now();

    if !destination.exists() {
        let err = format!(
            "Cannot place horcruxes in directory `{}`. Try creating them in a different directory.",
            destination.to_string_lossy()
        );
        fs::create_dir_all(destination).expect(&err);
    }
    let default_file_name = OsStr::from("secret.txt");
    let default_file_stem = OsStr::from("secret");

    let canonical_file_name = &source
        .file_name()
        .unwrap_or(&default_file_name)
        .to_string_lossy();
    let file_stem = &source
        .file_stem()
        .unwrap_or(&default_file_stem)
        .to_string_lossy();
    let mut horcrux_files: Vec<File> = Vec::with_capacity(total as usize);

    for i in 0..total {
        let index = i + 1;
        let key_fragment = Vec::from(&key_fragments[i as usize]);
        let nonce_fragment = Vec::from(&nonce_fragments[i as usize]);
        let header = HorcruxHeader {
            canonical_file_name: canonical_file_name.to_string(),
            timestamp,
            index,
            total,
            threshold,
            nonce_fragment,
            key_fragment,
        };

        let json_header = serde_json::to_string(&header)?;
        let horcrux_filename = format!("{}_{}_of_{}.horcrux", file_stem, index, total);

        let horcrux_path = Path::new(&destination).join(&horcrux_filename);

        let horcrux_file: File = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open(&horcrux_path)?;

        let contents = formatted_header(index, total, json_header);
        let mut line_writer = LineWriter::new(&horcrux_file);

        line_writer.write_all(contents.as_bytes())?;
        drop(line_writer);
        horcrux_files.push(horcrux_file);
    }

    /* Strategy:
    In this state all the horcrux files contain their headers and an empty body.
    In order to avoid calling `encrypt_file` on each file, instead, we
    calculate the byte length after the header of the first file and store it as a variable. 
    Next we encrypt the first file, and then use seek to skip over the index file headers and copy only the necessary contents to the rest.
    This is possible because the body content is the same for each file.
    */
    let mut contents_to_encrypt = File::open(source)?;
    let mut initial_horcrux: &File = &horcrux_files[0];

    let read_pointer: u64 = initial_horcrux.seek(SeekFrom::End(0))?;

    let mut horcrux_handle = initial_horcrux.try_clone()?;

    encrypt_file(&mut contents_to_encrypt, &mut horcrux_handle, &key, &nonce)?;

    for horcrux in horcrux_files.iter().skip(1) {
        initial_horcrux.seek(SeekFrom::Start(read_pointer))?;
        io::copy(&mut initial_horcrux, &mut horcrux.to_owned())?;
    }
    Ok(())
}

