#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{self, BufReader, Read},
};

#[derive(Debug)]
enum Error {
    IO(io::Error),
    WrongInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => f.write_str(&format!("IOError: {}", err)),
            Error::WrongInput => f.write_str(&format!("Wrong input")),
        }
    }
}

impl std::error::Error for Error {}

fn load_input(path: &str) -> Result<Vec<u64>, Error> {
    let f = File::open(path).map_err(|err| Error::IO(err))?;
    let mut reader = BufReader::new(f);

    let mut data = String::new();

    reader
        .read_to_string(&mut data)
        .map_err(|err| Error::IO(err))?;

    let input: Result<Vec<_>, _> = data
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().map_err(|_| Error::WrongInput))
        .collect();
    input
}

fn new_state(input: &Vec<u64>) -> Vec<u64> {
    let mut state = vec![0; 9];

    for x in input {
        state.get_mut(*x as usize).map(|c| *c += 1);
    }

    state
}

fn next_day(state: &mut Vec<u64>) {
    let births = state[0];
    state.rotate_left(1);

    state[6] += births;
    state[8] = births;
}

fn count(state: &Vec<u64>) -> u64 {
    state.iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    let input = load_input(&input_file)?;
    let mut state = new_state(&input);

    let days = 256;

    for _ in 0..days {
        next_day(&mut state);
    }

    println!("{:?}", count(&state));

    Ok(())
}
