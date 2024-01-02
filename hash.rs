use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::{BufReader, Read, Result};

pub fn calculate_sha1(file_path: &str) -> Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    let mut buffer = [0; 8192]; // Read in chunks of 8KB

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
