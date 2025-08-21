use std::fs;
use std::io::Read;
use std::string;
fn main() {
    let mut base_list = match fs::File::open("wordleanswers.txt") {
        Ok(f) => f,
        Err(e) => panic!("error opening file {} {}!", e, e),
    };
    let mut contents = String::new();
    match base_list.read_to_string(&mut contents) {
        Ok(_) => println!("{}", contents),
        Err(e) => panic!("cannot read file{}", e),
    };
}
