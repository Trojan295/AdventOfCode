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

fn load_input(path: &str) -> Result<Vec<u32>, Error> {
    let f = File::open(path).map_err(|err| Error::IO(err))?;
    let mut reader = BufReader::new(f);

    let mut data = String::new();

    reader
        .read_to_string(&mut data)
        .map_err(|err| Error::IO(err))?;

    let input: Result<Vec<_>, _> = data
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().map_err(|_| Error::WrongInput))
        .collect();
    input
}

fn next_day(state: &mut Vec<u32>) {
    let mut new_fishes = 0;

    for fish in state.iter_mut() {
        if *fish == 0 {
            *fish = 6;
            new_fishes += 1;
        } else {
            *fish -= 1;
        }
    }

    for _ in 0..new_fishes {
        state.push(8);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    //let mut state = load_input(&input_file)?;
    let mut state = vec![3, 4, 3, 1, 2];

    let days = 256;

    for _ in 0..days {
        next_day(&mut state);
    }

    println!("{:?}", state.len());

    Ok(())
}

mod tests {

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_next_day(b: &mut Bencher) {
        b.iter(|| {
            let mut state = vec![3, 4, 3, 1, 2];
            for _ in 0..100 {
                next_day(&mut state)
            }
        })
    }
}
