use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Add;

lazy_static! {
    static ref THREEXTHREE_SQUARE_POINTS: Vec<Point> = vec![
        Point { x: 1, y: 1 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: -1, y: 0 },
        Point { x: 1, y: -1 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: -1 },
    ];
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub(crate) fn day20() {
    let f: File = File::open("data/day20.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut diagnostics_iter = diagnostics.iter();
    let image_enhancement_algorithm: Vec<char> = diagnostics_iter.next().unwrap().chars().collect();
    diagnostics_iter.next();
    let mut input_image: HashMap<Point, char> = HashMap::new();
    for (column_index, line) in diagnostics_iter.enumerate() {
        for (row_index, c) in line.chars().enumerate() {
            input_image.insert(
                Point {
                    x: row_index as i32,
                    y: column_index as i32,
                },
                c,
            );
        }
    }

    let mut output_image: HashMap<Point, char> = HashMap::new();
    for iteration in 1..51 {
        output_image.clear();
        process_image(
            &mut input_image,
            &mut output_image,
            &image_enhancement_algorithm,
            iteration,
        );
        drop(input_image);
        input_image = output_image.clone();

        if iteration == 2 {
            println!("Part 1: {}", count_light_points(&output_image));
        }
    }

    println!("Part 2: {}", count_light_points(&output_image));
}

fn count_light_points(image: &HashMap<Point, char>) -> usize {
    return image
        .values()
        .collect::<Vec<&char>>()
        .iter()
        .filter(|x| **x == &'#')
        .count();
}

fn process_image(
    input_image: &mut HashMap<Point, char>,
    output_image: &mut HashMap<Point, char>,
    algorithm: &[char],
    iteration: usize,
) {
    let mut default_char: char = '.';
    if iteration % 2 == 0 {
        default_char = '#';
    }
    for input_point in input_image.clone().keys() {
        for square_point in THREEXTHREE_SQUARE_POINTS.iter() {
            if !input_image.contains_key(&(input_point + square_point)) {
                input_image.insert((input_point + square_point).to_owned(), default_char);
            }
        }
    }
    for input_point in input_image.keys() {
        let mut algorithm_lookup: usize = 0;
        for (binary_index, square_point) in THREEXTHREE_SQUARE_POINTS.iter().enumerate() {
            if input_image
                .get(&(input_point + square_point))
                .unwrap_or(&default_char)
                == &'#'
            {
                algorithm_lookup += 2_i32.pow(binary_index as u32) as usize;
            }
        }
        output_image.insert(*input_point, *algorithm.get(algorithm_lookup).unwrap());
    }
}
