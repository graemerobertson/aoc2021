use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type BasinPoint = (usize, usize);

pub(crate) fn day09() {
    let f: File = File::open("data/day09.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    // Build a grid for the input data. We add borders with 9s because for the
    // purposes of the problem the borders might as well be 9, and it makes
    // subsequent computation easier.
    let mut grid: Vec<Vec<u32>> = vec![vec![9; diagnostics[0].len() + 2]; diagnostics.len() + 2];

    for (i, line) in diagnostics.iter().enumerate() {
        for (j, point) in line.chars().enumerate() {
            grid[i + 1][j + 1] = point as u32 - 48;
        }
    }

    let mut low_points: Vec<u32> = Vec::new();
    let mut basin_sizes: Vec<u32> = Vec::new();

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] < grid[i][j - 1]
                && grid[i][j] < grid[i][j + 1]
                && grid[i][j] < grid[i - 1][j]
                && grid[i][j] < grid[i + 1][j]
            {
                low_points.push(grid[i][j]);

                let mut basin_points: HashSet<BasinPoint> = HashSet::new();
                basin_points.insert((i, j));
                find_missing_points_in_basin((i, j), &mut basin_points, &grid);
                basin_sizes.push(basin_points.len() as u32);
            }
        }
    }
    println!(
        "Part 1: {}",
        low_points.iter().sum::<u32>() + low_points.len() as u32
    );

    // Sort in descending order
    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!(
        "Part 2: {}",
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    );
}

fn find_missing_points_in_basin(
    new_basin_point: BasinPoint,
    current_basin_points: &mut HashSet<BasinPoint>,
    grid: &[Vec<u32>],
) {
    let increments: Vec<i32> = vec![1, -1];
    for increment in &increments {
        let mut j: i32 = new_basin_point.1 as i32;
        while grid[new_basin_point.0][j as usize] != 9 {
            if !current_basin_points.contains(&(new_basin_point.0, j as usize)) {
                current_basin_points.insert((new_basin_point.0, j as usize));
                find_missing_points_in_basin(
                    (new_basin_point.0, j as usize),
                    current_basin_points,
                    grid,
                );
            }
            j += increment;
        }
    }
    for increment in &increments {
        let mut i: i32 = new_basin_point.0 as i32;
        while grid[i as usize][new_basin_point.1] != 9 {
            if !current_basin_points.contains(&(i as usize, new_basin_point.1)) {
                current_basin_points.insert((i as usize, new_basin_point.1));
                find_missing_points_in_basin(
                    (i as usize, new_basin_point.1),
                    current_basin_points,
                    grid,
                );
            }
            i += increment;
        }
    }
}
