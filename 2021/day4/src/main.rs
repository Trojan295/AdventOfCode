use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

#[derive(Debug)]
enum BingoError {
    MissingInput,
    WrongInput(ParseIntError),
}

impl std::fmt::Display for BingoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BingoError::MissingInput => f.write_str(&format!("missing input")),
            BingoError::WrongInput(err) => f.write_str(&format!("wrong input: {}", err)),
        }
    }
}

impl Error for BingoError {}

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<u32>,
    marked: Vec<bool>,
}

impl Board {
    fn new(numbers: &Vec<u32>) -> Board {
        Board {
            numbers: numbers.clone(),
            marked: vec![false; 25],
        }
    }

    fn is_winning(&self) -> bool {
        for i in 0..5 {
            if self.marked[5 * i + 0]
                && self.marked[5 * i + 1]
                && self.marked[5 * i + 2]
                && self.marked[5 * i + 3]
                && self.marked[5 * i + 4]
            {
                return true;
            }

            if self.marked[i]
                && self.marked[i + 5]
                && self.marked[i + 10]
                && self.marked[i + 15]
                && self.marked[i + 20]
            {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        let iter = self.marked.iter().zip(self.numbers.iter());

        let value: u32 = iter
            .filter_map(|(marked, number)| if !*marked { Some(*number) } else { None })
            .sum();

        value
    }
}

struct Game {
    round: u32,

    remaining_numbers: Box<dyn Iterator<Item = u32>>,
    boards: Vec<Board>,

    board_winning_rounds: Vec<u32>,
    board_winning_scores: Vec<u32>,
}

impl Game {
    fn new(numbers: &Vec<u32>, boards: &Vec<Board>) -> Game {
        let board_count = boards.len();

        Game {
            round: 0,
            remaining_numbers: Box::new(numbers.clone().into_iter()),
            boards: boards.clone(),

            board_winning_rounds: vec![0; board_count],
            board_winning_scores: vec![0; board_count],
        }
    }

    fn next(&mut self) -> Option<u32> {
        let selected_num = self.remaining_numbers.next()?;
        self.round += 1;

        for (pos, board) in self.boards.iter_mut().enumerate() {
            for (pos, number) in board.numbers.iter().enumerate() {
                if *number == selected_num {
                    board.marked[pos] = true;
                }
            }

            if self.board_winning_rounds[pos] == 0 && board.is_winning() {
                self.board_winning_rounds[pos] = self.round;
                self.board_winning_scores[pos] = board.score() * selected_num;
            }
        }

        Some(selected_num)
    }

    fn _get_winning(&self) -> Option<&Board> {
        for board in self.boards.iter() {
            if board.is_winning() {
                return Some(board);
            }
        }

        None
    }
}

fn create_game(input: &Vec<String>) -> Result<Game, BingoError> {
    let mut input_iter = input.iter();
    let numbers_str = input_iter.next().ok_or(BingoError::MissingInput)?;
    let numbers: Result<Vec<_>, _> = numbers_str
        .split(',')
        .map(|chr| {
            chr.parse::<u32>()
                .map_err(|err| BingoError::WrongInput(err))
        })
        .collect();

    let numbers = numbers?;

    let mut boards = vec![];

    for chunk in input_iter.collect::<Vec<&String>>().chunks_exact(6) {
        let mut board_numbers = vec![];

        for line in chunk {
            let number_strings: Vec<&str> = line.split_whitespace().collect();
            let new_numbers: Result<Vec<u32>, _> = number_strings
                .into_iter()
                .map(|x| x.parse::<u32>().map_err(|err| BingoError::WrongInput(err)))
                .collect();
            let new_numbers = new_numbers?;

            for n in new_numbers {
                board_numbers.push(n);
            }
        }

        let board = Board::new(&board_numbers);
        boards.push(board);
    }

    Ok(Game::new(&numbers, &boards))
}

fn load_report(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    let input = load_report(&input_file)?;

    let mut game = create_game(&input)?;

    while let Some(_) = game.next() {}

    let result = game
        .board_winning_rounds
        .iter()
        .zip(game.board_winning_scores.iter())
        .max_by_key(|(round, _)| *round);

    println!("{:?}", game.boards);

    if let Some(result) = result {
        println!("Round {}; Score: {}", result.0, result.1);
    }

    Ok(())
}
