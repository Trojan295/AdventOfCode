mod input;

use std::char;

use crate::input::RAW_INPUT;

#[derive(Debug)]
enum Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

struct Input(Vec<Vec<char>>);

fn get_input() -> Input {
    let data: Vec<Vec<char>> = RAW_INPUT
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    Input(data)
}

fn get_closing_bracket(c: &char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn get_score(s: &str) -> u64 {
    let mut score = 0;

    for c in s.chars() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }

    score
}

fn get_missing(line: &Vec<char>) -> Option<String> {
    let mut stack = vec![];

    let mut corrupted = false;

    for c in line.iter() {
        match *c {
            '(' | '[' | '{' | '<' => {
                stack.push(*c);
            }
            ')' | ']' | '}' | '>' => {
                let bracket = stack.pop().unwrap();
                if *c != get_closing_bracket(&bracket).unwrap() {
                    corrupted = true;
                    break;
                }
            }
            _ => {
                return None;
            }
        }
    }

    if corrupted {
        None
    } else {
        stack.reverse();

        let missing = stack
            .iter()
            .map(|c| get_closing_bracket(c).unwrap())
            .fold("".to_owned(), |a, b| format!("{}{}", a, b));
        Some(missing)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input();

    let mut scores = vec![];

    for line in input.0.iter() {
        let missing = get_missing(line);

        if let Some(missing) = missing {
            scores.push(get_score(&missing));
        }
    }

    scores.sort();
    println!("{:?}", scores);
    println!("{}", scores.len());
    let middle = (scores.len() - 1) / 2;
    println!("{}", middle);
    println!("{}", scores[middle]);

    Ok(())
}

#[test]
fn test_score() {
    assert!(get_score("])}>") == 294);
}

#[test]
fn test_missing() {
    assert_eq!(
        get_missing(&"{<[[]]>}<{[{[{[]{()[[[]".chars().collect()).unwrap(),
        "]]}}]}]}>".to_owned()
    );
}
