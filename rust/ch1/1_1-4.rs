use std::env;
use std::io::{self, BufRead};
use std::fs::File;

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = File::open(filename).expect("File not found");

    let mut set = HashSet::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(contents) = line {
            if !set.contains(&contents) {
                println!("{}", contents);
                set.insert(contents);
            }
        }
    }
}
