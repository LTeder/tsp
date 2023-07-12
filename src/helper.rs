use std::fs::File;
use std::io::{self, BufRead};

use tsp::City;

pub fn read_points_from_file(filename: &str) -> io::Result<Vec<City>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid line format"));
        }

        let x = parts[0].trim().parse::<f64>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number format"))?;
        let y = parts[1].trim().parse::<f64>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number format"))?;

        points.push(City { x, y });
    }
    Ok(points)
}