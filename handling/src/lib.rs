use std::path::Path;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   OpenOptions::new()
   .create(true) 
   .append(true)
   .open(&path)
   .unwrap();

}
