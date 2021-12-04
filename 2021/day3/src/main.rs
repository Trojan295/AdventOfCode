use std::{fs::File, io::BufRead, io::BufReader, ops::BitXor};

struct State {
    ones: Vec<u32>,
    zeros: Vec<u32>,
}

impl State {
    fn new(length: usize) -> State {
        State {
            ones: vec![0; length],
            zeros: vec![0; length],
        }
    }

    fn process_line(&mut self, line: &str) {
        for (i, char) in line.chars().enumerate() {
            match char {
                '0' => self.zeros[i] += 1,
                '1' => self.ones[i] += 1,
                _ => {}
            }
        }
    }

    fn get_gamma(&self) -> u32 {
        let mut gamma = 0;
        let length = self.ones.len();

        for i in 0..length {
            if self.ones[i] > self.zeros[i] {
                gamma += 1 << (length - i - 1);
            }
        }

        gamma
    }

    fn get_epsilon(&self) -> u32 {
        let mut value = 0;
        for i in 0..self.ones.len() {
            value += 1 << (i);
        }

        let gamma = self.get_gamma();
        gamma.bitxor(value)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "input.txt";

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap()?;

    let mut state = State::new(first_line.len());
    state.process_line(&first_line);

    for line in lines {
        let line = line?;
        state.process_line(&line);
    }

    println!("{}", state.get_gamma() * state.get_epsilon());

    Ok(())
}

#[test]
fn test_process_line() {
    let mut state = State::new(3);
    state.process_line("100");
    state.process_line("010");
    state.process_line("101");

    assert_eq!(state.zeros[0], 1);
    assert_eq!(state.zeros[1], 2);
    assert_eq!(state.zeros[2], 2);
    assert_eq!(state.ones[0], 2);
    assert_eq!(state.ones[1], 1);
    assert_eq!(state.ones[2], 1);

    assert_eq!(state.get_gamma(), 4, "gamma is wrong");
    assert_eq!(state.get_epsilon(), 3, "epsilon is wrong");
}
