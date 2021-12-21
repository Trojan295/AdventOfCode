use std::collections::{BinaryHeap, HashMap};

struct Dice {
    iter: Box<dyn Iterator<Item = usize>>,
    rolls: usize,
}

impl Iterator for Dice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;
        self.iter.next()
    }
}

struct Player {
    state: Vec<i32>,
    score: i32,
}

impl Player {
    fn new(start: i32) -> Self {
        Self {
            state: (start..11).chain(1..start).collect(),
            score: 0,
        }
    }

    fn next_turn<T>(&mut self, dice: &mut T) -> bool
    where
        T: Iterator<Item = usize>,
    {
        let roll: usize = dice.take(3).sum();
        let shift = roll % 10;

        self.state.rotate_left(shift);
        let pos = self.state[0];
        self.score += pos;

        self.score >= 1000
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GameState {
    p1_score: usize,
    p2_score: usize,

    p1_state: usize,
    p2_state: usize,

    turn: Turn,
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_min = vec![self.p1_score, self.p2_score]
            .into_iter()
            .min()
            .unwrap();
        let other_min = vec![other.p1_score, other.p2_score]
            .into_iter()
            .min()
            .unwrap();
        self_min.cmp(&other_min)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_min = vec![self.p1_score, self.p2_score]
            .into_iter()
            .min()
            .unwrap();
        let other_min = vec![other.p1_score, other.p2_score]
            .into_iter()
            .min()
            .unwrap();
        match self_min.partial_cmp(&other_min) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        let self_max = vec![self.p1_score, self.p2_score]
            .into_iter()
            .max()
            .unwrap();
        let other_max = vec![other.p1_score, other.p2_score]
            .into_iter()
            .max()
            .unwrap();
        match self_max.partial_cmp(&other_max) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        match self.p1_state.partial_cmp(&other.p1_state) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.p2_state.partial_cmp(&other.p2_state) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.turn.partial_cmp(&other.turn)
    }
}

#[derive(Debug, Clone)]
struct GameResults {
    p1_wins: usize,
    p2_wins: usize,
}

fn all_games() -> Vec<GameState> {
    let mut games = vec![];

    for turn in vec![Turn::Player1, Turn::Player2].iter() {
        for p1_state in 1..11 {
            for p2_state in 1..11 {
                for p1_score in 0..32 {
                    for p2_score in 0..31 {
                        let game = GameState {
                            p1_score,
                            p2_score,
                            p1_state,
                            p2_state,
                            turn: turn.clone(),
                        };
                        games.push(game);
                    }
                }
            }
        }
    }

    games
}

fn main() {
    let p1_start = 5;
    let p2_start = 10;

    let mut dice = Dice {
        iter: Box::new((1..101).cycle()),
        rolls: 0,
    };
    let dice = dice.by_ref();

    let mut p1 = Player::new(p1_start);
    let mut p2 = Player::new(p2_start);

    loop {
        if p1.next_turn(dice) {
            println!("P1 won - {}", p2.score * dice.rolls as i32);
            break;
        }

        if p2.next_turn(dice) {
            println!("P2 won - {}", p1.score * dice.rolls as i32);
            break;
        }
    }

    let mut winning_games: HashMap<GameState, GameResults> = HashMap::new();

    let mut games = BinaryHeap::new();
    for game in all_games() {
        games.push(game);
    }

    let rolls: Vec<usize> = (1..4)
        .map(|x| {
            (1..4)
                .map(move |y| (1..4).map(move |z| x + y + z))
                .flatten()
        })
        .flatten()
        .collect();

    while let Some(game) = games.pop() {
        if game.p1_score >= 21 {
            winning_games.insert(
                game,
                GameResults {
                    p1_wins: 1,
                    p2_wins: 0,
                },
            );
            continue;
        }

        if game.p2_score >= 21 {
            winning_games.insert(
                game,
                GameResults {
                    p1_wins: 0,
                    p2_wins: 1,
                },
            );
            continue;
        }

        match game.turn {
            Turn::Player1 => {
                let mut result = GameResults {
                    p1_wins: 0,
                    p2_wins: 0,
                };

                for roll in rolls.iter() {
                    let new_state = (game.p1_state - 1 + roll) % 10 + 1;
                    let mut g = game.clone();
                    g.p1_state = new_state;
                    g.p1_score += new_state;
                    g.turn = Turn::Player2;

                    let res = winning_games.get(&g).unwrap().clone();
                    result.p1_wins += res.p1_wins;
                    result.p2_wins += res.p2_wins;
                }

                winning_games.insert(game.clone(), result);
            }
            Turn::Player2 => {
                let mut result = GameResults {
                    p1_wins: 0,
                    p2_wins: 0,
                };

                for roll in rolls.iter() {
                    let new_state = (game.p2_state - 1 + roll) % 10 + 1;
                    let mut g = game.clone();
                    g.p2_state = new_state;
                    g.p2_score += new_state;
                    g.turn = Turn::Player1;

                    let res = winning_games.get(&g).unwrap().clone();
                    result.p1_wins += res.p1_wins;
                    result.p2_wins += res.p2_wins;
                }

                winning_games.insert(game.clone(), result);
            }
        }
    }

    //for (game, results) in winning_games {
    //    println!("{:?}: {:?}", game, results);
    //}

    let result = winning_games
        .get(&GameState {
            p1_score: 0,
            p2_score: 0,
            p1_state: p1_start as usize,
            p2_state: p2_start as usize,
            turn: Turn::Player1,
        })
        .unwrap();

    println!("{:?}", result);
}
