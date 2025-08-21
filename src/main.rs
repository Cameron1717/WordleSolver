use std::fmt::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let mut base_list = match fs::File::open("wordleanswers.txt") {
        Ok(f) => f,
        Err(e) => panic!("error opening file {} {}!", e, e),
    };
    let mut contents: Vec<String> = Vec::new();
    let reader = BufReader::new(base_list);
    contents = reader.lines().collect::<Result<Vec<String>, std::Error>>();
    let mut refined = contents.iter();

    refined.filter(|word| word.contains("a")).collect();
    for item in refined {
        println!("{}", item);
    }
}
fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
