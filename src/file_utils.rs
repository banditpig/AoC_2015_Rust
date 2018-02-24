use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Ooops!");

    contents
}