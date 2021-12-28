use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Error {
    ParseError,
}

#[derive(Debug, PartialEq, Eq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Ident(Register),
    NumberLiteral(i64),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Register>().map(|r| Self::Ident(r)).or_else(|_| {
            s.parse::<i64>()
                .map(|x| Self::NumberLiteral(x))
                .map_err(|_| Error::ParseError)
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Inp(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let op = match tokens.next() {
            Some(op) => op,
            None => unreachable!(),
        };

        match op {
            "inp" => {
                let register = tokens.next().unwrap().parse::<Register>()?;
                Ok(Instruction::Inp(register))
            }
            "add" => {
                let register = tokens.next().unwrap().parse::<Register>()?;
                let value = tokens.next().unwrap().parse::<Value>()?;
                Ok(Instruction::Add(register, value))
            }
            "mul" => {
                let register = tokens.next().unwrap().parse::<Register>()?;
                let value = tokens.next().unwrap().parse::<Value>()?;
                Ok(Instruction::Mul(register, value))
            }
            "div" => {
                let register = tokens.next().unwrap().parse::<Register>()?;
                let value = tokens.next().unwrap().parse::<Value>()?;
                Ok(Instruction::Div(register, value))
            }
            "mod" => {
                let register = tokens.next().unwrap().parse::<Register>()?;
                let value = tokens.next().unwrap().parse::<Value>()?;
                Ok(Instruction::Mod(register, value))
            }
            "eql" => {
                let register = tokens.next().unwrap().parse::<Register>()?;
                let value = tokens.next().unwrap().parse::<Value>()?;
                Ok(Instruction::Eql(register, value))
            }

            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ALU {
    fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn get_register(&self, reg: &Register) -> i64 {
        match reg {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set_register(&mut self, reg: &Register, val: i64) {
        match reg {
            Register::W => self.w = val,
            Register::X => self.x = val,
            Register::Y => self.y = val,
            Register::Z => self.z = val,
        }
    }

    fn get_value(&self, val: &Value) -> i64 {
        match val {
            Value::NumberLiteral(v) => *v,
            Value::Ident(reg) => self.get_register(reg),
        }
    }

    fn execute(&mut self, input: &[i64], instructions: &[Instruction]) {
        let mut input_iter = input.iter();

        for inst in instructions {
            match inst {
                Instruction::Inp(reg) => {
                    let val = input_iter.next().unwrap();
                    self.set_register(reg, *val);
                }
                Instruction::Mul(reg, val) => {
                    let mut acc = self.get_register(reg);
                    acc *= self.get_value(val);
                    self.set_register(reg, acc);
                }
                Instruction::Add(reg, val) => {
                    let mut acc = self.get_register(reg);
                    acc += self.get_value(val);
                    self.set_register(reg, acc);
                }
                Instruction::Div(reg, val) => {
                    let mut acc = self.get_register(reg);
                    acc /= self.get_value(val);
                    self.set_register(reg, acc);
                }
                Instruction::Mod(reg, val) => {
                    let mut acc = self.get_register(reg);
                    acc = acc % self.get_value(val);
                    self.set_register(reg, acc);
                }
                Instruction::Eql(reg, val) => {
                    let mut acc = self.get_register(reg);
                    let lit = self.get_value(val);

                    match acc == lit {
                        true => acc = 1,
                        false => acc = 0,
                    }

                    self.set_register(reg, acc);
                }
            }
        }
    }
}

fn load_instructions(instr: &[&str]) -> Vec<Instruction> {
    instr
        .iter()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>()
}

fn sn_to_input(sn: i64) -> Vec<i64> {
    sn.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day24/input.txt")?;
    let reader = BufReader::new(file);

    let lines: std::io::Result<Vec<String>> = reader.lines().collect();
    let lines = lines?;
    let lines: Vec<_> = lines.iter().map(String::as_str).collect();

    let instructions = load_instructions(&lines);

    for i in (11111111111111..11211791111365 + 1).rev() {
        let input = sn_to_input(i);
        if input.contains(&0) {
            continue;
        }

        let mut alu = ALU::new();
        alu.execute(&input, &instructions);

        if alu.z == 0 {
            println!("Valid: {}", i);
        } else {
            //println!("{}", i);
        }
    }

    Ok(())
}

#[test]
fn test_parse_instruction() {
    assert_eq!(
        "inp w".parse::<Instruction>().unwrap(),
        Instruction::Inp(Register::W)
    );
    assert_eq!(
        "add x 5".parse::<Instruction>().unwrap(),
        Instruction::Add(Register::X, Value::NumberLiteral(5))
    );
    assert_eq!(
        "add x z".parse::<Instruction>().unwrap(),
        Instruction::Add(Register::X, Value::Ident(Register::Z))
    );
    assert_eq!(
        "mul x -1".parse::<Instruction>().unwrap(),
        Instruction::Mul(Register::X, Value::NumberLiteral(-1))
    );
    assert_eq!(
        "div x -1".parse::<Instruction>().unwrap(),
        Instruction::Div(Register::X, Value::NumberLiteral(-1))
    );
    assert_eq!(
        "mod y 100".parse::<Instruction>().unwrap(),
        Instruction::Mod(Register::Y, Value::NumberLiteral(100))
    );
    assert_eq!(
        "eql x z".parse::<Instruction>().unwrap(),
        Instruction::Eql(Register::X, Value::Ident(Register::Z))
    );
}

#[test]
fn simple_program() {
    let instr = load_instructions(&["inp x", "mul x -1"]);
    let mut alu = ALU::new();
    alu.execute(&[5], &instr);
    assert_eq!(alu.x, -5);

    let instr = load_instructions(&["inp z", "inp x", "mul z 3", "eql z x"]);
    let mut alu = ALU::new();
    alu.execute(&[1, 3], &instr);
    assert_eq!(alu.z, 1);

    let instr = load_instructions(&[
        "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2", "add x w",
        "mod x 2", "div w 2", "mod w 2",
    ]);
    let mut alu = ALU::new();
    alu.execute(&[5], &instr);
    assert_eq!([alu.w, alu.x, alu.y, alu.z], [0, 1, 0, 1]);
}
