use std::fs::OpenOptions;
use std::path::Path;
use std::{write, fs};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let mut file = fs::OpenOptions::new()
   .create(true) 
   .append(true)
   .open(&path)
   .unwrap();

   file.write_all(content.as_bytes()).unwrap()

}
