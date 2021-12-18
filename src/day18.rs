use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum SnailfishNumberCharacters {
    Char(char),
    Number(u32),
}

fn explode_snailfish_number(snailfish_number: &mut Vec<SnailfishNumberCharacters>) -> bool {
    let mut count_open_brackets: u32 = 0;
    let mut exploded_snailfish_number: Vec<SnailfishNumberCharacters> = Vec::new();
    let mut prev_number_index: usize = 0;
    for (c_index, c) in snailfish_number.clone().iter().enumerate() {
        match c {
            SnailfishNumberCharacters::Char('[') => count_open_brackets += 1,
            SnailfishNumberCharacters::Char(']') => count_open_brackets -= 1,
            SnailfishNumberCharacters::Number(_) => prev_number_index = c_index,
            _ => {}
        }
        if count_open_brackets > 4 {
            exploded_snailfish_number.push(SnailfishNumberCharacters::Number(0));
            exploded_snailfish_number.extend_from_slice(
                snailfish_number
                    .get(c_index + 5..snailfish_number.len())
                    .unwrap(),
            );

            if prev_number_index > 0 {
                if let SnailfishNumberCharacters::Number(lhexplosion) =
                    snailfish_number.get(c_index + 1).unwrap()
                {
                    if let SnailfishNumberCharacters::Number(mut prev_number) =
                        exploded_snailfish_number
                            .get_mut(prev_number_index)
                            .unwrap()
                    {
                        prev_number += lhexplosion;
                        exploded_snailfish_number[prev_number_index] =
                            SnailfishNumberCharacters::Number(prev_number);
                    }
                }
            }

            let reduced_snailfish_number_len = exploded_snailfish_number.len();
            for inner_c in exploded_snailfish_number
                .get_mut(c_index + 1..reduced_snailfish_number_len)
                .unwrap()
            {
                if let SnailfishNumberCharacters::Number(unwrapped_inner_c) = inner_c {
                    if let SnailfishNumberCharacters::Number(rhexplosion) =
                        snailfish_number.get(c_index + 3).unwrap()
                    {
                        *inner_c =
                            SnailfishNumberCharacters::Number(*unwrapped_inner_c + rhexplosion);

                        break;
                    }
                }
            }
            *snailfish_number = exploded_snailfish_number;
            return true;
        } else {
            exploded_snailfish_number.push(*c);
        }
    }
    false
}

fn split_snailfish_number(snailfish_number: &mut Vec<SnailfishNumberCharacters>) -> bool {
    let mut split_snailfish_number: Vec<SnailfishNumberCharacters> = Vec::new();
    for (c_index, c) in snailfish_number.iter().enumerate() {
        if let SnailfishNumberCharacters::Number(unwrapped_number) = c {
            if unwrapped_number > &9 {
                split_snailfish_number.extend([
                    SnailfishNumberCharacters::Char('['),
                    SnailfishNumberCharacters::Number(unwrapped_number / 2),
                    SnailfishNumberCharacters::Char(','),
                    SnailfishNumberCharacters::Number(unwrapped_number / 2 + unwrapped_number % 2),
                    SnailfishNumberCharacters::Char(']'),
                ]);
                split_snailfish_number.extend_from_slice(
                    snailfish_number
                        .get(c_index + 1..snailfish_number.len())
                        .unwrap(),
                );
                *snailfish_number = split_snailfish_number;
                return true;
            }
        }
        split_snailfish_number.push(*c);
    }
    false
}

fn reduce_snailfish_number(snailfish_number: &mut Vec<SnailfishNumberCharacters>) {
    loop {
        if explode_snailfish_number(snailfish_number) {
            continue;
        }
        if !split_snailfish_number(snailfish_number) {
            break;
        }
    }
}

fn parse_snailfish_number(string_representation: &str) -> Vec<SnailfishNumberCharacters> {
    let mut snailfish_number: Vec<SnailfishNumberCharacters> = Vec::new();
    for c in string_representation.chars() {
        if c == '[' || c == ']' || c == ',' {
            snailfish_number.push(SnailfishNumberCharacters::Char(c));
        } else {
            snailfish_number.push(SnailfishNumberCharacters::Number(c.to_digit(10).unwrap()));
        }
    }
    snailfish_number
}

fn calculate_snailfish_number_magnitude(snailfish_number: &[SnailfishNumberCharacters]) -> u32 {
    if let SnailfishNumberCharacters::Number(value) = snailfish_number.get(0).unwrap() {
        return *value;
    }

    let mut this_element_comma_index = 0;
    let mut count_open_brackets = 0;
    for (c_index, c) in snailfish_number.iter().enumerate() {
        match c {
            SnailfishNumberCharacters::Char('[') => count_open_brackets += 1,
            SnailfishNumberCharacters::Char(']') => count_open_brackets -= 1,
            SnailfishNumberCharacters::Char(',') => {
                if count_open_brackets == 1 {
                    this_element_comma_index = c_index;
                }
            }
            _ => {}
        }
    }
    let mut first_element: Vec<SnailfishNumberCharacters> = Vec::new();
    first_element.extend_from_slice(snailfish_number.get(1..this_element_comma_index).unwrap());
    let mut second_element: Vec<SnailfishNumberCharacters> = Vec::new();
    second_element.extend_from_slice(
        snailfish_number
            .get(this_element_comma_index + 1..snailfish_number.len() - 1)
            .unwrap(),
    );
    3 * calculate_snailfish_number_magnitude(&first_element)
        + 2 * calculate_snailfish_number_magnitude(&second_element)
}

pub(crate) fn day18() {
    let f: File = File::open("data/day18.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut diagnostics_iter = diagnostics.iter();

    let mut lhs: Vec<SnailfishNumberCharacters> =
        parse_snailfish_number(diagnostics_iter.next().unwrap());

    let mut new_snailfish_number: Vec<SnailfishNumberCharacters> = Vec::new();
    for rhs in diagnostics_iter {
        new_snailfish_number.clear();
        new_snailfish_number.push(SnailfishNumberCharacters::Char('['));
        new_snailfish_number.extend(lhs);
        new_snailfish_number.push(SnailfishNumberCharacters::Char(','));
        new_snailfish_number.extend(parse_snailfish_number(rhs));
        new_snailfish_number.push(SnailfishNumberCharacters::Char(']'));
        reduce_snailfish_number(&mut new_snailfish_number);
        lhs = new_snailfish_number.clone();
    }
    println!("Part 1: {:?}", calculate_snailfish_number_magnitude(&lhs));

    let mut possible_magnitudes: Vec<u32> = Vec::new();
    for lhs in &diagnostics {
        for rhs in &diagnostics {
            let mut part2_snailfish_number: Vec<SnailfishNumberCharacters> =
                vec![SnailfishNumberCharacters::Char('[')];
            part2_snailfish_number.extend(parse_snailfish_number(lhs));
            part2_snailfish_number.push(SnailfishNumberCharacters::Char(','));
            part2_snailfish_number.extend(parse_snailfish_number(rhs));
            part2_snailfish_number.push(SnailfishNumberCharacters::Char(']'));
            reduce_snailfish_number(&mut part2_snailfish_number);
            possible_magnitudes.push(calculate_snailfish_number_magnitude(
                &part2_snailfish_number,
            ));
        }
    }
    println!("Part 2: {}", possible_magnitudes.iter().max().unwrap());
}
