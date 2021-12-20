use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Translation {
    x: i32,
    y: i32,
    z: i32,
    rotation_index_complex: usize,
    rotation_index_simple: usize,
}

pub(crate) fn day19() {
    let f: File = File::open("data/day19.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut scanners: HashMap<u32, HashSet<Coord>> = HashMap::new();

    let mut current_scanner: u32 = 0;
    let mut current_scanner_beacons: HashSet<Coord> = HashSet::new();

    for line in diagnostics {
        if line.is_empty() {
            scanners.insert(current_scanner, current_scanner_beacons.clone());
        } else if line.starts_with("--- scanner") {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"--- scanner (\d+) ---$").unwrap();
            }
            let cap = RE.captures(&line).unwrap();
            current_scanner = cap[1].parse::<u32>().unwrap();
            current_scanner_beacons.clear();
        } else {
            let mut coords = line.split(',');
            current_scanner_beacons.insert(Coord {
                x: coords.next().unwrap().parse::<i32>().unwrap(),
                y: coords.next().unwrap().parse::<i32>().unwrap(),
                z: coords.next().unwrap().parse::<i32>().unwrap(),
            });
        }
    }
    scanners.insert(current_scanner, current_scanner_beacons.clone());

    let mut all_beacons: HashSet<Coord> = HashSet::new();
    let mut scanners_traversed: HashSet<u32> = HashSet::new();
    scanners_traversed.insert(0);
    all_beacons.extend(scanners.get(&(0 as u32)).unwrap());
    let mut all_translations: HashSet<Translation> = HashSet::new();
    all_translations.insert(Translation {
        x: 0,
        y: 0,
        z: 0,
        rotation_index_complex: 0,
        rotation_index_simple: 0,
    });
    loop {
        for target_scanner in 0..scanners.len() {
            if !scanners_traversed.contains(&(target_scanner as u32)) {
                continue;
            }
            for scanner in 1..scanners.len() {
                if scanners_traversed.contains(&(scanner as u32)) {
                    // We don't want to translate this, we'll match it on the next swing past
                    continue;
                }
                let target_scanner_beacons =
                    scanners.get(&(target_scanner as u32)).unwrap().clone();
                if scanner == target_scanner as usize {
                    // Not this.
                    continue;
                }
                let potential_matching_scanner_becaons = scanners.get(&(scanner as u32)).unwrap();
                'outer: for (target_beacon_index, target_beacon) in
                    target_scanner_beacons.iter().enumerate()
                {
                    if target_beacon_index > target_scanner_beacons.len() - 12 {
                        break;
                    }
                    for (beacon_index, beacon) in
                        potential_matching_scanner_becaons.iter().enumerate()
                    {
                        if beacon_index > potential_matching_scanner_becaons.len() - 12 {
                            break;
                        }
                        for rotation_index_complex in 0..8 {
                            for rotation_index_simple in 0..3 {
                                let translated_beacon = rotate_beacon(
                                    beacon,
                                    rotation_index_complex,
                                    rotation_index_simple,
                                );

                                let potential_translation: Translation = Translation {
                                    x: target_beacon.x - translated_beacon.x,
                                    y: target_beacon.y - translated_beacon.y,
                                    z: target_beacon.z - translated_beacon.z,
                                    rotation_index_complex,
                                    rotation_index_simple,
                                };

                                let mut translated_beacon_set: HashSet<Coord> = HashSet::new();
                                for gdr_beacon in potential_matching_scanner_becaons {
                                    let mut tmp_beacon = rotate_beacon(
                                        gdr_beacon,
                                        potential_translation.rotation_index_complex,
                                        potential_translation.rotation_index_simple,
                                    );
                                    tmp_beacon.x += potential_translation.x;
                                    tmp_beacon.y += potential_translation.y;
                                    tmp_beacon.z += potential_translation.z;
                                    translated_beacon_set.insert(tmp_beacon);
                                }
                                if translated_beacon_set
                                    .intersection(&target_scanner_beacons)
                                    .count()
                                    >= 12
                                {
                                    all_translations.insert(potential_translation);
                                    all_beacons.extend(translated_beacon_set.clone());
                                    scanners.insert(scanner as u32, translated_beacon_set);

                                    scanners_traversed.insert(scanner as u32);
                                    if scanners_traversed.len() == scanners.len() {
                                        println!(
                                            "Total number of beacons is {}",
                                            all_beacons.len()
                                        );
                                        let mut manhattan_distances: Vec<i32> = Vec::new();
                                        for g1 in &all_translations {
                                            for g2 in &all_translations {
                                                manhattan_distances.push(
                                                    (g1.x - g2.x).abs()
                                                        + (g1.y - g2.y).abs()
                                                        + (g1.z - g2.z).abs(),
                                                );
                                            }
                                        }
                                        println!(
                                            "Largest Manhattan distance is {}",
                                            manhattan_distances.iter().max().unwrap()
                                        );
                                        return;
                                    }
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn rotate_beacon(b: &Coord, rotation_index_complex: usize, rotation_index_simple: usize) -> Coord {
    let mut new_beacon: Coord = match rotation_index_complex {
        0 => Coord {
            x: b.x,
            y: b.y,
            z: b.z,
        },
        1 => Coord {
            x: b.x,
            y: -b.y,
            z: -b.z,
        },
        2 => Coord {
            x: b.x,
            y: b.z,
            z: -b.y,
        },
        3 => Coord {
            x: b.x,
            y: -b.z,
            z: b.y,
        },
        4 => Coord {
            x: -b.x,
            y: b.y,
            z: -b.z,
        },
        5 => Coord {
            x: -b.x,
            y: b.z,
            z: b.y,
        },
        6 => Coord {
            x: -b.x,
            y: -b.y,
            z: b.z,
        },
        7 => Coord {
            x: -b.x,
            y: -b.z,
            z: -b.y,
        },
        _ => panic!("Invalid rotation_index_complex"),
    };
    for _ in 0..rotation_index_simple {
        let tmp = new_beacon.x;
        new_beacon.x = new_beacon.y;
        new_beacon.y = new_beacon.z;
        new_beacon.z = tmp;
    }
    new_beacon
}
