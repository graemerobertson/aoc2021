use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use itertools::Itertools;

struct Board {
    rows: Vec<HashSet<u32>>,
    columns: Vec<HashSet<u32>>,
    has_won: bool,
}

impl Board {
    fn remove_number(&mut self, number: u32) {
        for row in &mut *self.rows {
            row.remove(&number);
        }
        for column in &mut *self.columns {
            column.remove(&number);
        }
    }

    fn check_for_win(&mut self) -> bool {
        for row in &self.rows {
            if row.is_empty() {
                self.has_won = true;
                return true;
            }
        }
        for column in &self.columns {
            if column.is_empty() {
                self.has_won = true;
                return true;
            }
        }
        false
    }

    fn calculate_score(&self, most_recent_number: u32) -> u32 {
        let mut sum_remaining_numbers: u32 = 0;
        for row in &self.rows {
            sum_remaining_numbers += row.iter().sum::<u32>();
        }
        sum_remaining_numbers * most_recent_number
    }
}

fn build_board(board_data: Vec<String>) -> Board {
    let mut rows: Vec<HashSet<u32>> = vec![HashSet::new(); 5];
    let mut columns: Vec<HashSet<u32>> = vec![HashSet::new(); 5];
    for (row_index, line) in board_data.into_iter().enumerate() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        for (column_index, number) in numbers.into_iter().enumerate() {
            rows[row_index].insert(number);
            columns[column_index].insert(number);
        }
    }
    Board {
        rows,
        columns,
        has_won: false,
    }
}

pub(crate) fn day04() {
    let f: File = File::open("data/day04.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let called_numbers: Vec<u32> = diagnostics[0]
        .split(",")
        .map(|x| x.clone().trim().parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    for lines in &diagnostics
        .into_iter()
        .filter(|x| x.len() > 0 && !x.contains(','))
        .chunks(5)
    {
        boards.push(build_board(lines.collect_vec()));
    }

    let mut found_best_board: bool = false;
    let mut most_recent_score: u32 = 0;

    for number in called_numbers {
        for board in &mut *boards {
            if !board.has_won {
                board.remove_number(number);
                if board.check_for_win() {
                    most_recent_score = board.calculate_score(number);
                    if !found_best_board {
                        println!("Best board final score is {}", most_recent_score);
                        found_best_board = true;
                    }
                }
            }
        }
    }
    println!("Worst board final score is {}", most_recent_score);
}
