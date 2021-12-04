use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct ParseError {
    line: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse line: {}", self.line)
    }
}

impl Error for ParseError {}

struct Position {
    depth: i64,
    x: i64,
    aim: i64,
}

impl Position {
    fn mv(&mut self, action: &Action) {
        match action {
            Action::Forward(value) => {
                self.x += value;
                self.depth += self.aim * value;
            }
            Action::Down(value) => {
                self.aim += value;
            }
            Action::Up(value) => {
                self.aim -= value;
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum Action {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn get_result(input_file: &str) -> Result<Position, Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut pos = Position {
        depth: 0,
        x: 0,
        aim: 0,
    };

    for line in lines {
        if let Ok(line) = line {
            let mv = parse_action(&line)?;
            pos.mv(&mv);
        }
    }

    Ok(pos)
}

fn parse_action(line: &str) -> Result<Action, Box<dyn Error>> {
    let mut cols = line.split(' ');
    let command = cols.next().ok_or(ParseError {
        line: line.to_owned(),
    })?;
    let value = cols
        .next()
        .ok_or(ParseError {
            line: line.to_owned(),
        })
        .map(|val| val.parse::<i64>())??;

    match command {
        "forward" => Ok(Action::Forward(value)),
        "up" => Ok(Action::Up(value)),
        "down" => Ok(Action::Down(value)),
        &_ => Err(Box::new(ParseError {
            line: line.to_owned(),
        })),
    }
}

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());

    match get_result(&input_file) {
        Err(err) => eprintln!("Failed: {}", err),
        Ok(coords) => println!("Result : {}", coords.x * coords.depth),
    }
}

#[test]
fn test_move() {
    let mut pos = Position {
        depth: 0,
        x: 0,
        aim: 0,
    };
    pos.mv(&Action::Forward(1));
    pos.mv(&Action::Down(4));
    pos.mv(&Action::Up(3));
    pos.mv(&Action::Forward(1));

    assert_eq!(pos.depth, 1);
    assert_eq!(pos.x, 2);
}

#[test]
fn test_parse_move() {
    assert_eq!(parse_action("forward 2").unwrap(), Action::Forward(2));
    assert_eq!(parse_action("up 3").unwrap(), Action::Up(3));
    assert_eq!(parse_action("down 4").unwrap(), Action::Down(4));
    assert!(parse_action("err 2").is_err());
}
