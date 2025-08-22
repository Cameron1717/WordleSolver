use std::fmt::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const DEFAULT_FILE: &str = "wordleanswers.txt";
fn main() {
    let reader = read_lines(DEFAULT_FILE).unwrap_or_else(|err| {
        panic!("error occured {}", err);
    });
    let mut filtered = reader.into_iter();
    let result = filtered.filter(|word| !word.contains("a"));
    for word in result {
        println!("{}", word);
    }
}
fn read_lines(file_path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    io::BufReader::new(file).lines().collect()
}
struct Letter {
    char: String,
    allowed: Vec<u32>,
    disallowe: Vec<u32>,
}
