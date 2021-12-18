use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Area(Point, Point);

impl From<((i32, i32), (i32, i32))> for Area {
    fn from(((x1, y1), (x2, y2)): ((i32, i32), (i32, i32))) -> Self {
        let min_x = vec![x1, x2].into_iter().min().unwrap();
        let max_x = vec![x1, x2].into_iter().max().unwrap();
        let min_y = vec![y1, y2].into_iter().min().unwrap();
        let max_y = vec![y1, y2].into_iter().max().unwrap();

        Self {
            0: Point::new(min_x, min_y),
            1: Point::new(max_x, max_y),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn in_area(&self, area: &Area) -> bool {
        self.x >= area.0.x && self.x <= area.1.x && self.y >= area.0.y && self.y <= area.1.y
    }
}

#[test]
fn test_in_area() {
    let area: Area = Area::from(((1, 5), (3, 8)));
    assert!(!Point::new(1, 2).in_area(&area));
    assert!(!Point::new(4, 9).in_area(&area));
    assert!(Point::new(1, 5).in_area(&area));
    assert!(Point::new(3, 8).in_area(&area));
}

struct Probe {
    step: usize,
    velocity: Point,
    position: Point,
}

impl Probe {
    fn new(initial_velocity: &Point) -> Probe {
        Probe {
            position: Point::new(0, 0),
            step: 0,
            velocity: initial_velocity.clone(),
        }
    }

    fn next(&mut self) {
        self.position = self.position + self.velocity;

        self.velocity.y -= 1;
        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        }

        self.step += 1;
    }
}

#[test]
fn test_probe_next() {
    let mut probe = Probe::new(&Point::new(2, 2));
    assert_eq!(probe.velocity, Point::new(2, 2));
    assert_eq!(probe.position, Point::new(0, 0));
    probe.next();
    assert_eq!(probe.velocity, Point::new(1, 1));
    assert_eq!(probe.position, Point::new(2, 2));
    probe.next();
    assert_eq!(probe.velocity, Point::new(0, 0));
    assert_eq!(probe.position, Point::new(3, 3));
    probe.next();
    assert_eq!(probe.velocity, Point::new(0, -1));
    assert_eq!(probe.position, Point::new(3, 3));
    probe.next();
    assert_eq!(probe.velocity, Point::new(0, -2));
    assert_eq!(probe.position, Point::new(3, 2));
}

fn y_target_function(s_y: i32, n: i32) -> Option<i32> {
    let n = n as f32;
    let s_y = s_y as f32;

    // formula derived by hand
    let value = (2.0 * s_y + n * (n + 1.0) - 2.0 * n) / (2.0 * n);

    if value.fract() == 0.0 {
        Some(value as i32)
    } else {
        None
    }
}

fn calculate_max_height(v_y: i32) -> i32 {
    if v_y > 0 {
        (v_y + 1) * v_y / 2
    } else {
        0
    }
}

fn find_max_initial_vertical_velocity(target: &Area) -> i32 {
    let mut max_v = i32::MIN;
    let min_y = target.0.y;

    for target_height in target.0.y..target.1.y + 1 {
        let mut previous_value = i32::MIN;
        for step in 1.. {
            if let Some(value) = y_target_function(target_height, step) {
                if value <= previous_value || value > min_y.abs() {
                    break;
                } else {
                    previous_value = value;

                    if value > max_v {
                        max_v = value;
                    }
                }
            }
        }
    }
    max_v
}

fn main() {
    let target_area: Area = Area::from(((156, -110), (202, -69)));
    let max_v0_y = find_max_initial_vertical_velocity(&target_area);
    let max_s_y = calculate_max_height(max_v0_y);

    println!("Part1: {}", max_s_y);

    let mut initial_velocities: HashSet<Point> = HashSet::new();

    for v0_y in target_area.0.y..max_v0_y + 1 {
        for v0_x in 1..target_area.1.x + 1 {
            let initial_velocity = &Point::new(v0_x, v0_y);
            let mut probe = Probe::new(initial_velocity);

            while probe.velocity.y >= -110 {
                probe.next();

                if probe.position.in_area(&target_area) {
                    initial_velocities.insert(*initial_velocity);
                }
            }
        }
    }

    println!("Part2: {}", initial_velocities.len())
}
