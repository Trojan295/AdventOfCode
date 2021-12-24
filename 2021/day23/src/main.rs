use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

use self::Amphipod::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Field {
    Empty,
    Taken(Amphipod),
}

use self::Field::*;

impl Field {
    fn amphiopod(&self) -> Option<Amphipod> {
        match &self {
            Taken(a) => Some(a.clone()),
            Empty => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Burrow {
    rooms: [(Field, Field, Field, Field); 4],
    hallway: [Field; 7],
}

impl Burrow {
    fn new(rooms: [(Field, Field, Field, Field); 4]) -> Self {
        Self {
            hallway: [Field::Empty; 7],
            rooms: rooms,
        }
    }

    fn completed(&self) -> bool {
        match self.rooms {
            [(Taken(Amber), Taken(Amber), Taken(Amber), Taken(Amber)), (Taken(Bronze), Taken(Bronze), Taken(Bronze), Taken(Bronze)), (Taken(Copper), Taken(Copper), Taken(Copper), Taken(Copper)), (Taken(Desert), Taken(Desert), Taken(Desert), Taken(Desert))] => {
                true
            }
            _ => false,
        }
    }

    fn allowed_moves(&self) -> Vec<(Burrow, usize)> {
        let mut allowed_moves = vec![];

        // from rooms
        for room_idx in 0..4 {
            let mut burrow = self.clone();
            let amphipod = match &mut burrow.rooms[room_idx] {
                (Empty, Empty, Empty, Empty) => None,
                (Empty, Empty, Empty, field) => Some((field, 4)),
                (Empty, Empty, field, _) => Some((field, 3)),
                (Empty, field, _, _) => Some((field, 2)),
                (field, _, _, _) => Some((field, 1)),
            };
            if let Some((field, moves)) = amphipod {
                let amphipod = field.amphiopod().unwrap();
                let new_field = field.clone();
                *field = Empty;

                // left
                for i in 0..2 + room_idx {
                    let pos = 1 + room_idx - i;
                    if let Taken(_) = self.hallway[pos] {
                        break;
                    }

                    let mut burrow = burrow.clone();
                    burrow.hallway[pos] = new_field;

                    let mut moves = moves + 1 + (i * 2);
                    if pos == 0 {
                        moves -= 1;
                    }

                    allowed_moves.push((burrow, moves * amphipod.energy()));
                }
                // right
                for i in 0..5 - room_idx {
                    let pos = 2 + i + room_idx;
                    if let Taken(_) = self.hallway[pos] {
                        break;
                    }

                    let mut a_burrow = burrow.clone();
                    a_burrow.hallway[pos] = new_field;

                    let mut moves = moves + 1 + (i * 2);
                    if pos == 6 {
                        moves -= 1;
                    }

                    allowed_moves.push((a_burrow, moves * amphipod.energy()));
                }
            }
        }

        for hallway_idx in 0..7 {
            let mut burrow = self.clone();
            let from_field = &mut burrow.hallway[hallway_idx];

            let target_room: Option<usize> = match from_field.clone() {
                Empty => None,
                Taken(Amber) => Some(0),
                Taken(Bronze) => Some(1),
                Taken(Copper) => Some(2),
                Taken(Desert) => Some(3),
            };
            if target_room.is_none() {
                continue;
            }
            let amphipod = from_field.amphiopod().unwrap();

            let mut moves = 0;

            let target_room = target_room.unwrap();
            let (field1, field2, field3, field4) = &mut burrow.rooms[target_room];
            let dest_field = match (&field1, &field2, &field3, &field4) {
                (Taken(_), _, _, _) => continue,
                (Empty, Taken(a), Taken(b), Taken(c)) => {
                    if amphipod != *a || amphipod != *b || amphipod != *c {
                        continue;
                    }

                    moves += 1;
                    field1
                }
                (Empty, Empty, Taken(a), Taken(b)) => {
                    if amphipod != *a || amphipod != *b {
                        continue;
                    }

                    moves += 2;
                    field2
                }
                (Empty, Empty, Empty, Taken(a)) => {
                    if amphipod != *a {
                        continue;
                    }

                    moves += 3;
                    field3
                }

                (Empty, Empty, Empty, Empty) => {
                    moves += 4;
                    field4
                }
                _ => continue,
            };

            let amphipod = from_field.amphiopod().unwrap();
            *from_field = Empty;

            let mut hallway_idx = hallway_idx;

            match (burrow.hallway[1], burrow.hallway[5], hallway_idx) {
                (Taken(_), _, 0) => continue,
                (_, Taken(_), 6) => continue,
                (Empty, _, 0) => {
                    moves += 1;
                    hallway_idx = 1;
                }
                (_, Empty, 6) => {
                    moves += 1;
                    hallway_idx = 5;
                }
                _ => {}
            }

            let move_dir = if (target_room as f32 + 1.5) - hallway_idx as f32 >= 0.0 {
                1
            } else {
                -1
            };

            let checks: Vec<usize> = match move_dir {
                1 => ((hallway_idx + 1)..(target_room + 2)).collect(),
                -1 => ((target_room + 2)..(hallway_idx)).collect(),
                _ => unreachable!(),
            };

            let mut skip = false;
            for check in checks.iter() {
                if let Taken(_) = burrow.hallway[*check] {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            };

            moves += 1 + checks.len() * 2;
            *dest_field = Taken(amphipod);
            allowed_moves.push((burrow, moves * amphipod.energy()));
        }

        allowed_moves
    }
}

#[derive(PartialEq, Eq)]
struct BurrowState {
    burrow: Burrow,
    energy: usize,
}

impl Ord for BurrowState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl PartialOrd for BurrowState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.energy.partial_cmp(&self.energy)
    }
}

fn main() {
    let burrow = Burrow::new([
        (Taken(Copper), Taken(Desert), Taken(Desert), Taken(Desert)),
        (Taken(Copper), Taken(Copper), Taken(Bronze), Taken(Amber)),
        (Taken(Bronze), Taken(Bronze), Taken(Amber), Taken(Bronze)),
        (Taken(Desert), Taken(Amber), Taken(Copper), Taken(Amber)),
    ]);

    let mut visited: HashSet<Burrow> = HashSet::new();

    let mut burrows = BinaryHeap::new();
    burrows.push(BurrowState {
        burrow: burrow,
        energy: 0,
    });

    while let Some(burrow_state) = burrows.pop() {
        if visited.contains(&burrow_state.burrow) {
            continue;
        }
        visited.insert(burrow_state.burrow);

        if burrow_state.burrow.completed() {
            println!("Complated: {}", burrow_state.energy);
            break;
        }

        let moves = burrow_state.burrow.allowed_moves();

        for (burrow, new_energy) in moves {
            burrows.push(BurrowState {
                burrow: burrow,
                energy: burrow_state.energy + new_energy,
            });
        }
    }

    println!("{}", burrows.len())
}
