use crate::commands::horcrux;
use chacha20poly1305::aead::OsRng;
use clap::builder::OsStr;
use rand::RngCore;
use sharks::{Share, Sharks};
use std::{
    borrow::BorrowMut,
    clone,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    time::SystemTime, cell::RefCell,
};

use super::{
    horcrux::{Horcrux, HorcruxHeader},
    utils::encrypt_file,
};

pub fn split(
    path: &PathBuf,
    destination: &str,
    total: u8,
    threshold: u8,
) -> Result<(), Box<dyn Error>> {
    // let mut key = [0u8; 32];
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    let crypto_shark = Sharks(threshold);

    //Break up key, nonce into same number of fragments
    let key_dealer = crypto_shark.dealer(key.as_slice());
    let key_fragments: Vec<Share> = key_dealer.take(total as usize).collect();

    let nonce_dealer = crypto_shark.dealer(nonce.as_slice());
    let nonce_fragments: Vec<Share> = nonce_dealer.take(total as usize).collect();

    let timestamp = SystemTime::now();

    let destination_dir = Path::new(destination);
    if !destination_dir.exists() {
        let err = format!("Error cannot place horcruxes in directory {}. Try creating them in a different directory.", destination);
        fs::create_dir_all(destination_dir).expect(&err);
    } else if !destination_dir.is_dir() {
        //Return error
    }

    let default_file_name = OsStr::from("horcrux.txt");
    let canonical_filename = &path
        .file_name()
        .unwrap_or(&default_file_name)
        .to_string_lossy();
    let file_stem = path.file_stem().unwrap().to_string_lossy();
    let mut horcrux_files: Vec<File> = Vec::with_capacity(total as usize);

    for i in 0..total {
        let index = i + 1;
        let key_fragment = Vec::from(&key_fragments[i as usize]);
        let nonce_fragment = Vec::from(&nonce_fragments[i as usize]);
        let header = HorcruxHeader {
            canonical_file_name: canonical_filename.to_string(),
            timestamp: timestamp,
            index: index,
            total: total,
            threshold: threshold,
            nonce_fragment: nonce_fragment,
            key_fragment: key_fragment,
        };

        let json_header = serde_json::to_string(&header)?;

        let horcrux_filename = format!("{}_{}_of_{}.horcrux", file_stem, index, total);

        let horcrux_path = Path::new(destination).join(&horcrux_filename);

        let horcrux_file: File = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .write(true)
            .open(&horcrux_path)?;
        horcrux_files.push(horcrux_file);
        let contents = formatted_header(index, total, json_header);
        fs::write(&horcrux_path, contents)?;
    }

    /* Strategy
    1) In this state we have total `n` number of files only containing headers.
    2) We will use the first file in the to write the encrypted contents into and then seek it after the headers
    and copy it to the rest.
    */
    let mut contents_to_encrypt = File::open(&path)?;
    let mut initial_horcrux: &File = &horcrux_files[0];

    let read_pointer: u64 = initial_horcrux.seek(SeekFrom::End(0))?;
    let mut cloned_horcrux = initial_horcrux.try_clone()?;

    encrypt_file(&mut contents_to_encrypt, &mut cloned_horcrux, &key, &nonce)
        .expect("Error encrypting your file.");
    
    cloned_horcrux.seek(SeekFrom::Start(read_pointer))?;


    //This seems to only copy on the first loop then forgets the rest ... ?
    for horcrux in horcrux_files.iter().skip(1) { //i in 1..horcrux_files.len()
        let mut writer = BufWriter::new(horcrux);
        io::copy(&mut cloned_horcrux, &mut writer).expect("Something wrong");
    }
    Ok(())
}

//Refactor this into the struct and call it as a method
fn formatted_header(index: u8, total: u8, json_header: String) -> String {
    let remaining = total - 1;
    let file = format!("?? THIS FILE IS A HORCRUX. \n?? IT IS ONE OF {total} HORCRUXES THAT EACH CONTAIN PART OF AN ORIGINAL FILE. \n?? THIS IS HORCRUX NUMBER {index} of {total}. \n?? IN ORDER TO RESURRECT THIS ORIGINAL FILE YOU MUST FIND THE OTHER {remaining} HORCRUXES AND THEN BIND THEM USING THE PROGRAM FOUND AT THE FOLLOWING URL \n?? https://github.com \n \n-- HEADER --\n{json_header} \n-- BODY --\n");
    return file;
}