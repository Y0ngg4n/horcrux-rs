use crate::{
    commands::horcrux::{Horcrux, HorcruxHeader},
    crypto::decrypt_file,
};
use anyhow::{anyhow, Error};
use sharks::{Share, Sharks};
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

//Strategy is to get all files ending in .horcrux or .hx and then parse them. Next we filter them by matching nonce
fn find_horcrux_file_paths(directory: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let paths = fs::read_dir(directory)?;

    let horcruxes: Vec<PathBuf> = paths
        .flat_map(|entry| {
            let entry = entry.expect("Failed to read directory entry.");
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
        .collect();
    Ok(horcruxes)
}

//Strategy is to find all horcrux files in a directory find any matches with the first one
// And try recovery from there
pub fn bind(source: &PathBuf, destination: &PathBuf) -> Result<(), anyhow::Error> {
    let horcrux_paths = find_horcrux_file_paths(source)?;

    if horcrux_paths.is_empty() {
        let err = format!(
            "No horcrux files found in directory {}",
            source.to_string_lossy()
        );
        return Err(anyhow!(err));
    }

    let horcruxes: Vec<Horcrux> = horcrux_paths.into_iter().try_fold(
        Vec::new(),
        |mut acc: Vec<Horcrux>, entry: PathBuf| -> Result<Vec<Horcrux>, Error> {
            let hx = Horcrux::from_path(&entry)?;
            acc.push(hx);
            Ok(acc)
        },
    )?;

    let initial_horcrux = &horcruxes[0];
    let initial_header: &HorcruxHeader = &initial_horcrux.header;
    let threshold: u8 = initial_header.threshold;

    let mut key_shares: Vec<Share> = Vec::with_capacity(initial_header.total as usize);
    let mut nonce_shares: Vec<Share> = Vec::with_capacity(initial_header.total as usize);
    let mut matching_horcruxes: Vec<&Horcrux> = Vec::with_capacity(initial_header.total as usize);

    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for horcrux in &horcruxes {
        if horcrux.header.canonical_file_name == initial_header.canonical_file_name
            && horcrux.header.timestamp == initial_header.timestamp
        {
            let kshare: Share = Share::try_from(horcrux.header.key_fragment.as_slice())
                .map_err(|op| anyhow!(op))?;
            let nshare: Share = Share::try_from(horcrux.header.nonce_fragment.as_slice())
                .map_err(|op| anyhow!(op))?;
            key_shares.push(kshare);
            nonce_shares.push(nshare);
            matching_horcruxes.push(horcrux);
        }
    }

    if !(matching_horcruxes.is_empty() || matching_horcruxes.len() >= threshold.to_owned() as usize)
    {
        return Err(anyhow!(
            format!("Cannot find enough horcruxes to recover `{}` found {} matching horcruxes and {} matches are required to recover the file.",initial_header.canonical_file_name, matching_horcruxes.len(), threshold)
        ));
    }
    //Recover the secret
    let crypto_shark = Sharks(threshold);

    let key_result = crypto_shark
        .recover(&key_shares)
        .map_err(|_e| anyhow!("Not enough key fragments."))?;

    let nonce_result = crypto_shark
        .recover(&nonce_shares)
        .map_err(|_e| anyhow!("Not enough nonce fragments."))?;

    let key: [u8; 32] = key_result
        .try_into()
        .map_err(|_e| anyhow!("Cannot recover key fragment."))?;
    let nonce: [u8; 19] = nonce_result
        .try_into()
        .map_err(|_e| anyhow!("Cannot recover nonce fragment."))?;

    let mut recovered_file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination.join(&initial_horcrux.header.canonical_file_name))?;

    let mut contents = initial_horcrux.contents.try_clone().unwrap();

    decrypt_file(&mut contents, &mut recovered_file, &key, &nonce)?;
    Ok(())
}
