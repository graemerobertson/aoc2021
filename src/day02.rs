use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day02() {
    let f: File = File::open("data/day02.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let commands: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    let mut aim: i32 = 0;

    for instruction in commands {
        let mut instruction_iter: std::str::SplitWhitespace = instruction.split_whitespace();
        let direction: &str = instruction_iter.next().unwrap();
        match direction {
            "forward" => {
                let value: i32 = instruction_iter.next().unwrap().parse::<i32>().unwrap();
                position = position + value;
                depth = aim * value + depth;
            }
            "up" => {
                aim = aim - instruction_iter.next().unwrap().parse::<i32>().unwrap();
            }
            "down" => {
                aim = aim + instruction_iter.next().unwrap().parse::<i32>().unwrap();
            }
            _ => {
                panic!("Unexpected operation: {}", direction);
            }
        }
    }

    println!("{}", depth * position);
}
