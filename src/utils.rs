use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Ooops!");

    contents
}
pub fn has_duplicates<T: Eq + Ord + Clone>(v: &Vec<T>) -> bool {
    if v.len() == 0 { return false; }

    let (head, tail) = v.split_at(1);
    if tail.contains(&head[0]) {
        return true;
    }
    has_duplicates(&tail.to_vec())
}