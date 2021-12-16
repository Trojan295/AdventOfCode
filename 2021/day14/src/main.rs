use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type PolimerTemplate = String;
type InsertionPairs = HashMap<(char, char), char>;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    MissingInput,
    Wrapped(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => f.write_str(&format!("IO error: {}", err)),
            Error::MissingInput => f.write_str(&format!("Missing input")),
            Error::Wrapped(err) => f.write_str(&format!("Wrapped error: {}", err)),
        }
    }
}

impl std::error::Error for Error {}

fn parse_insetion_pair(line: &str) -> ((char, char), char) {
    let mut cols = line.split(" -> ");
    let pair: Vec<char> = cols.next().unwrap().chars().collect();
    let insert = cols.next().unwrap().chars().collect::<Vec<char>>()[0];

    ((pair[0], pair[1]), insert)
}

fn read_input(filepath: &str) -> Result<(PolimerTemplate, InsertionPairs), Error> {
    let f = File::open(filepath).map_err(|err| Error::IO(err))?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let template: PolimerTemplate = lines
        .next()
        .ok_or(Error::MissingInput)?
        .map_err(|err| Error::Wrapped(Box::new(err)))?;

    let lines: Result<Vec<String>, Error> = lines
        .map(|line| line.map_err(|err| Error::Wrapped(Box::new(err))))
        .collect();
    let lines = lines?;
    let x = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| parse_insetion_pair(&line))
        .collect();
    Ok((template, x))
}

fn polymerize(
    pair_count: &HashMap<(char, char), u64>,
    pairs: &InsertionPairs,
) -> HashMap<(char, char), u64> {
    let mut counter: HashMap<(char, char), u64> = HashMap::new();

    for (pair, &count) in pair_count {
        let &(first, second) = pair;

        let inserted = pairs.get(pair).unwrap();

        let first_pair = (first, *inserted);
        let first_value = counter.entry(first_pair).or_insert(0);
        *first_value += count;

        let second_pair = (*inserted, second);
        let second_value = counter.entry(second_pair).or_insert(0);
        *second_value += count;
    }

    counter
}

fn count(pair_count: &HashMap<(char, char), u64>, first: char, last: char) -> HashMap<char, u64> {
    let mut frequency: HashMap<char, u64> = HashMap::new();

    for (&pair, &count) in pair_count {
        let (first, second) = pair;
        *frequency.entry(first).or_insert(0) += count;
        *frequency.entry(second).or_insert(0) += count;
    }

    *frequency.entry(first).or_insert(1) += 1;
    *frequency.entry(last).or_insert(1) += 1;

    for (_, count) in frequency.iter_mut() {
        *count /= 2;
    }

    frequency
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or("day14/input.txt".to_owned());
    let (template, pairs) = read_input(&input_file)?;

    let mut polimer: HashMap<(char, char), u64> = HashMap::new();

    for x in template.chars().collect::<Vec<char>>().windows(2) {
        let pair = (x[0], x[1]);
        *polimer.entry(pair).or_insert(0) += 1;
    }

    for i in 0..40 {
        polimer = polymerize(&polimer, &pairs);
        println!("{}", i);
    }

    let frequency = count(&polimer, 'F', 'B');

    let (_, most) = frequency.iter().max_by_key(|(_, x)| **x).unwrap();
    let (_, least) = frequency.iter().min_by_key(|(_, x)| **x).unwrap();

    println!("Result: {}", most - least);

    Ok(())
}

#[test]
fn test_polimerize() {
    let pairs: HashMap<(char, char), char> = HashMap::from_iter(vec![
        (('a', 'b'), 'c'),
        (('a', 'c'), 'b'),
        (('c', 'b'), 'a'),
    ]);
    let mut counter: HashMap<(char, char), u64> = HashMap::from_iter(vec![(('a', 'b'), 1)]);

    for _ in 0..2 {
        counter = polymerize(&counter, &pairs);
    }
    println!("{:?}", counter);

    assert_eq!(counter.get(&('a', 'b')).unwrap_or(&0), &2);
    assert_eq!(counter.get(&('c', 'a')).unwrap_or(&0), &1);
    assert_eq!(counter.get(&('b', 'c')).unwrap_or(&0), &1);
}

#[test]
fn test_count() {
    let counter: HashMap<(char, char), u64> =
        HashMap::from_iter(vec![(('a', 'b'), 2), (('b', 'c'), 1), (('c', 'a'), 1)]);

    let freq = count(&counter, 'a', 'b');
    assert_eq!(freq.get(&'a').unwrap_or(&0), &2);
    assert_eq!(freq.get(&'b').unwrap_or(&0), &2);
    assert_eq!(freq.get(&'c').unwrap_or(&0), &1);
}
