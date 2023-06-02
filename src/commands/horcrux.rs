use std::{time::SystemTime, path::{PathBuf, Path}, fs::{File, OpenOptions}, io::{Error, BufReader, Read}, borrow::Borrow};
use std::io::{self, BufRead, Seek, SeekFrom};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HorcruxHeader {
    pub canonical_file_name: String,
    pub timestamp: SystemTime,
    pub index: u8,
    pub total: u8,
    pub threshold: u8,
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub key_fragment: Vec<u8>
}

pub struct Horcrux {
    pub header: HorcruxHeader,
    pub path: PathBuf,
    pub contents: File,
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
            .open(path)
            .unwrap();
        



        let reader = BufReader::new(file.by_ref());
        let mut marker: Option<&str> = None;
        let mut body_position: usize = 0;
        // let mut header_content: String = String::new();

        let mut header_content = String::new();
        let mut header_found: bool = false;
        // Iterate over the lines of the file
        for (position, line) in reader.lines().enumerate() {
            let line = line.expect("Failed to read line");
            
            if line == "-- HEADER --" {
                header_found = true;
                continue;
            } else if header_found && line != "-- BODY --" {
                header_content.push_str(&line);
            } else if line == "-- BODY --" {
                body_position = position;
                break;
            }
        }

        println!("HERE");

        dbg!(&header_content);
        let header_result: Result<HorcruxHeader, _> = serde_json::from_str(&header_content);

        let header = match header_result {
            Ok(h) => h,
            Err(error) => panic!("Error with parsing {:?}", error)
        };
        dbg!(body_position);
            // let mut file_copy = file.by_ref().try_clone()?;
        file.seek(SeekFrom::Current(body_position as i64)).expect("Failed to seek position");
        
        
        // file.by_ref().seek(SeekFrom::Start((body_position as u64)));
        
        let horcrux = Horcrux::new(
            path,
            header,
            file
        );
        Ok(horcrux)
    }
}




