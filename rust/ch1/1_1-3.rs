use std::env;
use std::io::{self, BufRead};
use std::fs::File;

use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = File::open(filename).expect("File not found");

    let mut stack: VecDeque<String> = VecDeque::with_capacity(50);
    for line in io::BufReader::new(file).lines() {
        if let Ok(contents) = line {
            if stack.len() == 42 {
                let popped = stack.pop_front().unwrap();
                if contents.is_empty() {
                    println!("{}", popped);
                }
            }
            stack.push_back(contents);
        }
    }
}
