use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use crate::tsp::City;

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

pub fn parse_output_csv(file_path: &str) -> Result<(Vec<City>, Vec<(usize, f64, Vec<usize>)>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers = rdr.headers()?.clone();
    let mut cities = Vec::new();

    for header in headers.iter().skip(2) { // Skip "iteration" and "fitness"
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Invalid city coordinate format")));
        }
        let x: f64 = parts[0].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number format for x"))?;
        let y: f64 = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number format for y"))?;
        cities.push(City { x, y });
    }

    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let iteration: usize = record[0].parse()?;
        let fitness: f64 = record[1].parse()?;
        let path: Vec<usize> = record.iter().skip(2).map(|s| s.parse().unwrap()).collect();
        data.push((iteration, fitness, path));
    }

    Ok((cities, data))
}