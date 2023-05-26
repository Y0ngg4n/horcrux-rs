use core::time;
use std::{time::{SystemTime, UNIX_EPOCH}, path::Path, fs::{self, File}, error::Error, io::Write};

//TODO here stdin: Strategy for handling pipeline is check if 
//Input string is greater than 255 characters if it is then we can safely assume 
// what was passed is a the string contents of a file. If it's less than we check if it's a directory and then
// we create 
use rand::RngCore;
use sharks::{ Sharks, Share };

use super::horcrux::HorcruxHeader;




pub fn split(path: &str, destination: &str, total: u8, threshold: u8) -> Result<(), Box<dyn Error>> {
    let key = generate_key();
    if !key.is_some() {
        //Return err
    }
    let sharks = Sharks(threshold);
    // Obtain an iterator over the shares for secret [1, 2, 3, 4]
    let dealer = sharks.dealer(key.unwrap().as_slice());
    // Get 10 shares
    let fragments: Vec<Share> = dealer.take(total as usize).collect();
    
    let timestamp = SystemTime::now();

    let destination_dir = Path::new(destination);
    if !destination_dir.exists() {
        fs::create_dir_all(destination_dir);
    } else if !destination_dir.is_dir() {
        //Return error
    }
    //Open file
    let file = File::open(path);
    let original_filename = Path::new(path).file_name().unwrap().to_string_lossy().to_string();
    let mut horcrux_files:Vec<File> = Vec::with_capacity(total as usize);

    for i in 0..total {
        let index = i + 1;
        let fragment = Vec::from(&fragments[i as usize]);
        let header = HorcruxHeader {
            canonical_file_name: original_filename,
            timestamp: timestamp,
            index: index,
            total: total,
            threshold: threshold,
            key_fragment: fragment
        };
        
        //originalFilename := filepath.Base(path)
        let header_bytes: Vec<u8> = serde_json::to_vec(&header)?;

        let original_filename_without_ext = Path::new(&original_filename)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        let horcrux_filename = format!("{}_{}_of_{}.horcrux", original_filename_without_ext, index, total);
        let horcrux_path = Path::new(destination).join(&horcrux_filename);
        println!("creating {:?}", horcrux_path);

        let horcrux_file = File::create(&horcrux_path)?;
        horcrux_files.push(horcrux_file);

        let contents = formatted_header(index, total, header_bytes);

        fs::write(&horcrux_path, contents)?;
    }

    let mut reader = File::open(path)?;
    let mut reader = crypto_reader(&mut reader, &key)?;

    let writers: Vec<&mut dyn Write> = horcrux_files.iter_mut().map(|f| f).collect();
    let mut writer = std::io::BufWriter::new(std::io::sink());
    for w in writers {
        writer.get_mut().extend(w);
    }
    std::io::copy(&mut reader, &mut writer)?;
    Ok(())
}


fn generate_key() -> Option<Vec<u8>> {
    let mut key = vec![0; 32];
    rand::thread_rng().try_fill_bytes(&mut key);
    Some(key)
}

//Refactor this into the struct and call it as a method
fn formatted_header(index: u8, total: u8, header_bytes: Vec<u8>) -> String {
    let remaining = total - 1;
    let file = format!("# THIS FILE IS A HORCRUX.
    # IT IS ONE OF {total} HORCRUXES THAT EACH CONTAIN PART OF AN ORIGINAL FILE.
    # THIS IS HORCRUX NUMBER {index}.
    # IN ORDER TO RESURRECT THIS ORIGINAL FILE YOU MUST FIND THE OTHER {remaining} HORCRUXES AND THEN BIND THEM USING THE PROGRAM FOUND AT THE FOLLOWING URL
    # TODO
    
    -- HEADER --
    {header_bytes}
    -- BODY --");
    return file
}