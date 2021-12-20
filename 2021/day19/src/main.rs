use std::{
    borrow::BorrowMut,
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn twisted(&self) -> Self {
        Self {
            x: self.x,
            y: self.z,
            z: -self.y,
        }
    }

    fn rotated(&self, direction: usize) -> Self {
        match direction {
            1 => Self::from((-self.y, self.x, self.z)),
            2 => Self::from((-self.x, -self.y, self.z)),
            3 => Self::from((self.y, -self.x, self.z)),
            4 => Self::from((self.z, self.y, -self.x)),
            5 => Self::from((-self.z, self.y, self.x)),
            _ => self.clone(),
        }
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        (self.x - other.x).abs() as usize
            + (self.y - other.y).abs() as usize
            + (self.z - other.z).abs() as usize
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    position: Point,
    beacons: HashSet<Point>,
}

impl Scanner {
    fn all_rotations(&self) -> Vec<Self> {
        let mut rots = vec![];
        let beacons = &self.beacons;

        for dir in 0..6 {
            let mut beacons: Vec<Point> = beacons.iter().map(|p| p.rotated(dir)).collect();
            let mut position = self.position.rotated(dir);

            for _ in 0..4 {
                rots.push(Self {
                    position: position,
                    beacons: HashSet::from_iter(beacons.clone()),
                });
                beacons = beacons.into_iter().map(|p| p.twisted()).collect();
                position = position.twisted();
            }
        }

        rots
    }

    fn translate(&self, v: &Point) -> Self {
        Self {
            position: self.position + *v,
            beacons: self.beacons.iter().map(|p| *p + *v).collect(),
        }
    }

    fn find_beacons_relative(&self, other: &Scanner) -> Option<Scanner> {
        for reference_beacon in self.beacons.iter() {
            for other in other.all_rotations() {
                for b in other.beacons.iter() {
                    let translation_vec = -*b + *reference_beacon;
                    let other = other.clone().translate(&translation_vec);

                    let mut counter = 0;
                    for b in other.beacons.iter() {
                        if self.beacons.contains(b) {
                            counter += 1;
                        }
                    }

                    if counter >= 12 {
                        return Some(other);
                    }
                }
            }
        }

        None
    }

    fn add_beacons<'a, T>(&mut self, beacons: T)
    where
        T: Iterator<Item = &'a Point>,
    {
        for b in beacons {
            self.beacons.insert(*b);
        }
    }
}

impl From<(i32, i32, i32)> for Point {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Point::new(x, y, z)
    }
}

fn read_input(path: &str) -> Vec<Scanner> {
    let mut scanners = vec![];

    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let lines: std::io::Result<Vec<String>> = reader.lines().collect();
    let mut lines = lines.unwrap().into_iter();
    let lines = lines.borrow_mut();

    while let Some(_) = lines.next() {
        let data = lines.take_while(|x| !x.is_empty());
        let beacons: Vec<Point> = data
            .map(|line| line.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
            .map(|x: Vec<i32>| Point::from((x[0], x[1], x[2])))
            .collect();

        let scanner = Scanner {
            position: Point::new(0, 0, 0),
            beacons: HashSet::from_iter(beacons),
        };
        scanners.push(scanner);
    }

    scanners
}

fn main() {
    let input = "day19/input.txt";
    let scanners = read_input(input);

    let mut ref_scanner = scanners[0].clone();
    let mut scanner_positions = vec![Point::new(0, 0, 0)];

    let mut remaining = VecDeque::new();
    for s in scanners.iter().skip(1) {
        remaining.push_back(s);
    }

    while remaining.len() > 0 {
        let s = remaining.pop_front().unwrap();

        let scanner = ref_scanner.find_beacons_relative(s);
        if scanner.is_none() {
            remaining.push_back(s);
            continue;
        }

        let scanner = scanner.unwrap();

        ref_scanner.add_beacons(scanner.beacons.iter());
        scanner_positions.push(scanner.position);
    }

    println!("Beacon number: {}", ref_scanner.beacons.len());

    let distance = scanner_positions
        .iter()
        .map(|x| {
            scanner_positions
                .iter()
                .map(move |y| x.manhattan_distance(y))
                .collect::<Vec<usize>>()
        })
        .flatten()
        .max();

    println!("Manhattan distance: {}", distance.unwrap());
}
