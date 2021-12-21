const PLAYER_1_INITIAL_POSITION: u32 = 4;
const PLAYER_2_INITIAL_POSITION: u32 = 10;
const PART_1_WINNING_SCORE: u32 = 1000;
const PART_2_WINNING_SCORE: u32 = 21;

pub(crate) fn day21() {
    println!("Part 1: {}", play_practice_game());
    println!("Part 2: {}", play_real_game());
}

fn play_real_game() -> u128 {
    let player_scores: Vec<u32> = vec![0; 2];
    let player_positions: Vec<u32> = vec![PLAYER_1_INITIAL_POSITION, PLAYER_2_INITIAL_POSITION];
    let mut player_wins: Vec<u128> = vec![0; 2];
    next_turn(player_scores, player_positions, &mut player_wins, 1, 0);
    *player_wins.iter().max().unwrap()
}

fn next_turn(
    player_scores: Vec<u32>,
    player_positions: Vec<u32>,
    player_wins: &mut Vec<u128>,
    current_number_of_universes: u128,
    player_turn: usize,
) {
    // Each turn creates 27 new universes and in each of those universes the sum of the
    // three dice rolls is a number between 3 and 9 in the following distribution.
    for possibilities in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut forked_player_scores = player_scores.clone();
        let mut forked_player_positions = player_positions.clone();

        forked_player_positions[player_turn] =
            (forked_player_positions[player_turn] + possibilities.0) % 10;
        if forked_player_positions[player_turn] == 0 {
            forked_player_positions[player_turn] += 10;
        }
        forked_player_scores[player_turn] += forked_player_positions[player_turn];

        if forked_player_scores[player_turn] >= PART_2_WINNING_SCORE {
            player_wins[player_turn] += current_number_of_universes * possibilities.1 as u128;
        } else {
            next_turn(
                forked_player_scores,
                forked_player_positions,
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
            player_positions[p] = (player_positions[p] + 3 * dice_counter + 6) % 10;
            if player_positions[p] == 0 {
                player_positions[p] += 10;
            }
            player_scores[p] += player_positions[p];
            dice_counter += 3;
            if player_scores[p] >= PART_1_WINNING_SCORE {
                return player_scores[(p + 1) % 2] * dice_counter;
            }
        }
    }
}
