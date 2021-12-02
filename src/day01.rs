use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day01() {
    let f: File = File::open("data/day01.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut depths: Vec<u32> = vec![];
    for line in reader.lines() {
        depths.push(line.unwrap().parse::<u32>().unwrap());
    }

    let mut previous_depth: u32 = *depths.get(0).unwrap();
    let mut part_1_increases_count: u32 = 0;
    for depth in depths.clone() {
        if depth > previous_depth {
            part_1_increases_count = part_1_increases_count + 1;
        }
        previous_depth = depth;
    }

    println!("Part 1: {}", part_1_increases_count);

    let mut part_2_increases_count: u32 = 0;
    for n in 3..depths.len() {
        if depths.get(n).unwrap() > depths.get(n - 3).unwrap() {
            part_2_increases_count = part_2_increases_count + 1;
        }
    }

    println!("Part 2: {}", part_2_increases_count);
}
