 use std::fs::{self, File};

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}


