use std::{time::SystemTime, path::{PathBuf, Path}, fs::{File, OpenOptions}, io::{Error, BufReader, Read}, borrow::Borrow};
use std::io::{self, BufRead, Seek, SeekFrom};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct HorcruxHeader {
    pub canonical_file_name: String,
    pub timestamp: SystemTime,
    pub index: u8,
    pub total: u8,
    pub threshold: u8,
    pub nonce: [u8; 24],
    pub key_fragment: Vec<u8>
}

pub struct Horcrux {
    header: HorcruxHeader,
    path: PathBuf,
    contents: File,
}


impl Horcrux {
    pub fn new(path: &PathBuf, header: HorcruxHeader, contents: File) -> Horcrux {
        Self {
            path: path.to_owned(),
            header: header,
            contents: contents,
        }
    }


    //Given a file this will use BufReader to extract out the header 
    pub fn from_path(path: &PathBuf) -> Result<Horcrux, std::io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("Failed to open file");
        let reader = BufReader::new(file.by_ref());
        let mut marker: Option<&str> = None;
        let mut body_position:usize = 0;
        let mut header_content = String::new();

        // Iterate over the lines of the file
        for (position, line) in reader.lines().enumerate() {
            let line = line.expect("Failed to read line");
            
            match (marker, line.as_str()) {
                (None, "-- HEADER --") => marker = Some("header"),
                (None, "-- BODY --") => marker = Some("body"),
                (Some("header"), _) => header_content.push_str(&line),
                (Some("body"), _) => body_position = position,
                _ => {}
            }
        }


        let header: Result<HorcruxHeader, _> = serde_json::from_str(&header_content);
        

        file.seek(SeekFrom::Start(body_position as u64)).expect("Failed to seek position");
        
        // file.by_ref().seek(SeekFrom::Start((body_position as u64)));

        let horcrux = Horcrux::new(
            path,
            header.unwrap(),
            file
        );
        Ok(horcrux)
    }
}




