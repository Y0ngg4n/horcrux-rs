use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Read, self},
    path::{Path, PathBuf},
    time::{SystemTime}, borrow::BorrowMut
};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    XChaCha20Poly1305, XNonce, Key
};


use clap::builder::OsStr;
use sharks::{Share, Sharks};

use super::{horcrux::HorcruxHeader, utils::encrypt_file};

pub fn split(
    path: &PathBuf,
    destination: &str,
    total: u8,
    threshold: u8,
) -> Result<(), Box<dyn Error>> {
    // let mut key = [0u8; 32];
    let key: Key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce: XNonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
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

    let canonical_filename = &path.file_name().unwrap_or(&default_file_name).to_string_lossy();
    
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

        let json_header = serde_json::to_string_pretty(&header)?;

        let file_stem = path.file_stem().unwrap().to_string_lossy();

        let horcrux_filename = format!(
            "{}_{}_of_{}.horcrux",
            file_stem, index, total
        );

        let horcrux_path = Path::new(destination).join(&horcrux_filename);

        let horcrux_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&horcrux_path)?;
        
        horcrux_files.push(horcrux_file);
        let contents = formatted_header(index, total, json_header);
        fs::write(&horcrux_path, contents)?;
    }


    //Strategy here is to pass the first horcrux file
    let mut contents = File::open(path)?;
    let mut initial_horcrux = horcrux_files[0];

    encrypt_file(&mut contents, &mut initial_horcrux, &key, &nonce).expect("Error encrypting your file.");
    for horcrux in horcrux_files.iter().skip(1) {
        let mut writer = BufWriter::new(horcrux);
        io::copy(&mut initial_horcrux.take(u64::MAX), &mut writer)?;
    }

    
    Ok(())
}

//Refactor this into the struct and call it as a method
fn formatted_header(index: u8, total: u8, json_header: String) -> String {
    let remaining = total - 1;
    let file = format!("?? THIS FILE IS A HORCRUX. \n?? IT IS ONE OF {total} HORCRUXES THAT EACH CONTAIN PART OF AN ORIGINAL FILE. \n?? THIS IS HORCRUX NUMBER {index} of {total}. \n?? IN ORDER TO RESURRECT THIS ORIGINAL FILE YOU MUST FIND THE OTHER {remaining} HORCRUXES AND THEN BIND THEM USING THE PROGRAM FOUND AT THE FOLLOWING URL \n?? https://github.com \n \n-- HEADER --\n{json_header} \n-- BODY --\n");
    return file;
}