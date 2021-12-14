use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pair {
    first_c: char,
    second_c: char,
}

impl Pair {
    fn string(&self) -> String {
        self.first_c.to_string() + &self.second_c.to_string()
    }
}

pub(crate) fn day14() {
    let f: File = File::open("data/day14.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut diagnostics_iter = diagnostics.iter();

    let polymer_template = diagnostics_iter.next().unwrap().to_owned();
    let mut character_counts: HashMap<char, u128> = HashMap::new();
    let mut pairing_counts: HashMap<Pair, u128> = HashMap::new();
    let mut polymer_template_iter = polymer_template.chars();
    let mut prev_c = polymer_template_iter.next().unwrap();
    character_counts.insert(prev_c, 1);

    for c in polymer_template_iter {
        *character_counts.entry(c).or_insert(0) += 1;
        *pairing_counts
            .entry(Pair {
                first_c: prev_c,
                second_c: c,
            })
            .or_insert(0) += 1;
        prev_c = c;
    }

    diagnostics_iter.next();
    let mut pair_insertion_rules: HashMap<String, char> = HashMap::new();
    for line in diagnostics_iter {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([A-Z][A-Z]) -> ([A-Z])$").unwrap();
        }
        let cap = RE.captures(line).unwrap();
        pair_insertion_rules.insert(
            cap[1].parse::<String>().unwrap(),
            cap[2].parse::<char>().unwrap(),
        );
    }

    for step in 1..41 {
        let mut new_pairing_counts: HashMap<Pair, u128> = HashMap::new();
        for (pair, count) in &pairing_counts {
            if pair_insertion_rules.contains_key(&pair.string()) {
                let new_c = pair_insertion_rules.get(&pair.string()).unwrap();
                *character_counts.entry(*new_c).or_insert(0) += count;

                for new_pair in &[
                    Pair {
                        first_c: pair.first_c,
                        second_c: *new_c,
                    },
                    Pair {
                        first_c: *new_c,
                        second_c: pair.second_c,
                    },
                ] {
                    *new_pairing_counts.entry(*new_pair).or_insert(0) += count;
                }
            } else {
                *new_pairing_counts.entry(*pair).or_insert(0) += count;
            }
        }
        pairing_counts = new_pairing_counts.clone();

        if step == 10 || step == 40 {
            println!(
                "Puzzle answer after step {}: {}",
                step,
                character_counts.iter().max_by_key(|f| f.1).unwrap().1
                    - character_counts.iter().min_by_key(|f| f.1).unwrap().1
            );
        }
    }
}
