use std::collections::HashSet;

fn flash(octopuses: &mut Vec<Vec<u32>>, (x, y): (usize, usize)) {
    let adjacent = vec![
        (x as i32 - 1, y as i32 - 1),
        (x as i32, y as i32 - 1),
        (x as i32 + 1, y as i32 - 1),
        (x as i32 - 1, y as i32),
        (x as i32 + 1, y as i32),
        (x as i32 - 1, y as i32 + 1),
        (x as i32, y as i32 + 1),
        (x as i32 + 1, y as i32 + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| !(x < 0 || y < 0 || x > 9 || y > 9));

    for (x, y) in adjacent {
        octopuses[y as usize][x as usize] += 1;
    }
}

fn next_step(octopuses: &mut Vec<Vec<u32>>) -> u32 {
    for row in octopuses.iter_mut() {
        for oct in row.iter_mut() {
            *oct += 1;
        }
    }

    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut flashes = 0;

    loop {
        let mut to_flash = HashSet::new();

        for (y, row) in octopuses.iter().enumerate() {
            for (x, oct) in row.iter().enumerate() {
                if !flashed.contains(&(x, y)) && *oct > 9 {
                    to_flash.insert((x, y));
                }
            }
        }

        if to_flash.len() == 0 {
            break;
        }

        flashes += to_flash.len() as u32;

        for coord in to_flash.into_iter() {
            flash(octopuses, coord);
            flashed.insert(coord);
        }
    }

    for row in octopuses.iter_mut() {
        for oct in row.iter_mut() {
            if *oct > 9 {
                *oct = 0;
            }
        }
    }

    flashes
}

fn main() {
    let mut octopuses: Vec<Vec<u32>> = INPUT
        .split('\n')
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut counter = 0;
    loop {
        counter += 1;
        if next_step(&mut octopuses) == 100 {
            break;
        }
    }

    println!("After {} steps", counter);
}

const INPUT: &'static str = r#"4764745784
4643457176
8322628477
7617152546
6137518165
1556723176
2187861886
2553422625
4817584638
3754285662"#;
