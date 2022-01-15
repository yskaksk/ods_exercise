use std::env;
use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = File::open(filename).expect("File not found");

    let mut stack: Vec<String> = vec![];
    for line in io::BufReader::new(file).lines() {
        if let Ok(contents) = line {
            stack.push(contents);
        }
    }
    for i in (0..stack.len()).rev() {
        let contents = stack.get(i).unwrap();
        println!("{:2}: {}", i+1, contents);
    }
}
