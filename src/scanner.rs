use std::fs;
use std::path::Path;

pub fn scan_directory(path: &str) {
    println!("Scanning directory: {}", path);
    if let Ok(entries) = fs::read_dir(Path::new(path)) {
        for entry in entries.flatten() {
            println!("Found file: {:?}", entry.path());
        }
    }
}