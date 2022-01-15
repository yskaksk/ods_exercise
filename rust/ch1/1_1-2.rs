use std::env;
use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = File::open(filename).expect("File not found");

    let mut stack: Vec<String> = vec![];
    let mut offset = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(contents) = line {
            stack.push(contents);
        }
        if stack.len() == 50 {
            output(&stack, offset);
            stack = vec![];
            offset += 50;
        }
    }
    output(&stack, offset);
}

fn output(stack: &Vec<String>, offset: usize) {
    for i in (0..stack.len()).rev() {
        let c = stack.get(i).unwrap();
        println!("{:4}: {}", i+offset+1, c);
    }
}
