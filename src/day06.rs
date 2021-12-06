use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAYS: u128 = 256;

pub(crate) fn day06() {
    let f: File = File::open("data/day06.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let school: Vec<u128> = diagnostics[0]
        .split(',')
        .map(|x| x.trim().parse::<u128>().unwrap())
        .collect();

    let mut fish_timer_counts: Vec<u128> = vec![0; 7];
    let mut baby_fish_timer_counts: Vec<u128> = vec![0; 9];
    for fish in school {
        fish_timer_counts[fish as usize] += 1;
    }

    let mut births: u128;
    for _ in 0..DAYS {
        births = baby_fish_timer_counts[0] + fish_timer_counts[0];

        for i in 0..8 {
            baby_fish_timer_counts[i] = baby_fish_timer_counts[i + 1];
        }
        baby_fish_timer_counts[8] = births;

        for i in 0..6 {
            fish_timer_counts[i] = fish_timer_counts[i + 1];
        }

        // Number of births is the same as the number of new parents.
        fish_timer_counts[6] = births;
    }

    println!(
        "Number of fish after {} days: {}",
        DAYS,
        fish_timer_counts.into_iter().sum::<u128>()
            + baby_fish_timer_counts.into_iter().sum::<u128>()
    );
}
