use serde::{Deserialize, Serialize};
use std::io::{BufRead, Seek, SeekFrom};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HorcruxHeader {
    pub canonical_file_name: String,
    pub timestamp: SystemTime,
    pub index: u8,
    pub total: u8,
    pub threshold: u8,
    #[serde(with = "serde_bytes")]
    pub nonce_fragment: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub key_fragment: Vec<u8>,
}

pub struct Horcrux {
    pub path: PathBuf,
    pub header: HorcruxHeader,
    pub contents: File,
}

impl Horcrux {
    pub fn new(path: PathBuf, header: HorcruxHeader, contents: File) -> Self {
        Self {
            path: path,
            header: header,
            contents: contents,
        }
    }

    //This naively assumes you have already passed in a file ending in .horcrux
    // Open the file, read over each line and extracting json content after reaching the -- HEADER -- marker. Stop scanning after reaching the -- BODY -- marker.
    // Set the file read pointer (seek) to be after the body marker
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let mut file: File = OpenOptions::new().read(true).append(true).open(path)?;

        let mut total_bytes_scanned = 0;
        let reader = BufReader::new(file.by_ref());
        let mut header_content = String::new();
        let mut header_found: bool = false;

        for line in reader.lines() {
            let line = line.expect("Failed to read line(s) in horcrux file.");
            total_bytes_scanned += line.len() + 1;

            if line == "-- HEADER --" {
                header_found = true;
                continue;
            }
            if header_found && line != "-- BODY --" {
                header_content.push_str(&line);
            }

            if line == "-- BODY --" {
                break; // Stop reading after reaching the body marker seek from here
            }
        }
        file.seek(SeekFrom::Start(total_bytes_scanned as u64))
            .expect("Failed seek horcrux file.");

        let header: HorcruxHeader =
            serde_json::from_str(&header_content).expect("Failed to parse horcrux file header.");

        Ok(Self {
            path: path.to_path_buf(),
            header: header,
            contents: file,
        })
    }
}
