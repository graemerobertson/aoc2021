use lazy_static::lazy_static;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

lazy_static! {
    static ref SURROUNDING_POINTS: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
}

pub(crate) fn day11() {
    let f: File = File::open("data/day11.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; 10]; 10];

    for (i, line) in diagnostics.iter().enumerate() {
        for (j, point) in line.chars().enumerate() {
            grid[i][j] = point as u32 - 48;
        }
    }

    let mut total_flashes: u32 = 0;
    let mut step: u32 = 1;

    loop {
        let mut step_flashes: u32 = 0;
        for line in grid.iter_mut() {
            for point in line.iter_mut() {
                *point += 1;
            }
        }

        for (i, line) in grid.clone().iter().enumerate() {
            for (j, x) in line.iter().enumerate() {
                if x == &10 {
                    step_flashes += flash((i as i32, j as i32), &mut grid);
                }
            }
        }

        for line in grid.iter_mut() {
            for point in line.iter_mut() {
                if point > &mut 9 {
                    *point = 0;
                }
            }
        }

        total_flashes += step_flashes;

        if step == 100 {
            println!("Total flashes after 100 steps: {}", total_flashes);
        }
        if step_flashes == 100 {
            println!("All octopuses flashing on step {}", step);
            break;
        }

        step += 1;
    }
}

fn flash(point: (i32, i32), grid: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes_count: u32 = 1;

    for delta in SURROUNDING_POINTS.iter() {
        let new_point = (point.0 + delta.0, point.1 + delta.1);
        if new_point.0 >= 0 && new_point.1 >= 0 && new_point.0 < 10 && new_point.1 < 10 {
            grid[new_point.0 as usize][new_point.1 as usize] += 1;
            if grid[new_point.0 as usize][new_point.1 as usize] == 10 {
                flashes_count += flash(new_point, grid);
            }
        }
    }
    flashes_count
}
