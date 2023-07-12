extern crate clap;

mod tsp;
mod helper;

use clap::Parser;
use tsp::Simulation;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long)]
    points_filename: String,

    #[arg(short, long, default_value_t = 100)]
    iterations: usize,

    #[arg(short, long, default_value_t = 50)]
    population_size: usize,

    #[arg(short, long, default_value_t = 0.9)]
    crossover_rate: f64,

    #[arg(short, long, default_value_t = 0.05)]
    mutation_rate: f64,

    #[arg(short, long, default_value_t = 0.5)]
    survival_rate: f64
}

fn main() {
    let args = Args::parse();
    let points_filename = args.points_filename;
    let iterations = args.iterations;
    let population_size = args.population_size;
    let crossover_rate = args.crossover_rate;
    let mutation_rate = args.mutation_rate;
    let survival_rate = args.survival_rate;

    let mut points_vec = Vec::new();
    match helper::read_points_from_file(&points_filename) {
        Ok(points) => {
            points_vec = points
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    // Run simulation
    let mut sim = Simulation::new(
        points_vec,
        iterations,
        population_size,
        crossover_rate,
        mutation_rate,
        survival_rate
    );
    sim.run();
}
