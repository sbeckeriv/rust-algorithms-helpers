pub mod timer;
pub mod stdin_lines;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

pub fn read_file_lines(file_string: String) -> Vec<String> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|num| num.to_string()).collect::<Vec<String>>()
}

pub fn read_file_chars_as_numbers(file_string: String) -> Vec<isize> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>()
}

pub fn read_file_chars(file_string: String) -> Vec<String> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|num| num.to_string()).collect::<Vec<String>>()
}

#[test]
fn it_works() {}
