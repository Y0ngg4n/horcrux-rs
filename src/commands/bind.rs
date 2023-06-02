use std::{path::{Path, PathBuf}, fs::{self, OpenOptions, File}, io::{self, BufWriter, Read}, error::Error};

use sharks::{Share, Sharks};

use super::{horcrux::Horcrux, utils::decrypt_small_file};



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
    for path in &horcrux_paths {
        dbg!(path);
    }
    let horcruxes: Vec<Horcrux> = horcrux_paths
        .iter()
        .flat_map(|entry| Horcrux::from_path(entry))
        .collect();
    println!("LEN OF {:?}", horcruxes.len());

    let mut shares: Vec<Share> = Vec::new();
    let mut matching_horcruxes: Vec<&Horcrux> =  Vec::new();

    let first_horcrux = &horcruxes[0];
    let target_nonce: &[u8] = &horcruxes[0].header.nonce.as_slice();
    let target_file_name = &horcruxes[0].header.canonical_file_name;
    let threshold: &u8 = &horcruxes[0].header.threshold;
    
    for horcrux in &horcruxes  {
        if horcrux.header.nonce == target_nonce.to_owned() && horcrux.header.canonical_file_name == target_file_name.to_owned() {
            let share: Share = Share::try_from(horcrux.header.key_fragment.as_slice()).unwrap();
            shares.push(share);
            matching_horcruxes.push(&horcrux);
        }
    }

    dbg!(matching_horcruxes.len());
    
    if !(matching_horcruxes.len() > 0 && matching_horcruxes.len() >= threshold.to_owned() as usize) {
        println!("Failed threshold: found {:?} horcruxes and {:?} are required to recover the file", matching_horcruxes.len(), threshold)
    }
    //Recover the secret
    let sharks = Sharks(threshold.clone());
    
    let key: [u8; 32] = sharks.recover(&shares).unwrap().try_into().expect("Cannot recover secret");
    
    println!("RECOV KEY");
    
    let recovered_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open("test.recovered.txt").unwrap();
    let mut contents = first_horcrux.contents.try_clone().unwrap();
    let decrypted = decrypt_small_file(&mut contents, &key, target_nonce.try_into().unwrap());
    
    let fc = match decrypted {
        Ok(a) => a,
        Err(why) => panic!("Cannot decrypt {:?}", why)
    };
    
    let mut reader: &[u8] = fc.as_slice();
    let mut writer = BufWriter::new(recovered_file);

    io::copy(&mut reader, &mut writer)?;

    Ok(())
}