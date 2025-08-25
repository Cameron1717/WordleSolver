use std::alloc::realloc;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const DEFAULT_FILE: &str = "wordleanswers.txt";
const DEFAULT_SIZE: u32 = 5;
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
    greens: Option<Vec<u32>>,
    yellow: Option<Vec<u32>>,
    blacklist: bool,
}
fn new_letter(char: String, position: u32, color: &str) -> Letter {
    match color {
        "G" => {
            return Letter {
                char,
                greens: Option::Some(vec![position]),
                yellow: Option::None,
                blacklist: false,
            };
        }
        "Y" => {
            return Letter {
                char,
                greens: Option::None,
                yellow: Option::Some(vec![position]),
                blacklist: false,
            };
        }
        "B" => {
            return Letter {
                char,
                greens: Option::None,
                yellow: Option::None,
                blacklist: true,
            };
        }
        //fix this, allow for user correction
        _ => panic!("invalid char"),
    }
}

fn update_letter(char: String, position: u32, color: &str, old_data: &mut Letter) {
    todo!()
}
fn update_green(char: String, position: u32, color: &str, old_data: &mut Letter) {
    if old_data.blacklist {
        panic!(" Error: blacklisted letter is now green")
    }
    match old_data.greens.as_mut() {
        Some(g) => {
            if !g.contains(&position) {
                g.push(position);
                old_data.yellow = None;
            }
        }
        None => {
            old_data.greens = Some(vec![position]);
            old_data.yellow = None;
        }
    }
}
fn update_yellow(position: u32, old_data: &mut Letter) {
    if old_data.blacklist {
        panic!("error, previously blacklisted char is now allowed");
    }
    match old_data.yellow.as_mut() {
        Some(y) => {
            if !y.contains(&position) {
                y.push(position);
            }
        }
        None => {
            old_data.yellow = Some(vec![position]);
        }
    }
}
