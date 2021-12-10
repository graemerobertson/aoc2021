use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const UNEXPECTED_ROUND: u128 = 3;
const UNEXPECTED_SQUARE: u128 = 57;
const UNEXPECTED_SQUIGGLY: u128 = 1197;
const UNEXPECTED_ANGLED: u128 = 25137;
const MISSING_ROUND: u128 = 1;
const MISSING_SQUARE: u128 = 2;
const MISSING_SQUIGGLY: u128 = 3;
const MISSING_ANGLED: u128 = 4;

pub(crate) fn day10() {
    let f: File = File::open("data/day10.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let bracket_pairings: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let scores: HashMap<char, (u128, u128)> = HashMap::from([
        (')', (UNEXPECTED_ROUND, MISSING_ROUND)),
        (']', (UNEXPECTED_SQUARE, MISSING_SQUARE)),
        ('}', (UNEXPECTED_SQUIGGLY, MISSING_SQUIGGLY)),
        ('>', (UNEXPECTED_ANGLED, MISSING_ANGLED)),
    ]);

    let mut part_1_score: u128 = 0;
    let mut possible_part_2_scores: Vec<u128> = Vec::new();

    for line in diagnostics {
        let mut expected_closing_brackets: Vec<char> = Vec::new();
        let mut corrupt: bool = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    expected_closing_brackets.push(*bracket_pairings.get(&c).unwrap());
                }
                ')' | ']' | '}' | '>' => {
                    if expected_closing_brackets.pop().unwrap() != c {
                        part_1_score += scores.get(&c).unwrap().0;
                        corrupt = true;
                        break;
                    }
                }
                _ => {
                    panic!("Line is really corrupt");
                }
            }
        }

        if !corrupt {
            let mut part_2_score: u128 = 0;
            expected_closing_brackets.reverse();
            for c in expected_closing_brackets {
                part_2_score *= 5;
                part_2_score += scores.get(&c).unwrap().1;
            }
            possible_part_2_scores.push(part_2_score);
        }
    }
    println!("Part 1: {}", part_1_score);
    possible_part_2_scores.sort_unstable();
    println!(
        "Part 2: {}",
        possible_part_2_scores
            .get((possible_part_2_scores.len() - 1) / 2)
            .unwrap()
    );
}
