use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   fs::OpenOptions::new()
   .create(true) 
   .append(true)
   .open(&path)
   .unwrap();

}
