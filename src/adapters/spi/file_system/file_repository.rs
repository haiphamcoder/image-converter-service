use std::fs;
use std::io::{self, Write};

pub fn save_file(file_path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(data)
}

pub fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
    fs::read(file_path)
}