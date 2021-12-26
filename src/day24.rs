use std::collections::HashSet;

// By inspection, the input program simplifies down to these 14 functions that act on
// each digit of the model number. At the end of each block, z depends purely on the
// new input digit and the value of z at the beginning of the block.
fn act_on_digit_0(w: u128, _prev_z: u128) -> u128 {
    w + 8
}

fn act_on_digit_1(w: u128, prev_z: u128) -> u128 {
    prev_z * 26 + w + 16
}

fn act_on_digit_2(w: u128, prev_z: u128) -> u128 {
    prev_z * 26 + w + 4
}

fn act_on_digit_3(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w + 11 {
        z = 26 * z + w + 1;
    }
    z
}

fn act_on_digit_4(w: u128, prev_z: u128) -> u128 {
    prev_z * 26 + w + 13
}

fn act_on_digit_5(w: u128, prev_z: u128) -> u128 {
    prev_z * 26 + w + 5
}

fn act_on_digit_6(w: u128, prev_z: u128) -> u128 {
    prev_z * 26 + w
}

fn act_on_digit_7(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w + 5 {
        z = 26 * z + w + 10;
    }
    z
}

fn act_on_digit_8(w: u128, prev_z: u128) -> u128 {
    prev_z * 26 + w + 7
}

fn act_on_digit_9(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w {
        z = 26 * z + w + 2;
    }
    z
}

fn act_on_digit_10(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w + 11 {
        z = 26 * z + w + 13;
    }
    z
}

fn act_on_digit_11(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w + 13 {
        z = 26 * z + w + 15;
    }
    z
}

fn act_on_digit_12(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w + 13 {
        z = 26 * z + w + 14;
    }
    z
}

fn act_on_digit_13(w: u128, prev_z: u128) -> u128 {
    let mut z = prev_z / 26;
    if (prev_z % 26) != w + 11 {
        z = 26 * z + w + 9;
    }
    z
}

// Recursive function that checks whether each 14 digit number is a valid model number
// and returns the first valid model number (as a string) it finds.
//
// The slightly random start_stop_step variable is used to determine how we work
// through potential values for each digit - and is just used to search from
// 99999999999999 downwards or 11111111111111 upwards.
fn process_next_digit(
    digit_index: usize,
    prev_z: u128,
    digit_functions: &[&dyn Fn(u128, u128) -> u128],
    z_states: &mut Vec<HashSet<u128>>,
    start_stop_step: (i32, i32, i32),
) -> Option<String> {
    // For each potential value for this digit.
    for w in num::range_step_inclusive(start_stop_step.0, start_stop_step.1, start_stop_step.2) {
        // Calculate the new z using the correct function.
        let z = digit_functions.get(digit_index).unwrap()(w as u128, prev_z);

        if digit_index == 13 {
            // We've now processed 14 digits - either we have a winner or we don't.
            if z == 0 {
                return Some(format!("{}", w));
            } else {
                continue;
            }
        }

        if z_states.get(digit_index).unwrap().contains(&z) {
            // We know how this story ends and there's no point in continuing.
            continue;
        } else {
            // Move onto the next digit.
            let ret = process_next_digit(
                digit_index + 1,
                z,
                digit_functions,
                z_states,
                start_stop_step,
            );
            if ret.is_some() {
                // We've found a valid model number.
                return Some(format!("{}{}", w, ret.unwrap()));
            }
            // We've not found a valid model number from this position. Record our
            // value of z in the appropriate z state set so that we can skip
            // processing if we reach this state in the future.
            z_states.get_mut(digit_index).unwrap().insert(z);
        }
    }
    None
}

fn find_highest_valid_model_number(
    digit_functions: &[&dyn Fn(u128, u128) -> u128],
    z_states: &mut Vec<HashSet<u128>>,
) -> Option<String> {
    process_next_digit(0, 0, digit_functions, z_states, (9, 1, -1))
}

fn find_lowest_valid_model_number(
    digit_functions: &[&dyn Fn(u128, u128) -> u128],
    z_states: &mut Vec<HashSet<u128>>,
) -> Option<String> {
    process_next_digit(0, 0, digit_functions, z_states, (1, 9, 1))
}

pub(crate) fn day24() {
    let mut all_z_states: Vec<HashSet<u128>> = Vec::new();
    for _ in 0..14 {
        all_z_states.push(HashSet::new());
    }
    let digit_functions: Vec<&dyn Fn(u128, u128) -> u128> = vec![
        &act_on_digit_0,
        &act_on_digit_1,
        &act_on_digit_2,
        &act_on_digit_3,
        &act_on_digit_4,
        &act_on_digit_5,
        &act_on_digit_6,
        &act_on_digit_7,
        &act_on_digit_8,
        &act_on_digit_9,
        &act_on_digit_10,
        &act_on_digit_11,
        &act_on_digit_12,
        &act_on_digit_13,
    ];

    // Search from the highest possible number downwards.
    println!(
        "Highest valid model number: {}",
        find_highest_valid_model_number(&digit_functions, &mut all_z_states).unwrap()
    );
    // Search from the lowest possible number upwards.
    println!(
        "Lowest valid model number: {}",
        find_lowest_valid_model_number(&digit_functions, &mut all_z_states).unwrap()
    );
}
