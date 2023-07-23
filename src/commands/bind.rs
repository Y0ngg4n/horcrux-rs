use std::{path::{PathBuf}, fs::{self, OpenOptions, File}, io::{self, BufWriter}, error::Error};

use chacha20poly1305::XNonce;
use rand::{rngs::OsRng, RngCore};
use sharks::{Share, Sharks};

use crate::commands::horcrux::HorcruxHeader;

use super::{horcrux::Horcrux, utils::decrypt_file};



//Strategy is to get all files ending in .horcrux or .hx and then parse them. Next we filter them by matching nonce
fn find_horcrux_file_paths(directory: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(directory)
        .expect("Failed to read directory")
        .flat_map(|entry| {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "horcrux" || extension ==  "hx" {
                        return Some(path);
                    }
                }
            }
            
            None
        })
        .collect()
}


//Not to be confused with validate but group all of them with same nonces then sort by index
fn qualify_horcruxes() {
    
}


//Strategy is to find all horcrux files in a directory find any matches with the first one 
// And try recovery from there
pub fn bind(directory: &PathBuf) -> Result<(), Box<dyn Error>> {
    let horcrux_paths = find_horcrux_file_paths(directory);
    
    let horcruxes: Vec<Horcrux> = horcrux_paths
        .iter()
        .flat_map(|entry| Horcrux::from_path(entry))
        .collect();
    println!("LEN OF {:?}", horcruxes.len());

    

    let mut key_shares: Vec<Share> = Vec::new();
    let mut nonce_shares : Vec<Share> = Vec::new();
    let mut matching_horcruxes: Vec<&Horcrux> =  Vec::new();

    let initial_horcrux = &horcruxes[0];
    let initial_header: &HorcruxHeader = &initial_horcrux.header;
    
    let threshold: u8 = initial_header.threshold;
    
    for horcrux in &horcruxes  {
        if horcrux.header.canonical_file_name == initial_header.canonical_file_name.to_owned() && horcrux.header.timestamp == initial_header.timestamp {
            let kshare: Share = Share::try_from(horcrux.header.key_fragment.as_slice()).unwrap();
            let nshare: Share = Share::try_from(horcrux.header.nonce_fragment.as_slice()).unwrap();
            key_shares.push(kshare);
            nonce_shares.push(nshare);
            matching_horcruxes.push(&horcrux);
        }
    }

    
    if !(matching_horcruxes.len() > 0 && matching_horcruxes.len() >= threshold.to_owned() as usize) {
        //Err
        println!("Failed threshold: found {:?} horcruxes and {:?} are required to recover the file", matching_horcruxes.len(), threshold)
    }
    //Recover the secret
    let crypto_shark = Sharks(threshold);

    let key: [u8; 32] = crypto_shark.recover(&key_shares).unwrap().try_into().expect("Cannot recover key");
    let nonce: XNonce = crypto_shark.recover(&nonce_shares).unwrap().try_into().expect("Cannot recover nonce");
    
    let recovered_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(initial_horcrux.header.canonical_file_name).unwrap();
    let mut contents = initial_horcrux.contents.try_clone().unwrap();


    let decrypted = decrypt_file(&mut contents, &key, &tnonce).expect("Cannot decrypt file contents");
    
    let mut reader: &[u8] = decrypted.as_slice();
    let mut writer = BufWriter::new(recovered_file);

    io::copy(&mut reader, &mut writer)?;

    Ok(())
}