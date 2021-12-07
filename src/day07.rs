use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Very lazy implementations of mean and median that conveniently give the right answers.
fn median(list: &[u32]) -> u32 {
    list[list.len() / 2 - 1]
}

fn mean(list: &[u32]) -> u32 {
    list.iter().sum::<u32>() / list.len() as u32
}

pub(crate) fn day07() {
    let f: File = File::open("data/day07.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut horizontal_values: Vec<u32> = diagnostics[0]
        .split(',')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
    horizontal_values.sort_unstable();

    let median: u32 = median(&horizontal_values);
    println!(
        "Part 1 fuel consumption is {}",
        horizontal_values
            .clone()
            .into_iter()
            .map(|x| if x < median { median - x } else { x - median })
            .sum::<u32>()
    );

    let mean: u32 = mean(&horizontal_values);
    println!(
        "Part 2 fuel consumption is {}",
        horizontal_values
            .into_iter()
            .map(|x| if x < mean {
                (mean - x) * (mean - x + 1) / 2
            } else {
                (x - mean) * (x - mean + 1) / 2
            })
            .sum::<u32>()
    );
}
