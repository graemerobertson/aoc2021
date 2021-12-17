const X_MAX: i32 = 182;
const X_MIN: i32 = 155;
const Y_MAX: i32 = -67;
const Y_MIN: i32 = -117;
// Part 1, calculated in my head because it was just obvious through basic math.
const Y_INITIAL_VELOCITY_MAX: i32 = 116;

pub(crate) fn day17() {
    let mut possible_trajectories: u32 = 0;
    for x_initial_velocity in 0..X_MAX + 1 {
        for y_initial_velocity in Y_MIN..Y_INITIAL_VELOCITY_MAX + 1 {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut x_current_velocity: i32 = x_initial_velocity;
            let mut y_current_velocity: i32 = y_initial_velocity;
            loop {
                if x > X_MAX || y < Y_MIN {
                    // We've already missed the target
                    break;
                } else if X_MIN <= x && x <= X_MAX && Y_MIN <= y && y <= Y_MAX {
                    // We've hit the target
                    possible_trajectories += 1;
                    break;
                } else {
                    // Take another step
                    x += x_current_velocity;
                    y += y_current_velocity;
                    if x_current_velocity > 0 {
                        x_current_velocity -= 1;
                    }
                    y_current_velocity -= 1;
                }
            }
        }
    }
    println!(
        "Number of possible initial trajectories: {}",
        possible_trajectories
    );
}
