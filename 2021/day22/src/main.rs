use regex::Regex;
use std::{collections::BTreeSet, str::FromStr};

mod inputs;

#[derive(Debug, PartialEq, Eq)]
struct Step {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl FromStr for Step {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(.+) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$").unwrap();
        let captures = re.captures(s).unwrap();

        let on = &captures[1] == "on";

        let step = Self {
            on: on,
            x: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
            y: (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
            z: (captures[6].parse().unwrap(), captures[7].parse().unwrap()),
        };

        Ok(step)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CompressionMap {
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
}

fn load_steps(s: &str) -> Vec<Step> {
    s.split('\n').map(|s| s.parse::<Step>().unwrap()).collect()
}

fn calculate_cube(initial: bool, steps: &Vec<Step>) -> BTreeSet<(u16, u16, u16)> {
    let mut cubes_on: BTreeSet<(u16, u16, u16)> = BTreeSet::new();

    for (i, step) in steps.iter().enumerate() {
        println!("{}/{}, {}", i + 1, steps.len(), cubes_on.len());

        if initial
            && (step.x.0 < -50
                || step.x.1 > 50
                || step.y.0 < -50
                || step.y.1 > 50
                || step.z.0 < -50
                || step.z.1 > 50)
        {
            continue;
        }

        let mut f: Box<dyn FnMut((u16, u16, u16))> = match step.on {
            true => Box::new(|x| {
                cubes_on.insert(x);
            }),
            false => Box::new(|x| {
                cubes_on.remove(&x);
            }),
        };

        for x in step.x.0..step.x.1 + 1 {
            for y in step.y.0..step.y.1 + 1 {
                for z in step.z.0..step.z.1 + 1 {
                    f((x as u16, y as u16, z as u16));
                }
            }
        }
    }

    cubes_on
}

fn compress_steps(steps: &Vec<Step>) -> (Vec<Step>, CompressionMap) {
    let mut mappings = CompressionMap {
        x: vec![1],
        y: vec![1],
        z: vec![1],
    };

    let mut x_points: Vec<i32> = steps.iter().map(|s| vec![s.x.0, s.x.1]).flatten().collect();
    x_points.sort();
    x_points.dedup();
    for window in x_points.windows(2) {
        mappings.x.push(window[1] - window[0] - 1);
        mappings.x.push(1);
    }

    let mut y_points: Vec<i32> = steps.iter().map(|s| vec![s.y.0, s.y.1]).flatten().collect();
    y_points.sort();
    y_points.dedup();
    for window in y_points.windows(2) {
        mappings.y.push(window[1] - window[0] - 1);
        mappings.y.push(1);
    }

    let mut z_points: Vec<i32> = steps.iter().map(|s| vec![s.z.0, s.z.1]).flatten().collect();
    z_points.sort();
    z_points.dedup();
    for window in z_points.windows(2) {
        mappings.z.push(window[1] - window[0] - 1);
        mappings.z.push(1);
    }

    let new_steps = steps
        .iter()
        .map(|step| Step {
            on: step.on,
            x: (
                x_points.iter().position(|x| *x == step.x.0).unwrap() as i32 * 2,
                x_points.iter().position(|x| *x == step.x.1).unwrap() as i32 * 2,
            ),
            y: (
                y_points.iter().position(|x| *x == step.y.0).unwrap() as i32 * 2,
                y_points.iter().position(|x| *x == step.y.1).unwrap() as i32 * 2,
            ),
            z: (
                z_points.iter().position(|x| *x == step.z.0).unwrap() as i32 * 2,
                z_points.iter().position(|x| *x == step.z.1).unwrap() as i32 * 2,
            ),
        })
        .collect();

    (new_steps, mappings)
}

fn calculate_cubes_on(cubes: &BTreeSet<(u16, u16, u16)>, mapping: &CompressionMap) -> usize {
    let mut count = 0;

    for &(x, y, z) in cubes {
        count += mapping.x[x as usize] as usize
            * mapping.y[y as usize] as usize
            * mapping.z[z as usize] as usize;
    }

    count
}

fn main() {
    let steps = load_steps(inputs::INPUT);
    let cubes_on = calculate_cube(true, &steps);
    println!("Part1: {}", cubes_on.len());

    let (compressed, mapping) = compress_steps(&steps);
    let cubes = calculate_cube(false, &compressed);
    println!("Part2: {}", calculate_cubes_on(&cubes, &mapping));
}

#[test]
fn test_step_from_str() {
    let s = "on x=-43..9,y=-37..16,z=-24..23";
    let step = s.parse::<Step>().unwrap();

    assert_eq!(
        step,
        Step {
            on: true,
            x: (-43, 9),
            y: (-37, 16),
            z: (-24, 23)
        }
    )
}

#[test]
fn test_calc_cube() {
    let steps = load_steps(inputs::PART1_TEST_INPUT);
    assert_eq!(calculate_cube(true, &steps).len(), 590784);
}

#[test]
fn test_compress_steps() {
    let steps = vec![
        Step {
            on: true,
            x: (1, 10),
            y: (5, 8),
            z: (2, 3),
        },
        Step {
            on: true,
            x: (5, 11),
            y: (4, 9),
            z: (6, 8),
        },
    ];

    assert_eq!(calculate_cube(false, &steps).len(), 206);

    let (compressed, map) = compress_steps(&steps);

    assert_eq!(
        compressed,
        vec![
            Step {
                on: true,
                x: (0, 4),
                y: (2, 4),
                z: (0, 2)
            },
            Step {
                on: true,
                x: (2, 6),
                y: (0, 6),
                z: (4, 6)
            }
        ]
    );

    assert_eq!(
        map,
        CompressionMap {
            x: vec![1, 3, 1, 4, 1, 0, 1],
            y: vec![1, 0, 1, 2, 1, 0, 1],
            z: vec![1, 0, 1, 2, 1, 1, 1],
        }
    );

    let cubes = calculate_cube(false, &compressed);
    assert_eq!(calculate_cubes_on(&cubes, &map), 206)
}

#[test]
fn test_calc_cube_2() {
    let steps = load_steps(inputs::PART2_TEST_INPUT);

    let (steps, mappings) = compress_steps(&steps);

    let cube = calculate_cube(false, &steps);
    assert_eq!(calculate_cubes_on(&cube, &mappings), 2758514936282235);
}
