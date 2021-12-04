use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_number_of_increasing_measurements(input_file: &str) -> Result<u64, Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut counter = 0;

    let mut lines = reader.lines();
    let mut last_measurement: u64 = lines.next().unwrap().unwrap().parse()?;

    for line in lines {
        if let Ok(line) = line {
            let measurement: u64 = line.parse()?;
            if measurement > last_measurement {
                counter += 1;
            }

            last_measurement = measurement
        }
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
