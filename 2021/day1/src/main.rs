use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

fn get_sliding_window_values(measurements: &Vec<u32>) -> Vec<u32> {
    measurements
        .windows(3)
        .map(|f| f.into_iter().sum())
        .collect()
}

fn get_number_of_increasing_measurements(input_file: &str) -> Result<u64, Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut counter = 0;

    let lines: Result<Vec<u32>, ParseIntError> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>())
        .collect();
    let measurements = lines?;

    let measurements = get_sliding_window_values(&measurements);

    let mut last_measurement = measurements[0];
    for measurement in measurements.iter().skip(1) {
        if *measurement > last_measurement {
            counter += 1;
        }

        last_measurement = *measurement
    }

    Ok(counter)
}

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_owned());

    match get_number_of_increasing_measurements(&input_file) {
        Err(err) => eprintln!("Failed to get measurements: {}", err),
        Ok(measurements) => println!("Measurements: {}", measurements),
    }
}
