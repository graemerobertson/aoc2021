use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Space {
    A,
    B,
    C,
    D,
    Empty,
}

const ROOM_SIZE: usize = 4;
const HALLWAY_LENGTH: usize = 11;
const A_ENERGY: u32 = 1;
const B_ENERGY: u32 = 10;
const C_ENERGY: u32 = 100;
const D_ENERGY: u32 = 1000;
const A_ROOM_HALLWAY_POSITION: usize = 2;
const B_ROOM_HALLWAY_POSITION: usize = 4;
const C_ROOM_HALLWAY_POSITION: usize = 6;
const D_ROOM_HALLWAY_POSITION: usize = 8;

struct AmphipodInfo {
    move_energy: u32,
    room_hallway_position: usize,
}

lazy_static! {
    static ref AMPHIPODS: HashMap<Space, AmphipodInfo> = hashmap! {
        Space::A => AmphipodInfo { move_energy: A_ENERGY, room_hallway_position: A_ROOM_HALLWAY_POSITION},
        Space::B => AmphipodInfo { move_energy: B_ENERGY, room_hallway_position: B_ROOM_HALLWAY_POSITION},
        Space::C => AmphipodInfo { move_energy: C_ENERGY, room_hallway_position: C_ROOM_HALLWAY_POSITION},
        Space::D => AmphipodInfo { move_energy: D_ENERGY, room_hallway_position: D_ROOM_HALLWAY_POSITION},
    };
}

pub(crate) fn day23() {
    let hallway: Vec<Space> = vec![Space::Empty; HALLWAY_LENGTH];
    let amphipod_rooms: HashMap<Space, Vec<Space>> = hashmap! {
        Space::A => vec![Space::C, Space::D, Space::D, Space::B],
        Space::B => vec![Space::D, Space::B, Space::C, Space::C],
        Space::C => vec![Space::D, Space::A, Space::B, Space::A],
        Space::D => vec![Space::A, Space::C, Space::A, Space::B],
    };
    let mut min_energy: u32 = 0;
    run(&hallway, &amphipod_rooms, 0, &mut min_energy);
    println!("{}", min_energy);
}

fn run(
    hallway: &[Space],
    amphipod_rooms: &HashMap<Space, Vec<Space>>,
    current_energy_spent: u32,
    min_energy: &mut u32,
) {
    // For each room...
    for (room_type, room) in amphipod_rooms {
        // If we need to move amphipods out of the room...
        if room_contains_foreigners(room_type, room) {
            // Find the hallway spots that the closest amphipod can move into, and
            // try every option
            for space in free_hallway_spots(
                hallway,
                AMPHIPODS.get(room_type).unwrap().room_hallway_position,
            ) {
                let mut forked_hallway = hallway.to_owned();
                let mut forked_room = room.clone();
                let mut forked_amphipod_rooms = amphipod_rooms.clone();
                let mut forked_current_energy_spent = current_energy_spent;

                // Grim code for calculating energy for this move.
                let mut pos1 = space;
                let mut pos2 = AMPHIPODS.get(room_type).unwrap().room_hallway_position;
                if pos1 > pos2 {
                    pos1 = pos2;
                    pos2 = space;
                }
                forked_current_energy_spent += AMPHIPODS
                    .get(forked_room.last().unwrap())
                    .unwrap()
                    .move_energy
                    * (ROOM_SIZE + 1 - forked_room.len() + pos2 - pos1) as u32;

                // Perform the move.
                forked_hallway[space] = forked_room.pop().unwrap();
                forked_amphipod_rooms.insert(*room_type, forked_room);

                // Send any amphipods home that have been freed up.
                forked_current_energy_spent +=
                    flush_hallway_and_rooms(&mut forked_hallway, &mut forked_amphipod_rooms);

                // If we've not already exceeded the minimum required energy...
                if min_energy == &0 || &forked_current_energy_spent < min_energy {
                    // Check if everyone is home, and record the new minimum energy, or
                    // go back through this process.
                    if check_win(&forked_amphipod_rooms) {
                        *min_energy = forked_current_energy_spent;
                    } else {
                        run(
                            &forked_hallway,
                            &forked_amphipod_rooms,
                            forked_current_energy_spent,
                            min_energy,
                        );
                    }
                }
            }
        }
    }
}

