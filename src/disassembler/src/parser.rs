use std::fs::File;
use std::io::Read;
use std::io;

pub fn parse(filename: &str) -> io::Result<(Vec<u8>)> {
    let mut file=File::open(filename).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}