use std::{
    error::Error,
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc,
    thread,
};

#[derive(Debug)]
enum Errors {
    CannotLoad(Box<dyn Error>),
    WrongInput,
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::WrongInput => f.write_str(&format!("wrong input")),
            Errors::CannotLoad(err) => f.write_str(&format!("cannot load: {}", err)),
        }
    }
}

impl Error for Errors {}

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn contains(&self, c: &Point) -> bool {
        let a = &self.start;
        let b = &self.end;

        let denominator: i32 = (a.x * b.y) as i32 + (b.x * c.y) as i32 + (c.x * a.y) as i32
            - (b.y * c.x) as i32
            - (c.y * a.x) as i32
            - (a.y * b.x) as i32;

        if denominator == 0 {
            let (x_max, x_min) = if self.start.x >= self.end.x {
                (self.start.x, self.end.x)
            } else {
                (self.end.x, self.start.x)
            };
            let (y_max, y_min) = if self.start.y >= self.end.y {
                (self.start.y, self.end.y)
            } else {
                (self.end.y, self.start.y)
            };

            return c.x >= x_min && c.x <= x_max && c.y >= y_min && c.y <= y_max;
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Plane {
    size: usize,
    line_counter: Vec<Vec<u32>>,
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in self.line_counter.iter() {
            for c in col.iter() {
                f.write_str(&c.to_string())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Plane {
    fn new(size: usize) -> Plane {
        Plane {
            size: size,
            line_counter: vec![vec![0; size]; size],
        }
    }

    fn mark_clouds(&mut self, lines: &Vec<Line>) {
        let mut handlers = vec![];

        let (tx, rx) = mpsc::channel();

        for line in lines.iter() {
            let line = line.clone();
            let size = self.size;
            let tx = tx.clone();

            let handler = thread::spawn(move || {
                for x in 0..size {
                    for y in 0..size {
                        let p = Point {
                            x: x as u32,
                            y: y as u32,
                        };
                        if line.contains(&p) {
                            tx.send(p).unwrap();
                        }
                    }
                }
            });
            handlers.push(handler);
        }

        for h in handlers {
            h.join().unwrap();
        }
        drop(tx);

        for point in rx {
            self.line_counter[point.x as usize][point.y as usize] += 1;
        }
    }
}

fn load_lines(path: &str) -> Result<Vec<Line>, Errors> {
    let file = File::open(path).map_err(|err| Errors::CannotLoad(Box::new(err)))?;
    let reader = BufReader::new(file);

    let mut lines = vec![];

    for row in reader.lines() {
        let row = row.map_err(|err| Errors::CannotLoad(Box::new(err)))?;
        let mut cols = row.split(" -> ");

        let mut line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
        };

        if let Some(point) = cols.nth(0) {
            let coords: Result<Vec<u32>, Errors> = point
                .split(',')
                .map(|x| x.parse::<u32>().map_err(|_| Errors::WrongInput))
                .collect();
            let coords = coords?;

            if coords.len() == 2 {
                line.start.x = coords[0];
                line.start.y = coords[1];
            } else {
                return Err(Errors::WrongInput);
            }
        } else {
            return Err(Errors::WrongInput);
        }

        if let Some(point) = cols.nth(0) {
            let coords: Result<Vec<u32>, Errors> = point
                .split(',')
                .map(|x| x.parse::<u32>().map_err(|_| Errors::WrongInput))
                .collect();
            let coords = coords?;

            if coords.len() == 2 {
                line.end.x = coords[0];
                line.end.y = coords[1];
            } else {
                return Err(Errors::WrongInput);
            }
        } else {
            return Err(Errors::WrongInput);
        }

        lines.push(line);
    }

    Ok(lines)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());

    let lines = load_lines(&input_file)?;

    //Filter diagonal lines
    let lines = lines.into_iter().collect();

    let mut plane = Plane::new(1000);

    plane.mark_clouds(&lines);

    let mut counter: u32 = 0;
    for cols in plane.line_counter.iter() {
        for count in cols.iter() {
            if *count >= 2 {
                counter += 1;
            }
        }
    }

    println!("Points: {}", counter);

    Ok(())
}

#[test]
fn test_line_contains() {
    let line = Line {
        start: Point { x: 2, y: 2 },
        end: Point { x: 2, y: 1 },
    };

    assert!(line.contains(&Point { x: 2, y: 2 }));
    assert!(line.contains(&Point { x: 2, y: 1 }));
}
