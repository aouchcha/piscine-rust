use panic::*;
use std::fs::{self, File};

fn main() {
    let filename = "created.txt";
    File::create(filename).unwrap();

    println!("{:?}", open_file(filename));

    fs::remove_file(filename).unwrap();

    // this should panic!
    open_file(filename);
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn test_opening() {
    open_file("file.txt");
}

#[test]
fn test_opening_existing() {
    let filename = "created.txt";
    File::create(filename).unwrap();
    open_file(filename);
    fs::remove_file(filename).unwrap();
}