const PLAYER_1_INITIAL_POSITION: u32 = 4;
const PLAYER_2_INITIAL_POSITION: u32 = 10;
const PART_1_WINNING_SCORE: u32 = 1000;
const PART_2_WINNING_SCORE: u32 = 21;

pub(crate) fn day21() {
    println!("Part 1: {}", play_practice_game());
    println!("Part 2: {}", play_real_game());
}

fn play_real_game() -> u128 {
    let mut player_wins: Vec<u128> = vec![0; 2];

    // Use individual values for player scores and positions rather than a vector
    // because it's so much faster than cloning vectors for each universe.
    next_turn(
        0,
        0,
        PLAYER_1_INITIAL_POSITION,
        PLAYER_2_INITIAL_POSITION,
        &mut player_wins,
        1,
        0,
    );
    *player_wins.iter().max().unwrap()
}

fn calculate_new_score_and_position(position: &mut u32, score: &mut u32, progression: u32) {
    *position = (*position + progression) % 10;
    if position == &0 {
        *position += 10;
    }
    *score += *position;
}

fn next_turn(
    player_1_score: u32,
    player_2_score: u32,
    player_1_position: u32,
    player_2_position: u32,
    player_wins: &mut Vec<u128>,
    current_number_of_universes: u128,
    player_turn: usize,
) {
    // Each turn creates 27 new universes and in each of those universes the sum of the
    // three dice rolls is a number between 3 and 9 in the following distribution.
    for possibilities in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut forked_player_1_position = player_1_position;
        let mut forked_player_1_score = player_1_score;
        let mut forked_player_2_position = player_2_position;
        let mut forked_player_2_score = player_2_score;

        if player_turn == 0 {
            calculate_new_score_and_position(
                &mut forked_player_1_position,
                &mut forked_player_1_score,
                possibilities.0,
            );
        } else {
            calculate_new_score_and_position(
                &mut forked_player_2_position,
                &mut forked_player_2_score,
                possibilities.0,
            );
        }

        if forked_player_1_score >= PART_2_WINNING_SCORE
            || forked_player_2_score >= PART_2_WINNING_SCORE
        {
            player_wins[player_turn] += current_number_of_universes * possibilities.1 as u128;
        } else {
            next_turn(
                forked_player_1_score,
                forked_player_2_score,
                forked_player_1_position,
                forked_player_2_position,
                player_wins,
                current_number_of_universes * possibilities.1,
                (player_turn + 1) % 2,
            )
        }
    }
}

fn play_practice_game() -> u32 {
    let mut dice_counter: u32 = 0;
    let mut player_scores: Vec<u32> = vec![0; 2];
    let mut player_positions: Vec<u32> = vec![PLAYER_1_INITIAL_POSITION, PLAYER_2_INITIAL_POSITION];

    loop {
        for p in [0, 1] {
            calculate_new_score_and_position(
                &mut player_positions[p],
                &mut player_scores[p],
                3 * dice_counter + 6,
            );
            dice_counter += 3;
            if player_scores[p] >= PART_1_WINNING_SCORE {
                return player_scores[(p + 1) % 2] * dice_counter;
            }
        }
    }
}
