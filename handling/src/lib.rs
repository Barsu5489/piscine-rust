use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs,io::write};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let mut file: File = fs::OpenOptions::new()
   .create(true) 
   .append(true)
   .open(&path)?;

   file.write_all(content)

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
