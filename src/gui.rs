use piston_window::*;
use plotters::prelude::*;
use crate::tsp::City;
use crate::helper;

pub fn run_gui(input_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let cities = helper::read_points_from_file(&input_file)?;

    // Create a Piston window
    let mut window: PistonWindow = WindowSettings::new("TSP Visualization", [800, 600])
        .exit_on_esc(true)
        .build()?;

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            // Plot the champion path
            plot_champion_path(&mut window);
        }
    }

    Ok(())
}

fn plot_champion_path(window: &mut PistonWindow) {
    //current_path: &Vec<City>, best_path: &Vec<City>, 

}