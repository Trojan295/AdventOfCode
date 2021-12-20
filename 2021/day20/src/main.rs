use std::{
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
};

type Algorithm = Vec<bool>;

struct Image {
    fill_pixels: bool,
    pixels: Vec<Vec<bool>>,
}

impl Image {
    fn enhanced(&self, alg: &Algorithm) -> Self {
        let (size_x, size_y) = (self.pixels.len(), self.pixels[0].len());
        let (input_size_x, input_size_y) = (size_x + 4, size_y + 4);
        let (new_size_x, new_size_y) = (size_x + 2, size_y + 2);

        let mut input_image = vec![vec![self.fill_pixels; input_size_y]; input_size_x];
        let mut new_pixels = vec![vec![false; new_size_y]; new_size_x];

        for x in 0..size_x {
            for y in 0..size_y {
                input_image[y + 2][x + 2] = self.pixels[y][x];
            }
        }

        for x in 0..new_size_x {
            for y in 0..new_size_y {
                let index = Image::get_pixel_index(&input_image, x + 1, y + 1);
                new_pixels[y][x] = alg[index];
            }
        }

        let fill_pixel = match self.fill_pixels {
            false => alg[0],
            true => alg[511],
        };

        Self {
            pixels: new_pixels,
            fill_pixels: fill_pixel,
        }
    }

    fn get_pixel_index(pixels: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
        let index: usize = vec![
            pixels[y - 1][x - 1],
            pixels[y - 1][x],
            pixels[y - 1][x + 1],
            pixels[y][x - 1],
            pixels[y][x],
            pixels[y][x + 1],
            pixels[y + 1][x - 1],
            pixels[y + 1][x],
            pixels[y + 1][x + 1],
        ]
        .into_iter()
        .rev()
        .enumerate()
        .map(|(p, b)| match b {
            true => 1 << p,
            false => 0,
        })
        .sum();
        index
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.pixels.iter() {
            let line = row
                .iter()
                .map(|c| match c {
                    true => "#",
                    false => ".",
                })
                .collect::<Vec<&str>>()
                .join("");
            f.write_str(&line)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn read_input(path: &str) -> (Algorithm, Image) {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let lines: std::io::Result<Vec<String>> = reader.lines().collect();
    let lines = lines.unwrap();

    let mut lines = lines.into_iter();
    let alg_line = lines.next().unwrap();

    let alg: Algorithm = alg_line.chars().map(|c| c == '#').collect();

    lines.next();

    let image_lines = lines
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let image = Image {
        fill_pixels: false,
        pixels: image_lines,
    };

    (alg, image)
}

fn main() {
    let (algorithm, mut image) = read_input("day20/input.txt");

    {
        let image = image.enhanced(&algorithm).enhanced(&algorithm);
        let x: usize = image
            .pixels
            .iter()
            .map(|row| row.iter().filter(|x| **x).count())
            .sum();

        println!("Part1 - lit pixels: {}", x)
    }

    for _ in 0..50 {
        image = image.enhanced(&algorithm);
    }

    let x: usize = image
        .pixels
        .iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .sum();

    println!("Part2 - lit pixels: {}", x);
}
