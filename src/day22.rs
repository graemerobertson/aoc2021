use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
    on: bool,
}

impl Cuboid {
    fn new(line: &str) -> Cuboid {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$")
                    .unwrap();
        }
        let cap = RE.captures(line).unwrap();
        Cuboid {
            x1: cap[2].parse::<i64>().unwrap(),
            x2: cap[3].parse::<i64>().unwrap(),
            y1: cap[4].parse::<i64>().unwrap(),
            y2: cap[5].parse::<i64>().unwrap(),
            z1: cap[6].parse::<i64>().unwrap(),
            z2: cap[7].parse::<i64>().unwrap(),
            on: &cap[1] == "on",
        }
    }

    fn count_points(&self) -> i64 {
        (self.x2 + 1 - self.x1) * (self.y2 + 1 - self.y1) * (self.z2 + 1 - self.z1)
    }
}

fn intersection(c1: &Cuboid, c2: &Cuboid, on: bool) -> Option<Cuboid> {
    if c1.x1 > c2.x2
        || c2.x1 > c1.x2
        || c1.y1 > c2.y2
        || c2.y1 > c1.y2
        || c1.z1 > c2.z2
        || c2.z1 > c1.z2
    {
        return None;
    }
    Some(Cuboid {
        x1: cmp::max(c1.x1, c2.x1),
        x2: cmp::min(c1.x2, c2.x2),
        y1: cmp::max(c1.y1, c2.y1),
        y2: cmp::min(c1.y2, c2.y2),
        z1: cmp::max(c1.z1, c2.z1),
        z2: cmp::min(c1.z2, c2.z2),
        on,
    })
}

pub(crate) fn day22() {
    let f: File = File::open("data/day22.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut processed_cuboids: Vec<Cuboid> = Vec::new();
    for line in diagnostics {
        let cuboid = Cuboid::new(&line);
        for prev_cuboid in processed_cuboids.clone() {
            // The Lord giveth and the Lord taketh away.
            if let Some(new_intersection) = intersection(&cuboid, &prev_cuboid, !prev_cuboid.on) {
                processed_cuboids.push(new_intersection);
            }
        }
        if cuboid.on {
            processed_cuboids.push(cuboid);
        }
    }

    println!(
        "{}",
        processed_cuboids
            .iter()
            .map(|x| if x.on {
                x.count_points()
            } else {
                -x.count_points()
            })
            .sum::<i64>()
    );
}
