mod input;

use std::collections::HashSet;

use crate::input::RAW_INPUT;

#[derive(Debug)]
enum Error {
    WrongInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

struct HeightMap(Vec<Vec<u32>>);

impl HeightMap {
    fn get_local_minimums(&self) -> Vec<(usize, usize)> {
        let mut local_minimums = vec![];

        let (row_count, col_count) = (self.0.len(), self.0[0].len());

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let mut adjacents = vec![];
                if x > 0 {
                    adjacents.push(self.0[y][x - 1]);
                }
                if y > 0 {
                    adjacents.push(self.0[y - 1][x]);
                }
                if x < col_count - 1 {
                    adjacents.push(self.0[y][x + 1]);
                }
                if y < row_count - 1 {
                    adjacents.push(self.0[y + 1][x]);
                }
                if *adjacents.iter().min().unwrap() > *cell {
                    local_minimums.push((x, y));
                }
            }
        }

        local_minimums
    }

    fn get_basin_size(&self, point: &(usize, usize)) -> usize {
        let (x, y) = *point;
        let mut points: HashSet<(usize, usize)> = HashSet::new();
        if self.get_cell(x, y) == 9 {
            return 0;
        };

        self.search_basin(&mut points, point);

        points.len()
    }

    fn search_basin(&self, points: &mut HashSet<(usize, usize)>, point: &(usize, usize)) {
        let (x, y) = *point;
        if points.contains(point) || self.get_cell(x, y) == 9 {
            return;
        };

        points.insert((x, y));

        let (row_count, col_count) = (self.0.len(), self.0[0].len());
        if x > 0 {
            self.search_basin(points, &(x - 1, y));
        }
        if y > 0 {
            self.search_basin(points, &(x, y - 1));
        }
        if x < col_count - 1 {
            self.search_basin(points, &(x + 1, y));
        }
        if y < row_count - 1 {
            self.search_basin(points, &(x, y + 1));
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> u32 {
        self.0[y][x]
    }
}

fn get_heights() -> Result<HeightMap, Error> {
    let heights: Result<Vec<Vec<u32>>, Error> = RAW_INPUT
        .split('\n')
        .map(|line| {
            let row: Result<Vec<u32>, Error> = line
                .chars()
                .map(|c| c.to_digit(10).ok_or(Error::WrongInput))
                .collect();
            row
        })
        .collect();

    heights.map(|x| HeightMap(x))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_heights()?;

    let minimums = input.get_local_minimums();
    let risk_sum: u32 = minimums
        .iter()
        .map(|(x, y)| input.get_cell(*x, *y) + 1)
        .sum();

    println!("{}", risk_sum);

    let mut basins: Vec<usize> = minimums
        .iter()
        .map(|point| input.get_basin_size(point))
        .collect();
    basins.sort();
    basins.reverse();

    println!("{:?}", basins[0] * basins[1] * basins[2]);

    Ok(())
}
