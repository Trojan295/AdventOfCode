use std::{
    char,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
enum Error {
    IO(io::Error),
    WrongInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

fn load_input(path: &str) -> Result<Vec<Entry>, Error> {
    let f = File::open(path).map_err(|err| Error::IO(err))?;
    let reader = BufReader::new(f);

    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines.map_err(|err| Error::IO(err))?;

    let result = lines
        .into_iter()
        .map(|line| {
            let mut cols = line.split(" | ").take(2);

            let input_col: Result<Vec<HashSet<char>>, _> =
                cols.next().ok_or(Error::WrongInput).map(|input| {
                    input
                        .split_whitespace()
                        .map(|x| x.chars().collect())
                        .collect()
                });
            let output_col: Result<Vec<HashSet<char>>, _> =
                cols.next().ok_or(Error::WrongInput).map(|output| {
                    output
                        .split_whitespace()
                        .map(|x| x.chars().collect())
                        .collect()
                });

            match (input_col, output_col) {
                (Ok(input), Ok(output)) => Ok(Entry {
                    input: input,
                    output: output,
                }),
                (Err(err), _) => Err(err),
                (_, Err(err)) => Err(err),
            }
        })
        .collect();
    result
}

struct Entry {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl Entry {
    fn get_digits(&self) -> Vec<HashSet<char>> {
        let mut digits = vec![HashSet::new(); 10];

        for num in self.input.iter() {
            match num.len() {
                2 => digits[1] = num.clone(),
                4 => digits[4] = num.clone(),
                3 => digits[7] = num.clone(),
                7 => digits[8] = num.clone(),
                _ => {}
            }
        }

        // get 9
        for x in self.input.iter() {
            if x.len() == 6 && x.union(&digits[4]).count() == 6 {
                digits[9] = x.clone();
                break;
            }
        }

        // get 6
        for x in self.input.iter() {
            if x.len() == 6 && x.union(&digits[1]).count() == 7 {
                digits[6] = x.clone();
                break;
            }
        }

        // get 0
        for x in self.input.iter() {
            if x.len() == 6
                && x.difference(&digits[9]).count() > 0
                && x.difference(&digits[6]).count() > 0
            {
                digits[0] = x.clone();
                break;
            }
        }

        // get 3
        for x in self.input.iter() {
            if x.len() == 5 && digits[7].intersection(x).count() == 3 {
                digits[3] = x.clone();
                break;
            }
        }

        // get 2
        for x in self.input.iter() {
            if x.len() == 5 && digits[9].union(x).count() == 7 {
                digits[2] = x.clone();
                break;
            }
        }

        // get 5
        for x in self.input.iter() {
            if x.len() == 5 && digits[6].union(x).count() == 6 {
                digits[5] = x.clone();
                break;
            }
        }

        digits
    }
}

fn count_digits(entries: &Vec<Entry>) -> Vec<u32> {
    let mut counter = vec![0; 10];

    for entry in entries.iter() {
        let digits = entry.get_digits();

        for num in entry.output.iter() {
            let digit = digits.iter().position(|x| x == num).unwrap();
            counter[digit] += 1;
        }
    }

    counter
}

fn get_number(entry: &Entry) -> Option<u32> {
    let digits = entry.get_digits();

    let thousand = digits.iter().position(|x| *x == entry.output[0])?;
    let hundred = digits.iter().position(|x| *x == entry.output[1])?;
    let tens = digits.iter().position(|x| *x == entry.output[2])?;
    let units = digits.iter().position(|x| *x == entry.output[3])?;

    Some((1000 * thousand + 100 * hundred + 10 * tens + units) as u32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    let input = load_input(&input_file)?;

    let digits = count_digits(&input);
    let sum: u32 = digits[1] + digits[4] + digits[7] + digits[8];

    let numbers: Option<Vec<u32>> = input.iter().map(get_number).collect();
    let numbers = numbers.ok_or(Error::WrongInput)?;
    let number_sum: u32 = numbers.iter().sum();

    println!("{}", sum);
    println!("{}", number_sum);

    Ok(())
}
