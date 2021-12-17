use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day16() {
    let f: File = File::open("data/day16.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: String = reader.lines().next().unwrap().unwrap();
}
