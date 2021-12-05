use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Vent {
    x1: i32,
    x2: i32,
    x_step: i32,
    y1: i32,
    y2: i32,
    y_step: i32,
}

fn build_vent(vent_data: String) -> Vent {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    }
    let cap = RE.captures(&vent_data).unwrap();
    let x1 = cap[1].parse::<i32>().unwrap();
    let x2 = cap[3].parse::<i32>().unwrap();
    let y1 = cap[2].parse::<i32>().unwrap();
    let y2 = cap[4].parse::<i32>().unwrap();

    Vent {
        x1,
        x2,
        x_step: if x1 > x2 { -1 } else { 1 },
        y1,
        y2,
        y_step: if y1 > y2 { -1 } else { 1 },
    }
}

pub(crate) fn day05() {
    let f: File = File::open("data/day05.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let list_of_vent_data: Vec<String> =
        reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut vents: Vec<Vent> = Vec::new();

    for vent_data in list_of_vent_data {
        vents.push(build_vent(vent_data));
    }

    let mut sea_floor: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for vent in vents {
        if vent.x1 == vent.x2 {
            for y in num::range_step_inclusive(vent.y1, vent.y2, vent.y_step) {
                sea_floor[vent.x1 as usize][y as usize] += 1;
            }
        } else if vent.y1 == vent.y2 {
            for x in num::range_step_inclusive(vent.x1, vent.x2, vent.x_step) {
                sea_floor[x as usize][vent.y1 as usize] += 1;
            }
        } else {
            for (x, y) in num::range_step_inclusive(vent.x1, vent.x2, vent.x_step)
                .zip(num::range_step_inclusive(vent.y1, vent.y2, vent.y_step))
            {
                sea_floor[x as usize][y as usize] += 1;
            }
        }
    }

    println!(
        "Part 2: {}",
        sea_floor.into_iter().flatten().filter(|x| x > &1).count()
    );
}
