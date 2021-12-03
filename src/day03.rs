use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day03() {
    let f: File = File::open("data/day03.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut column_bits: Vec<String> = Vec::new();

    for binary_number in diagnostics.clone() {
        for (index, c) in binary_number.chars().enumerate() {
            if column_bits.get(index).is_none() {
                column_bits.push("".to_string());
            }
            column_bits[index].push(c);
        }
    }

    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();

    for column in column_bits {
        match column.matches("0").count() > column.matches("1").count() {
            true => {
                gamma_rate += "0";
                epsilon_rate += "1";
            }
            false => {
                gamma_rate += "1";
                epsilon_rate += "0";
            }
        }
    }

    println!(
        "Power consumption: {}",
        isize::from_str_radix(&gamma_rate, 2).unwrap()
            * isize::from_str_radix(&epsilon_rate, 2).unwrap()
    );

    println!(
        "Life support rating: {}",
        filter_diagnostics(diagnostics.clone(), true)
            * filter_diagnostics(diagnostics.clone(), false)
    );
}

fn filter_diagnostics(diagnostics: Vec<String>, oxygen: bool) -> isize {
    let mut filtered_diagnostics: Vec<String> = diagnostics.clone();
    let mut column_index = 0;
    while filtered_diagnostics.len() > 1 {
        filtered_diagnostics =
            filter_diagnostics_on_column(filtered_diagnostics, column_index, oxygen);
        column_index += 1;
    }
    isize::from_str_radix(filtered_diagnostics.get(0).unwrap(), 2).unwrap()
}

fn filter_diagnostics_on_column(
    diagnostics: Vec<String>,
    column_index: usize,
    oxygen: bool,
) -> Vec<String> {
    // Delta is the total number of 0s subtracted from the total number of 1s
    // in column_index
    let mut delta = 0;
    for binary_number in diagnostics.clone() {
        match binary_number.chars().nth(column_index).unwrap() {
            '1' => {
                delta += 1;
            }
            '0' => {
                delta -= 1;
            }
            _ => {
                panic!("Invalid diagnostic!")
            }
        }
    }
    let mut filtered_diagnostics: Vec<String> = Vec::new();

    let mut keeper: char = '0';
    if oxygen {
        if delta >= 0 {
            keeper = '1';
        }
    } else {
        if delta < 0 {
            keeper = '1';
        }
    }

    for binary_number in diagnostics {
        if binary_number.chars().nth(column_index).unwrap() == keeper {
            filtered_diagnostics.push(binary_number.clone());
        }
    }

    filtered_diagnostics
}
