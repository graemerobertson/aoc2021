use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub(crate) fn day02() {
    let f = File::open("data/day02.txt").unwrap();
    let reader = BufReader::new(f);
    let operations = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    println!("{:?}", operations);
}
