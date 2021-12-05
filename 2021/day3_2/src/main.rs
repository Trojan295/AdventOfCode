use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum ReportError {
    MoreThanOneValue,
}

impl Error for ReportError {}

impl Display for ReportError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn count_bits(report: &Vec<&str>, pos: usize) -> (u32, u32) {
    let mut bits = (0, 0);

    for row in report {
        match row.chars().nth(pos) {
            Some('0') => bits.0 += 1,
            Some('1') => bits.1 += 1,
            Some(_) | None => {}
        }
    }

    bits
}

fn get_oxygen_generator_rating(report: &Vec<String>) -> Result<u32, ReportError> {
    let mut report: Vec<&str> = report.iter().map(|x| x.as_str()).collect();
    let row_length = report[0].len();

    let mut pos = 0;
    while report.len() > 1 {
        if pos == row_length {
            return Err(ReportError::MoreThanOneValue);
        }

        let (zeros, ones) = count_bits(&report, pos);

        report = report
            .iter()
            .filter(|row| {
                if ones >= zeros {
                    row.chars().nth(pos).unwrap() == '1'
                } else {
                    row.chars().nth(pos).unwrap() == '0'
                }
            })
            .map(|row| *row)
            .collect();

        pos += 1;
    }

    Ok(bin_to_dec(report[0]))
}

fn get_co2_scrubber_rating(report: &Vec<String>) -> Result<u32, ReportError> {
    let mut report: Vec<&str> = report.iter().map(|x| x.as_str()).collect();
    let row_length = report[0].len();

    let mut pos = 0;
    while report.len() > 1 {
        if pos == row_length {
            return Err(ReportError::MoreThanOneValue);
        }

        let (zeros, ones) = count_bits(&report, pos);

        report = report
            .iter()
            .filter(|row| {
                if zeros <= ones {
                    row.chars().nth(pos).unwrap() == '0'
                } else {
                    row.chars().nth(pos).unwrap() == '1'
                }
            })
            .map(|row| *row)
            .collect();

        pos += 1;
    }

    Ok(bin_to_dec(report[0]))
}

fn bin_to_dec(bin: &str) -> u32 {
    let len = bin.len();
    let mut val: u32 = 0;
    for (i, c) in bin.chars().enumerate() {
        if c == '1' {
            val += 1 << (len - i - 1);
        }
    }
    val
}

fn load_report(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    let report = load_report(&input_file)?;

    let oxygen_gen_rating = get_oxygen_generator_rating(&report)?;
    let co2_scrubber_rating = get_co2_scrubber_rating(&report)?;

    println!("Oxygen generator rating: {}", oxygen_gen_rating);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);
    println!("Result: {}", oxygen_gen_rating * co2_scrubber_rating);
    Ok(())
}
