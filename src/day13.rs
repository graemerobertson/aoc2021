use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct FoldRule {
    x_fold: bool,
    crease: u32,
}

pub(crate) fn day13() {
    let f: File = File::open("data/day13.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut diagnostics_iter = diagnostics.iter();

    // Read lines until we find a blank line - these are the co-ordinates.
    let mut coordinates: HashSet<(u32, u32)> = HashSet::new();
    let mut line = diagnostics_iter.next().unwrap();
    while !line.is_empty() {
        let mut points = line.split(',');
        coordinates.insert((
            points.next().unwrap().parse::<u32>().unwrap(),
            points.next().unwrap().parse::<u32>().unwrap(),
        ));
        line = diagnostics_iter.next().unwrap();
    }

    // The remaining lines are fold rules.
    let mut rules: Vec<FoldRule> = Vec::new();
    for line in diagnostics_iter {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
        }
        let cap = RE.captures(line).unwrap();
        rules.push(FoldRule {
            x_fold: cap[1].parse::<char>().unwrap() == 'x',
            crease: cap[2].parse::<u32>().unwrap(),
        });
    }

    // Build a new set of co-ordinates after each fold.
    let mut new_coordinates: HashSet<(u32, u32)> = HashSet::new();
    for (rule_number, rule) in rules.iter().enumerate() {
        new_coordinates.clear();
        for point in &coordinates {
            if rule.x_fold {
                match point.0.cmp(&rule.crease) {
                    Ordering::Less => {
                        new_coordinates.insert((point.0, point.1));
                    }
                    Ordering::Greater => {
                        new_coordinates.insert((rule.crease * 2 - point.0, point.1));
                    }
                    Ordering::Equal => {}
                }
            } else {
                match point.1.cmp(&rule.crease) {
                    Ordering::Less => {
                        new_coordinates.insert((point.0, point.1));
                    }
                    Ordering::Greater => {
                        new_coordinates.insert((point.0, rule.crease * 2 - point.1));
                    }
                    Ordering::Equal => {}
                }
            }
        }

        if rule_number == 0 {
            println!("{} visible points after 1 fold", new_coordinates.len());
        }

        // Prepare for the next loop.
        if rule_number != rules.len() {
            coordinates = new_coordinates.clone();
        }
    }

    // Figure out the size of the remaining piece of paper.
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    for point in &new_coordinates {
        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    // Print the result in the correct orientation.
    for i in 0..max_y + 1 {
        for j in 0..max_x + 1 {
            if new_coordinates.contains(&(j, i)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
