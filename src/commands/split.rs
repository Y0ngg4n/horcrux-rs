use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{Write, BufWriter, Read, BufReader},
    path::Path,
    time::{SystemTime}, sync::{Arc, Mutex}, borrow::BorrowMut, cell::RefCell
};

//TODO here stdin: Strategy for handling pipeline is check if
//Input string is greater than 255 characters if it is then we can safely assume
// what was passed is a the string contents of a file. If it's less than we check if it's a directory and then
// we create
use rand::{RngCore, rngs::OsRng};
use sharks::{Share, Sharks};

use super::{horcrux::HorcruxHeader, utils::encrypt_small_file};

pub fn split(
    path: &str,
    destination: &str,
    total: u8,
    threshold: u8,
) -> Result<(), Box<dyn Error>> {
    let mut key = generate_key();
    let nonce = generate_nonce().unwrap();
    
    let sharks = Sharks(threshold);
    if !key.is_some() {
        //Return err
        println!("UH OH COULD NOT GENERATE KEY")
    }
    let clone = key.clone();
    // let secret: Result<[u8; 32], _> = key.unwrap().as_slice().try_into();
    // Obtain an iterator over the shares for secret [1, 2, 3, 4]
    let dealer = sharks.dealer(clone.unwrap().as_slice());
    // Get 10 shares
    let fragments: Vec<Share> = dealer.take(total as usize).collect();
    let timestamp = SystemTime::now();

    let destination_dir = Path::new(destination);
    if !destination_dir.exists() {
        fs::create_dir_all(destination_dir);
    } else if !destination_dir.is_dir() {
        //Return error
    }
    
    let original_filename = Path::new(path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    //Todo hashmap this fucker
    let mut horcrux_files: Vec<File> = Vec::with_capacity(total as usize);

    for i in 0..total {
        let index = i + 1;
        let fragment = Vec::from(&fragments[i as usize]);
        let header = HorcruxHeader {
            canonical_file_name: original_filename.to_owned(),
            timestamp: timestamp,
            index: index,
            total: total,
            threshold: threshold,
            nonce: nonce,
            key_fragment: fragment,
        };

        //originalFilename := filepath.Base(path)
        let json_header = serde_json::to_string_pretty(&header)?;

        let original_filename_without_ext = Path::new(&original_filename)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        let horcrux_filename = format!(
            "{}_{}_of_{}.horcrux",
            original_filename_without_ext, index, total
        );
        let horcrux_path = Path::new(destination).join(&horcrux_filename);
        println!("creating {:?}", horcrux_path);

        let horcrux_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&horcrux_path)?;
        horcrux_files.push(horcrux_file);

        let contents = formatted_header(index, total, json_header);

        fs::write(&horcrux_path, contents)?;
    }
    
    let encrypted = encrypt_small_file(&path, &key.unwrap(), &nonce);
    let reader: &[u8] = &encrypted.unwrap();

    for horcrux in horcrux_files {
        let mut writer = BufWriter::new(horcrux);
        std::io::copy(&mut reader.take(u64::MAX), &mut writer)?;
    }

    Ok(())
}

fn generate_key() -> Option<[u8; 32]> {
    let mut key: [u8; 32] = [0u8; 32];
    OsRng.try_fill_bytes(&mut key).expect("Failed to generate key");
    Some(key)
}


fn generate_nonce() -> Option<[u8; 24]> {
    let mut nonce: [u8; 24] = [0u8;24];
    OsRng.try_fill_bytes(&mut nonce).expect("Failed to generate nonce");
    Some(nonce)
}


//Refactor this into the struct and call it as a method
fn formatted_header(index: u8, total: u8, json_header: String) -> String {
    let remaining = total - 1;
    let file = format!("?? THIS FILE IS A HORCRUX. \n?? IT IS ONE OF {total} HORCRUXES THAT EACH CONTAIN PART OF AN ORIGINAL FILE. \n?? THIS IS HORCRUX NUMBER {index} of {total}. \n?? IN ORDER TO RESURRECT THIS ORIGINAL FILE YOU MUST FIND THE OTHER {remaining} HORCRUXES AND THEN BIND THEM USING THE PROGRAM FOUND AT THE FOLLOWING URL \n?? https://github.com \n \n-- HEADER -- \n{json_header} \n-- BODY -- \n");
    return file;
}