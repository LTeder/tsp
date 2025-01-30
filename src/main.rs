extern crate clap;

mod tsp;
mod helper;
mod gui;

use clap::{Parser, Subcommand};
use tsp::Simulation;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
        TSP {
        #[arg(short = 'f', long, default_value = "data/square.txt")]
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
        survival_rate: f64,

        #[arg(short = 'o', long)]
        output_csv: Option<String>,
    },
    Render {
        #[arg(short, long)]
        input: String,
    }
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::TSP{points_filename, iterations, population_size,
                           crossover_rate, mutation_rate, survival_rate, output_csv}) => {

            let mut cities = Vec::new();
            match helper::read_points_from_file(&points_filename) {
                Ok(points) => {
                    cities = points
                }
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        
            // Run simulation
            let mut sim = Simulation::new(
                cities,
                *population_size,
                *iterations,
                *crossover_rate,
                *mutation_rate,
                *survival_rate,
                output_csv.clone()
            );
            sim.run();
        }
        Some(Commands::Render{input}) => {
            if let Err(e) = gui::run_gui(input.clone()) {
                eprintln!("Error running GUI: {}", e);
            }
        }
        None => {
            println!("Please specify a command. Use --help for more information.");
        }
    }
}