fn check_win(amphipod_rooms: &HashMap<Space, Vec<Space>>) -> bool {
    for (amphipod_type, room) in amphipod_rooms {
        if room.iter().filter(|x| *x == amphipod_type).count() != ROOM_SIZE {
            return false;
        }
    }
    true
}

// This actually only flushes the hallway, and Amphipods make it home from other rooms
// by going via the hallway (by trying every possible hallway spot). Whilst that's
// super inefficient, it does work, and I can't be bothered to write the code to flush
// the rooms right now.
fn flush_hallway_and_rooms(
    hallway: &mut Vec<Space>,
    amphipod_rooms: &mut HashMap<Space, Vec<Space>>,
) -> u32 {
    let mut energy_spent: u32 = 0;
    let mut move_occurred = true;
    while move_occurred {
        move_occurred = false;
        for (amphipod_type, room) in amphipod_rooms.iter_mut() {
            if !room_contains_foreigners(amphipod_type, room) {
                // Scour the hallway.
                let energy_spent_this_loop = move_into_room(
                    hallway,
                    room,
                    amphipod_type,
                    AMPHIPODS.get(amphipod_type).unwrap(),
                );
                if energy_spent_this_loop > 0 {
                    energy_spent += energy_spent_this_loop;
                    move_occurred = true;
                }
            }
        }
    }
    energy_spent
}

fn room_contains_foreigners(amphipod_type: &Space, room: &[Space]) -> bool {
    for space in room {
        if space != amphipod_type {
            return true;
        }
    }
    false
}

fn free_hallway_spots(hallway: &[Space], room_hallway_position: usize) -> Vec<usize> {
    let mut available_spots: Vec<usize> = Vec::new();
    for space in room_hallway_position + 1..hallway.len() {
        if hallway.get(space).unwrap() == &Space::Empty {
            if ![
                A_ROOM_HALLWAY_POSITION,
                B_ROOM_HALLWAY_POSITION,
                C_ROOM_HALLWAY_POSITION,
                D_ROOM_HALLWAY_POSITION,
            ]
            .contains(&space)
            {
                available_spots.push(space);
            }
        } else {
            break;
        }
    }
    for space in 0..room_hallway_position {
        if hallway.get(room_hallway_position - 1 - space).unwrap() == &Space::Empty {
            if ![
                A_ROOM_HALLWAY_POSITION,
                B_ROOM_HALLWAY_POSITION,
                C_ROOM_HALLWAY_POSITION,
                D_ROOM_HALLWAY_POSITION,
            ]
            .contains(&(room_hallway_position - 1 - space))
            {
                available_spots.push(room_hallway_position - 1 - space);
            }
        } else {
            break;
        }
    }
    available_spots
}

