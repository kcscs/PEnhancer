use std::fs::*;
use std::*;
use std::io::*;
use std::vec::*;

pub fn load_blacklist() -> Vec<String>{
    let f = OpenOptions::new().read(true).open("blacklist").expect("Cant open blacklist");
    let buf_reader = BufReader::new(f);
    buf_reader.lines().map(|x| {x.expect("Error in lambda expect :(").to_lowercase()}).collect()
}