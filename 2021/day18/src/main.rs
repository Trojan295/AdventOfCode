#![feature(box_patterns)]

use std::{
    borrow::BorrowMut,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use serde_json::{json, Value};

extern crate serde_json;

#[derive(PartialEq, Debug, Clone)]
enum Sailfish {
    Number(i32),
    Pair(Box<Sailfish>, Box<Sailfish>),
}

struct ExploreResult {
    exploded: bool,
    left: Option<i32>,
    right: Option<i32>,
    destroy: bool,
}

impl ExploreResult {
    fn none() -> Self {
        Self {
            exploded: false,
            destroy: false,
            left: None,
            right: None,
        }
    }

    fn explode(left: i32, right: i32) -> Self {
        Self {
            exploded: true,
            destroy: true,
            left: Some(left),
            right: Some(right),
        }
    }
}

struct SplitResult {
    splited: bool,
}

impl SplitResult {
    fn none() -> Self {
        Self { splited: false }
    }

    fn split() -> Self {
        Self { splited: true }
    }
}

impl FromStr for Sailfish {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Value = serde_json::from_str(s)?;
        Ok(Sailfish::from_value(&v))
    }
}

impl std::fmt::Display for Sailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_value();
        let s = v.to_string();
        f.write_str(&s)
    }
}

impl Sailfish {
    fn to_value(&self) -> serde_json::Value {
        match &self {
            Sailfish::Number(x) => json!(x),
            Sailfish::Pair(left, right) => json!(vec![left.to_value(), right.to_value()]),
        }
    }

    fn from_value(v: &serde_json::Value) -> Self {
        match v {
            Value::Number(n) => Sailfish::Number(n.as_i64().unwrap() as i32),
            Value::Array(els) => Sailfish::Pair(
                Box::new(Sailfish::from_value(&els[0])),
                Box::new(Sailfish::from_value(&els[1])),
            ),
            _ => Sailfish::Number(-1),
        }
    }

    fn add_left(&mut self, x: i32) {
        match self {
            Sailfish::Pair(left, _) => {
                left.add_left(x);
            }
            Sailfish::Number(ref mut v) => *v += x,
        }
    }

    fn add_right(&mut self, x: i32) {
        match self {
            Sailfish::Pair(_, right) => right.add_right(x),
            Sailfish::Number(ref mut v) => *v += x,
        }
    }

    fn split(&mut self) -> SplitResult {
        match self {
            Sailfish::Number(v) => {
                if *v >= 10 {
                    let left = (*v as f64 / 2.0).floor() as i32;
                    let right = (*v as f64 / 2.0).ceil() as i32;

                    *self = Self::Pair(Box::new(Self::Number(left)), Box::new(Self::Number(right)));

                    SplitResult::split()
                } else {
                    SplitResult::none()
                }
            }
            Sailfish::Pair(box left, box right) => {
                let res = left.split();
                if res.splited {
                    return res;
                }

                let res = right.split();
                if res.splited {
                    return res;
                }

                SplitResult::none()
            }
        }
    }

    fn explode(&mut self, depth: usize) -> ExploreResult {
        match self {
            Sailfish::Number(_) => ExploreResult::none(),
            Sailfish::Pair(box Sailfish::Number(left), box Sailfish::Number(right)) => {
                if depth >= 4 {
                    ExploreResult::explode(*left, *right)
                } else {
                    ExploreResult::none()
                }
            }
            Sailfish::Pair(left, right) => {
                let left: &mut Sailfish = left.borrow_mut();
                let right: &mut Sailfish = right.borrow_mut();

                let mut res = left.explode(depth + 1);
                if res.exploded {
                    if let Some(right_val) = res.right {
                        right.add_left(right_val);
                        res.right = None;
                    }

                    if res.destroy {
                        *left = Self::Number(0);
                        res.destroy = false
                    }

                    return res;
                };

                let mut res = right.explode(depth + 1);
                if res.exploded {
                    if let Some(left_val) = res.left {
                        left.add_right(left_val);
                        res.left = None;
                    }

                    if res.destroy {
                        *right = Sailfish::Number(0);
                        res.destroy = false
                    }

                    return res;
                };

                ExploreResult::none()
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).exploded {
                continue;
            }

            if self.split().splited {
                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Sailfish::Number(x) => *x,
            Sailfish::Pair(box left, box right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl std::ops::Add for Sailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut pair = Self::Pair(Box::new(self), Box::new(other));
        pair.reduce();
        pair
    }
}

fn load_input(path: &str) -> Vec<Sailfish> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let lines: std::io::Result<Vec<String>> = reader.lines().collect();
    let lines = lines.unwrap();

    lines
        .into_iter()
        .map(|x| Sailfish::from_str(&x).unwrap())
        .collect()
}

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or("day18/input.txt".to_owned());

    let fishes = load_input(&input_file);

    let x = fishes.clone().into_iter().reduce(|x, y| x + y).unwrap();
    println!("Part1: {:?}", x.magnitude());

    let max_magnitude = fishes
        .iter()
        .map(|x| {
            fishes
                .iter()
                .map(move |y| (x.clone(), y.clone()))
                .collect::<Vec<(Sailfish, Sailfish)>>()
        })
        .flatten()
        .map(|(x, y)| x + y)
        .map(|x| x.magnitude())
        .max()
        .unwrap_or(0);

    println!("Part2: {}", max_magnitude);
}

#[test]
fn test_sailfish_from_str() {
    assert_eq!(
        Sailfish::from_str("[0,1]").unwrap(),
        Sailfish::Pair(Box::new(Sailfish::Number(0)), Box::new(Sailfish::Number(1)))
    );
}

#[test]
fn test_sailfish_to_string() {
    assert_eq!(
        Sailfish::Pair(Box::new(Sailfish::Number(0)), Box::new(Sailfish::Number(1))).to_string(),
        "[0,1]".to_string()
    );
}

#[test]
fn test_explode() {
    let mut s = Sailfish::from_str("[[[[[9,8],1],2],3],4]").unwrap();
    s.explode(0);
    assert_eq!(&s.to_string(), "[[[[0,9],2],3],4]");

    let mut s = Sailfish::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
    s.explode(0);
    assert_eq!(&s.to_string(), "[7,[6,[5,[7,0]]]]");

    let mut s = Sailfish::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();
    s.explode(0);
    assert_eq!(&s.to_string(), "[[6,[5,[7,0]]],3]");

    let mut s = Sailfish::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
    s.explode(0);
    assert_eq!(&s.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

    let mut s = Sailfish::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
    s.explode(0);
    assert_eq!(&s.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
}

#[test]
fn test_split() {
    let mut s = Sailfish::from_str("[10,0]").unwrap();
    s.split();
    assert_eq!(&s.to_string(), "[[5,5],0]");

    let mut s = Sailfish::from_str("[11,0]").unwrap();
    s.split();
    assert_eq!(&s.to_string(), "[[5,6],0]");
}

#[test]
fn test_reduce() {
    let mut s = Sailfish::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    s.reduce();
    assert_eq!(&s.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
}

#[test]
fn test_add() {
    let mut s = Sailfish::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
    let to_add = Sailfish::from_str("[1,1]").unwrap();
    s = s + to_add;
    assert_eq!(&s.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
}
