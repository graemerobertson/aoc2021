use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day12() {
    let f: File = File::open("data/day12.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    // Parse the input into a map of caves to adjacent caves. This code is
    // embarrassingly verbose but I can't work out how to collapse it.
    let mut links: HashMap<String, Vec<String>> = HashMap::new();
    for line in diagnostics {
        let caves = line.split('-').collect::<Vec<&str>>();
        if links.contains_key(caves[0]) {
            links.get_mut(caves[0]).unwrap().push(caves[1].to_string());
        } else {
            links.insert(caves[0].to_string(), vec![caves[1].to_string(); 1]);
        }
        if links.contains_key(caves[1]) {
            links.get_mut(caves[1]).unwrap().push(caves[0].to_string());
        } else {
            links.insert(caves[1].to_string(), vec![caves[0].to_string(); 1]);
        }
    }

    println!("Part 1: {}", count_paths_to_end(&[], "start", &links, true));
    println!(
        "Part 2: {}",
        count_paths_to_end(&[], "start", &links, false)
    );
}

fn count_paths_to_end(
    path_so_far: &[String],
    next_cave: &str,
    links: &HashMap<String, Vec<String>>,
    already_contains_repeat_small_cave: bool,
) -> u32 {
    let mut count: u32 = 0;
    for next_cave in links.get(next_cave).unwrap() {
        if next_cave == "end" {
            // We've found a complete path
            count += 1;
        } else if next_cave != "start" {
            // If we were back at start, we should drop this option
            if &next_cave.to_lowercase() == next_cave && path_so_far.contains(next_cave) {
                // We're re-visiting a small cave
                if !already_contains_repeat_small_cave {
                    let mut updated_path_so_far = path_so_far.to_owned();
                    updated_path_so_far.push(next_cave.to_string());
                    count += count_paths_to_end(&updated_path_so_far, next_cave, links, true);
                }
            } else {
                let mut updated_path_so_far = path_so_far.to_owned();
                updated_path_so_far.push(next_cave.to_string());
                count += count_paths_to_end(
                    &updated_path_so_far,
                    next_cave,
                    links,
                    already_contains_repeat_small_cave,
                );
            }
        }
    }
    count
}
