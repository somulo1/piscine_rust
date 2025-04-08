// src/lib.rs
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)   // Create if it doesn't exist
        .append(true)   // Append to the file
        .open(path)
        .unwrap();      // Panic if it fails

    file.write_all(content.as_bytes())
        .unwrap();      // Panic if writing fails
}
