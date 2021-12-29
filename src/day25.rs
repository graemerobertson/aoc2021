use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day25() {
    let f: File = File::open("data/day25.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let length = diagnostics.get(0).unwrap().len();
    let height = diagnostics.len();
    let mut seabed = vec![vec!['.'; length]; height];
    for (row_index, line) in diagnostics.iter().enumerate() {
        for (column_index, c) in line.chars().enumerate() {
            seabed[row_index][column_index] = c;
        }
    }

    let mut steps = 0;
    loop {
        steps += 1;
        let mut seabed_after_right_moves = vec![vec!['.'; length]; height];
        for (row_index, line) in seabed.iter().enumerate() {
            for (column_index, c) in line.iter().enumerate() {
                if c == &'.' && seabed[row_index][(column_index + length - 1) % length] == '>' {
                    seabed_after_right_moves[row_index][column_index] = '>';
                } else if c == &'>'
                    && seabed[row_index][(column_index + length + 1) % length] == '.'
                {
                    seabed_after_right_moves[row_index][column_index] = '.';
                } else {
                    seabed_after_right_moves[row_index][column_index] = *c;
                }
            }
        }

        let mut seabed_after_down_moves = vec![vec!['.'; length]; height];
        for (row_index, line) in seabed_after_right_moves.iter().enumerate() {
            for (column_index, c) in line.iter().enumerate() {
                if c == &'.'
                    && seabed_after_right_moves[(row_index + height - 1) % height][column_index]
                        == 'v'
                {
                    seabed_after_down_moves[row_index][column_index] = 'v';
                } else if c == &'v'
                    && seabed_after_right_moves[(row_index + height + 1) % height][column_index]
                        == '.'
                {
                    seabed_after_down_moves[row_index][column_index] = '.';
                } else {
                    seabed_after_down_moves[row_index][column_index] = *c;
                }
            }
        }

        if seabed_after_down_moves == seabed {
            println!("{}", steps);
            return;
        } else {
            seabed = seabed_after_down_moves;
        }
    }
}
