use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use num::ToPrimitive;

pub(crate) fn day08() {
    let f: File = File::open("data/day08.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut part_1_count: u32 = 0;
    let mut part_2_count: u32 = 0;

    for line in diagnostics {
        let mut iter = line.split('|');
        let input_digits: Vec<HashSet<char>> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();
        let output_digits = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.chars().collect::<HashSet<char>>());

        // We're going to figure out which set of characters maps to which digit. We're
        // logically storing them in a map of digit->character set, but since we're
        // mapping the digits 0->9, that's just easier to do in a vector.
        let mut encoding_map: Vec<HashSet<char>> = vec![HashSet::new(); 10];

        for digit in &input_digits {
            match digit.len() {
                2 => {
                    encoding_map[1] = digit.clone();
                }
                3 => {
                    encoding_map[7] = digit.clone();
                }
                4 => {
                    encoding_map[4] = digit.clone();
                }
                7 => {
                    encoding_map[8] = digit.clone();
                }
                // We need more information.
                _ => {}
            }
        }

        for digit in &input_digits {
            if digit.len() == 6 {
                if encoding_map[4].is_subset(digit) {
                    encoding_map[9] = digit.clone();
                } else if encoding_map[1].is_subset(digit) {
                    encoding_map[0] = digit.clone();
                } else {
                    encoding_map[6] = digit.clone();
                }
            }
        }

        for digit in input_digits {
            if digit.len() == 5 {
                if encoding_map[1].is_subset(&digit) {
                    encoding_map[3] = digit;
                } else if digit.is_subset(&encoding_map[6]) {
                    encoding_map[5] = digit;
                } else {
                    encoding_map[2] = digit;
                }
            }
        }

        let base: u32 = 10;
        for (power_of_10_index, digit) in output_digits.enumerate() {
            if vec![
                2.to_usize().unwrap(),
                3.to_usize().unwrap(),
                4.to_usize().unwrap(),
                7.to_usize().unwrap(),
            ]
            .contains(&digit.len())
            {
                part_1_count += 1;
            }

            for (number, encoding) in encoding_map.iter().enumerate() {
                if &digit == encoding {
                    part_2_count += base.pow(3 - power_of_10_index as u32) * number as u32;
                }
            }
        }
    }
    println!("{}", part_1_count);
    println!("{}", part_2_count);
}
