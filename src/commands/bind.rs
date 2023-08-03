use crate::{
    commands::horcrux::{Horcrux, HorcruxHeader},
    crypto::decrypt_file,
};
use anyhow::anyhow;
use sharks::{Share, Sharks};
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

//Strategy is to get all files ending in .horcrux or .hx and then parse them. Next we filter them by matching nonce
fn find_horcrux_file_paths(directory: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(&directory)
        .expect("Failed to read directory")
        .flat_map(|entry| {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "horcrux" || extension == "hx" {
                        return Some(path);
                    }
                }
            }

            None
        })
        .collect()
}

//Strategy is to find all horcrux files in a directory find any matches with the first one
// And try recovery from there
pub fn bind(directory: &PathBuf, destination: &PathBuf) -> Result<(), anyhow::Error> {
    let horcrux_paths = find_horcrux_file_paths(&directory);

    if !destination.exists() {
        let err = format!("Error cannot place horcruxes in directory `{}`. Try creating them in a different directory.", destination.to_string_lossy());
        fs::create_dir_all(&destination).expect(&err);
    }

    let horcruxes: Vec<Horcrux> = horcrux_paths
        .iter()
        .flat_map(|entry| Horcrux::from_path(&entry))
        .collect();

    let mut key_shares: Vec<Share> = Vec::new();
    let mut nonce_shares: Vec<Share> = Vec::new();
    let mut matching_horcruxes: Vec<&Horcrux> = Vec::new();

    let initial_horcrux = &horcruxes[0];
    let initial_header: &HorcruxHeader = &initial_horcrux.header;
    let threshold: u8 = initial_header.threshold;

    for horcrux in &horcruxes {
        if horcrux.header.canonical_file_name == initial_header.canonical_file_name.to_owned()
            && horcrux.header.timestamp == initial_header.timestamp
        {
            let kshare: Share = Share::try_from(horcrux.header.key_fragment.as_slice())
                .map_err(|op| anyhow!(op))?;
            let nshare: Share = Share::try_from(horcrux.header.nonce_fragment.as_slice())
                .map_err(|op| anyhow!(op))?;
            key_shares.push(kshare);
            nonce_shares.push(nshare);
            matching_horcruxes.push(&horcrux);
        }
    }

    if !(matching_horcruxes.len() > 0 && matching_horcruxes.len() >= threshold.to_owned() as usize)
    {
        return Err(anyhow!(
            format!("Cannot find enough horcruxes to recover `{}` found {} horcruxes and {} are required to recover the file.",initial_header.canonical_file_name, matching_horcruxes.len(), threshold)
        ));
    }
    //Recover the secret
    let crypto_shark = Sharks(threshold);

    let key: [u8; 32] = crypto_shark
        .recover(&key_shares)
        .unwrap()
        .try_into()
        .expect("Cannot recover key.");
    let nonce: [u8; 19] = crypto_shark
        .recover(&nonce_shares)
        .unwrap()
        .try_into()
        .expect("Cannot recover nonce.");

    let mut recovered_file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&initial_horcrux.header.canonical_file_name)?;
    let mut contents = initial_horcrux.contents.try_clone().unwrap();

    decrypt_file(&mut contents, &mut recovered_file, &key, &nonce)?;
    Ok(())
}
