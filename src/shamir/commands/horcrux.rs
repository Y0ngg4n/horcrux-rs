use std::{time::SystemTime, path::{PathBuf, Path}, fs::File, io::Error};


#[derive(Debug)]
struct HorcruxHeader {
    canonical_file_name: String,
    timestamp: SystemTime,
    index: i32,
    total: i32,
    threshold: i32,
    key_fragment: [u8]
}

struct Horcrux<'a> {
    path: PathBuf,
    header: &'a HorcruxHeader,
    file: &'a File,
}

//fn parse_file_header(file: &File)
