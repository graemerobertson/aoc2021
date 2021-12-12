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
        let first_cave = links.entry(caves[0].to_string()).or_insert_with(|| vec![]);
        first_cave.push(caves[1].to_string());
        let second_cave = links.entry(caves[1].to_string()).or_insert_with(|| vec![]);
        second_cave.push(caves[0].to_string());
    }

    println!(
        "Part 1: {}",
        count_paths_to_end(&["start".to_string(); 1], &links, false)
    );
    println!(
        "Part 2: {}",
        count_paths_to_end(&["start".to_string(); 1], &links, true)
    );
}

fn count_paths_to_end(
    path_so_far: &[String],
    links: &HashMap<String, Vec<String>>,
    allowed_to_revisit_small_cave: bool,
) -> u32 {
    let mut count: u32 = 0;
    for next_cave in links.get(path_so_far.last().unwrap()).unwrap() {
        if next_cave == "start" {
            // Not interested
            continue;
        } else if next_cave == "end" {
            // We've found a complete path
            count += 1;
        } else {
            let mut now_allowed_to_revisit_small_cave = allowed_to_revisit_small_cave;
            if &next_cave.to_lowercase() == next_cave && path_so_far.contains(next_cave) {
                if !allowed_to_revisit_small_cave {
                    // Second repeat small cave, not interested
                    continue;
                } else {
                    now_allowed_to_revisit_small_cave = false;
                }
            }

            let mut updated_path_so_far = path_so_far.to_owned();
            updated_path_so_far.push(next_cave.to_string());
            count += count_paths_to_end(
                &updated_path_so_far,
                links,
                now_allowed_to_revisit_small_cave,
            );
        }
    }
    count
}