fn move_into_room(
    hallway: &mut Vec<Space>,
    room: &mut Vec<Space>,
    amphipod_type: &Space,
    amphipod_info: &AmphipodInfo,
) -> u32 {
    let fixed_hallway = hallway.clone();
    for (hallway_space, occupant) in hallway.iter_mut().enumerate() {
        if occupant == amphipod_type {
            let mut occupant_can_move_into_room = true;
            let mut pos1 = hallway_space;
            let mut pos2 = amphipod_info.room_hallway_position;
            if amphipod_info.room_hallway_position < hallway_space {
                pos1 = amphipod_info.room_hallway_position;
                pos2 = hallway_space;
            }
            for journey_space in pos1 + 1..pos2 {
                if fixed_hallway.get(journey_space).unwrap() != &Space::Empty {
                    occupant_can_move_into_room = false;
                    break;
                }
            }
            if occupant_can_move_into_room {
                let required_energy: u32 =
                    amphipod_info.move_energy * (pos2 - pos1 + ROOM_SIZE - room.len()) as u32;
                room.push(*amphipod_type);
                *occupant = Space::Empty;
                return required_energy;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let mut hallway = vec![Space::Empty; HALLWAY_LENGTH];
        hallway[0] = Space::A;
        let a_room: Vec<Space> = vec![Space::A, Space::A, Space::A, Space::C];
        let b_room: Vec<Space> = vec![Space::B, Space::B, Space::B, Space::B];
        let c_room: Vec<Space> = vec![Space::C, Space::C, Space::C];
        let d_room: Vec<Space> = vec![Space::D, Space::D, Space::D, Space::D];
        let mut amphipod_rooms: HashMap<Space, Vec<Space>> = HashMap::new();
        amphipod_rooms.insert(Space::A, a_room);
        amphipod_rooms.insert(Space::B, b_room);
        amphipod_rooms.insert(Space::C, c_room);
        amphipod_rooms.insert(Space::D, d_room);
        let current_energy_spent: u32 = 5500;
        let mut min_energy = 0;

        run(
            &hallway,
            &amphipod_rooms,
            current_energy_spent,
            &mut min_energy,
        );
        assert_eq!(min_energy, 6103);
    }

    #[test]
    fn test_run2() {
        let hallway = vec![Space::Empty; HALLWAY_LENGTH];
        let a_room: Vec<Space> = vec![Space::A, Space::C, Space::A, Space::A];
        let b_room: Vec<Space> = vec![Space::B, Space::B, Space::B, Space::B];
        let c_room: Vec<Space> = vec![Space::C, Space::C, Space::C, Space::A];
        let d_room: Vec<Space> = vec![Space::D, Space::D, Space::D, Space::D];
        let mut amphipod_rooms: HashMap<Space, Vec<Space>> = HashMap::new();
        amphipod_rooms.insert(Space::A, a_room);
        amphipod_rooms.insert(Space::B, b_room);
        amphipod_rooms.insert(Space::C, c_room);
        amphipod_rooms.insert(Space::D, d_room);
        let current_energy_spent: u32 = 5500;
        let mut min_energy: u32 = 0;

        run(
            &hallway,
            &amphipod_rooms,
            current_energy_spent,
            &mut min_energy,
        );
        assert_eq!(min_energy, 6322);
    }

    #[test]
    fn test_run3() {
        let hallway = vec![Space::Empty; HALLWAY_LENGTH];
        let a_room: Vec<Space> = vec![Space::A, Space::A, Space::C, Space::B];
        let b_room: Vec<Space> = vec![Space::B, Space::B, Space::D, Space::C];
        let c_room: Vec<Space> = vec![Space::C, Space::C, Space::D, Space::A];
        let d_room: Vec<Space> = vec![Space::D, Space::D, Space::A, Space::B];
        let mut amphipod_rooms: HashMap<Space, Vec<Space>> = HashMap::new();
        amphipod_rooms.insert(Space::A, a_room);
        amphipod_rooms.insert(Space::B, b_room);
        amphipod_rooms.insert(Space::C, c_room);
        amphipod_rooms.insert(Space::D, d_room);
        let mut min_energy: u32 = 0;

        run(&hallway, &amphipod_rooms, 0, &mut min_energy);
        assert_eq!(min_energy, 14350);
    }

    #[test]
    fn test_run4() {
        let hallway = vec![Space::Empty; HALLWAY_LENGTH];
        let a_room: Vec<Space> = vec![Space::C, Space::D, Space::D, Space::B];
        let b_room: Vec<Space> = vec![Space::D, Space::B, Space::C, Space::C];
        let c_room: Vec<Space> = vec![Space::D, Space::A, Space::B, Space::A];
        let d_room: Vec<Space> = vec![Space::A, Space::C, Space::A, Space::B];
        let mut amphipod_rooms: HashMap<Space, Vec<Space>> = HashMap::new();
        amphipod_rooms.insert(Space::A, a_room);
        amphipod_rooms.insert(Space::B, b_room);
        amphipod_rooms.insert(Space::C, c_room);
        amphipod_rooms.insert(Space::D, d_room);
        let mut min_energy: u32 = 0;

        run(&hallway, &amphipod_rooms, 0, &mut min_energy);
        assert_eq!(min_energy, 49742);
    }

    #[test]
    fn test_free_hallway_spots() {
        let mut hallway = vec![Space::Empty; HALLWAY_LENGTH];
        assert_eq!(
            free_hallway_spots(&hallway, A_ROOM_HALLWAY_POSITION),
            [3, 5, 7, 9, 10, 1, 0]
        );

        hallway[7] = Space::B;
        hallway[0] = Space::C;
        assert_eq!(
            free_hallway_spots(&hallway, A_ROOM_HALLWAY_POSITION),
            [3, 5, 1]
        );
    }

    #[test]
    fn test_check_win() {
        let mut winning_map: HashMap<Space, Vec<Space>> = HashMap::new();
        winning_map.insert(Space::A, vec![Space::A; 4]);
        winning_map.insert(Space::B, vec![Space::B; 4]);
        winning_map.insert(Space::C, vec![Space::C; 4]);
        winning_map.insert(Space::D, vec![Space::D; 4]);
        assert_eq!(check_win(&winning_map), true);

        let mut losing_map: HashMap<Space, Vec<Space>> = HashMap::new();
        losing_map.insert(Space::A, vec![Space::A; 4]);
        losing_map.insert(Space::B, vec![Space::B; 4]);
        losing_map.insert(Space::C, vec![Space::C; 3]);
        losing_map.insert(Space::D, vec![Space::D; 4]);
        assert_eq!(check_win(&losing_map), false);
        losing_map.insert(Space::C, vec![Space::C; 4]);
        losing_map.insert(Space::D, vec![Space::A; 1]);
        assert_eq!(check_win(&losing_map), false);
    }

    #[test]
    fn test_move_into_room() {
        let mut hallway: Vec<Space> = vec![Space::Empty; HALLWAY_LENGTH];
        hallway[1] = Space::D;
        let mut d_room: Vec<Space> = vec![];

        assert_eq!(
            move_into_room(
                &mut hallway,
                &mut d_room,
                &Space::D,
                &AmphipodInfo {
                    move_energy: D_ENERGY,
                    room_hallway_position: D_ROOM_HALLWAY_POSITION
                }
            ),
            11000_u32
        );
        assert_eq!(d_room, [Space::D]);
        assert_eq!(
            hallway,
            [
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty
            ]
        );

        hallway[1] = Space::D;
        hallway[3] = Space::C;
        assert_eq!(
            move_into_room(
                &mut hallway,
                &mut d_room,
                &Space::D,
                &AmphipodInfo {
                    move_energy: D_ENERGY,
                    room_hallway_position: D_ROOM_HALLWAY_POSITION
                }
            ),
            0_u32
        );
        assert_eq!(d_room, [Space::D]);
        assert_eq!(
            hallway,
            [
                Space::Empty,
                Space::D,
                Space::Empty,
                Space::C,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty
            ]
        );

        let mut c_room: Vec<Space> = vec![];
        c_room.push(Space::C);
        c_room.push(Space::C);
        assert_eq!(
            move_into_room(
                &mut hallway,
                &mut c_room,
                &Space::C,
                &AmphipodInfo {
                    move_energy: C_ENERGY,
                    room_hallway_position: C_ROOM_HALLWAY_POSITION
                }
            ),
            500_u32
        );
        assert_eq!(c_room, [Space::C, Space::C, Space::C]);
        assert_eq!(
            hallway,
            [
                Space::Empty,
                Space::D,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty
            ]
        );

        hallway[10] = Space::A;
        let mut a_room: Vec<Space> = vec![Space::A; 3];
        assert_eq!(
            move_into_room(
                &mut hallway,
                &mut a_room,
                &Space::A,
                &AmphipodInfo {
                    move_energy: A_ENERGY,
                    room_hallway_position: A_ROOM_HALLWAY_POSITION
                }
            ),
            9_u32
        );
        assert_eq!(a_room, [Space::A, Space::A, Space::A, Space::A]);
        assert_eq!(
            hallway,
            [
                Space::Empty,
                Space::D,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty
            ]
        );
    }
}
