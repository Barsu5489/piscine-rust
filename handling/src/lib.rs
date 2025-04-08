use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let mut file = fs::OpenOptions::new()
   .create(true) 
   .append(true)
   .open(&path)
   .unwrap();

   file.write(content.as_bytes()).unwrap()

}
