use std::error::Error;
use piston_window::{*, ellipse, line, types::Color};
use crate::tsp::City;
use crate::helper;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const CITY_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const CURRENT_ROUTE_COLOR: Color = [0.2, 0.5, 0.7, 1.0];
const BEST_ROUTE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub fn run_gui(output_csv: String) -> Result<(), Box<dyn Error>> {
    let (cities, paths) = helper::parse_output_csv(&output_csv)?;

    let mut window: PistonWindow = WindowSettings::new("TSP Visualization", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()?;

    let max_coordinate = cities.iter()
        .flat_map(|city| vec![city.x.abs(), city.y.abs()])
        .fold(0.0, f64::max);
    let scale = (WINDOW_HEIGHT.min(WINDOW_WIDTH) * 0.4) / max_coordinate;
    let offset = [WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0];
    
    let mut best_fitness = std::f64::MIN;
    let mut best_path: Vec<usize> = Vec::new();

    for (_, fitness, path) in paths {
        if fitness > best_fitness {
            best_fitness = fitness;
            best_path = path.clone();
        }

        // Handle window events and drawing
        while let Some(event) = window.next() {
            window.draw_2d(&event, |c, g, _| {
                clear([1.0; 4], g);
                draw_cities(&cities, &c, g, scale, offset);
                draw_route(&cities, &best_path, BEST_ROUTE_COLOR, &c, g, scale, offset);
                draw_route(&cities, &path, CURRENT_ROUTE_COLOR, &c, g, scale, offset);
            });
            break; // Process one event per iteration
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

fn draw_cities(
    cities: &Vec<City>,
    c: &Context,
    g: &mut G2d,
    scale: f64,
    offset: [f64; 2]
) {
    for city in cities {
        let x = city.x * scale + offset[0];
        let y = city.y * scale + offset[1];
        ellipse(
            CITY_COLOR,
            [x - 5.0, y - 5.0, 10.0, 10.0],
            c.transform,
            g,
        );
    }
}

fn draw_route(
    cities: &Vec<City>, 
    path: &Vec<usize>, 
    color: [f32; 4], 
    c: &Context, 
    g: &mut G2d,
    scale: f64,
    offset: [f64; 2],
) {
    for i in 0..path.len() - 1 {
        let start = [
            cities[path[i]].x * scale + offset[0],
            cities[path[i]].y * scale + offset[1],
        ];
        let end = [
            cities[path[i + 1]].x * scale + offset[0],
            cities[path[i + 1]].y * scale + offset[1],
        ];
        line(color, 2.0, [start[0], start[1], end[0], end[1]], c.transform, g);
    }
}