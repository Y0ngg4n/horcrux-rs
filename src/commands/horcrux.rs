use std::{time::SystemTime, path::{PathBuf, Path}, fs::File, io::Error};
use std::io::{self, BufRead, Seek, SeekFrom};
use serde_json::{self, Error as JsonError};

#[derive(Debug)]
struct HorcruxHeader {
    canonical_file_name: String,
    timestamp: SystemTime,
    index: i32,
    total: i32,
    threshold: i32,
    key_fragment: Vec<u8>
}

#[derive(Default)]
struct Horcrux<'a> {
    path: PathBuf,
    header: HorcruxHeader,
    file: &'a File,
}


impl<'a> Horcrux<'a> {
    pub fn new(&self, path: PathBuf) -> Result<Horcrux, std::io::Error> {
        let mut file = File::open(path)?;

        let mut header = get_header(&mut file)?;
        let horcrux = Horcrux {
            path: path.to_owned(),
            header: header,
            file: &file,
        };
        Ok(horcrux)
    
    }

    

}

fn get_header(file: &mut File) -> Result<HorcruxHeader, io::Error> {
    let mut current_header = HorcruxHeader::default();
    let mut bytes_before_body = 0;

    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let line_bytes = line.as_bytes();
        bytes_before_body += line_bytes.len() + 1;

        if line == "-- HEADER --" {
            let header_line_result = reader.lines().next();
            if let Some(header_line_result) = header_line_result {
                let header_line = header_line_result?;
                let header_bytes = header_line.as_bytes();
                bytes_before_body += header_bytes.len() + 1;

                if let Err(err) = serde_json::from_slice(header_bytes, &mut current_header) {
                    return Err(io::Error::new(io::ErrorKind::Other, err.to_string()));
                }

                reader.lines().next(); // Skip the body line
                bytes_before_body += line_bytes.len() + 1;
                break;
            }
        }
    }

    file.seek(SeekFrom::Start(bytes_before_body as u64))?;

    if current_header.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Header missing from horcrux file"));
    }

    Ok(current_header)
}


