use std::fs::OpenOptions;
use std::path::Path;
use std::{fs,io::write};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let mut file: File = fs::OpenOptions::new()
   .create(true) 
   .append(true)
   .open(&path)
   .unwrap();

}
