use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let mut file: File = OpenOptions::new()
   .create(true) 
   .append(true)
   .open(path);

   if let Err(err) = file.write_all(content.as_bytes()) {
    panic!("{}", err);
}

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
