use std::{
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Error {
    InputError(Box<dyn std::error::Error>),
}

#[derive(Clone)]
enum Field {
    Empty,
    East,
    South,
}

impl From<char> for Field {
    fn from(x: char) -> Self {
        match x {
            '.' => Self::Empty,
            '>' => Self::East,
            'v' => Self::South,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_char('.'),
            Self::East => f.write_char('>'),
            Self::South => f.write_char('v'),
        }
    }
}

struct Region {
    fields: Vec<Vec<Field>>,
}

impl Region {
    fn next(&mut self) -> usize {
        let (y_size, x_size) = (self.fields.len(), self.fields[0].len());
        let mut moves_counter = 0;

        let mut east_move = vec![vec![Field::Empty; x_size]; y_size];
        for (y, row) in self.fields.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let next_x = (x + 1) % x_size;

                match (field, &self.fields[y][next_x]) {
                    (Field::East, Field::Empty) => {
                        east_move[y][next_x] = Field::East;
                        moves_counter += 1;
                    }
                    (Field::East, _) => {
                        east_move[y][x] = Field::East;
                    }
                    (Field::South, _) => {
                        east_move[y][x] = Field::South;
                    }
                    _ => {}
                }
            }
        }

        let mut south_move = vec![vec![Field::Empty; x_size]; y_size];
        for (y, row) in east_move.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let next_y = (y + 1) % y_size;

                match (field, &east_move[next_y][x]) {
                    (Field::South, Field::Empty) => {
                        south_move[next_y][x] = Field::South;
                        moves_counter += 1;
                    }
                    (Field::South, _) => {
                        south_move[y][x] = Field::South;
                    }
                    (Field::East, _) => south_move[y][x] = Field::East,
                    _ => {}
                }
            }
        }

        self.fields = south_move;

        moves_counter
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.fields.iter() {
            for field in row {
                field.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn load_input(path: &str) -> Result<Region, Error> {
    let fp = File::open(path).map_err(|e| Error::InputError(Box::new(e)))?;
    let reader = BufReader::new(fp);

    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines.map_err(|e| Error::InputError(Box::new(e)))?;

    let mut fields = vec![];
    for line in lines {
        let row: Vec<Field> = line.chars().map(|c| c.into()).collect();
        fields.push(row);
    }

    Ok(Region { fields })
}

fn main() {
    let mut region = load_input("day25/input.txt").unwrap();

    let mut counter = 1;
    while region.next() > 0 {
        counter += 1
    }
    println!("{}", region);
    println!("{}", counter);
}
