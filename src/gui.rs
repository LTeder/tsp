use crate::helper;
use crate::tsp::City;
use piston_window::{ellipse, line, types::Color, *};
use std::error::Error;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const CITY_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const CURRENT_ROUTE_COLOR: Color = [0.5, 0.7, 0.9, 1.0];
const BEST_ROUTE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Gui {
    cities: Vec<City>,
    paths: Vec<(usize, f64, Vec<usize>)>,
    window: PistonWindow,
    scale: f64,
    offset: [f64; 2],
    best_path: Vec<usize>,
    best_fitness: f64,
    update_interval: Duration,
    last_update: Instant,
}

struct GuiRenderer<'a> {
    cities: &'a [City],
    scale: f64,
    offset: [f64; 2],
    iteration: usize,
}

impl<'a> GuiRenderer<'a> {
    fn draw_cities(&self, c: &Context, g: &mut G2d) {
        for city in self.cities {
            let x = city.x * self.scale + self.offset[0];
            let y = city.y * self.scale + self.offset[1];
            ellipse(CITY_COLOR, [x - 5.0, y - 5.0, 10.0, 10.0], c.transform, g);
        }
    }

    fn draw_route(&self, path: &[usize], color: Color, radius: f64, c: &Context, g: &mut G2d) {
        for pair in path.windows(2) {
            let start = [
                self.cities[pair[0]].x * self.scale + self.offset[0],
                self.cities[pair[0]].y * self.scale + self.offset[1],
            ];
            let end = [
                self.cities[pair[1]].x * self.scale + self.offset[0],
                self.cities[pair[1]].y * self.scale + self.offset[1],
            ];
            line(color, radius, [start[0], start[1], end[0], end[1]], c.transform, g);
        }
    }

    fn draw_iteration(&self, c: &Context, g: &mut G2d) {
        // Display iteration count using rectangles in rows of 10
        let color = [0.0, 0.0, 0.0, 1.0];
        let rect_width = 6.0;
        let rect_height = 6.0;
        let spacing = 2.0;
        let row_height = rect_height + spacing;
        
        for i in 0..self.iteration {
            let row = i / 10;
            let col = i % 10;
            rectangle(
                color,
                [
                    10.0 + col as f64 * (rect_width + spacing), 
                    10.0 + row as f64 * row_height, 
                    rect_width, 
                    rect_height
                ],
                c.transform,
                g
            );
        }
    }

    fn render(&self, c: &Context, g: &mut G2d, current_path: &[usize], best_path: &[usize]) {
        clear([1.0; 4], g);
        self.draw_cities(c, g);
        self.draw_route(best_path, BEST_ROUTE_COLOR, 2., c, g);
        self.draw_route(current_path, CURRENT_ROUTE_COLOR, 1.5, c, g);
        self.draw_iteration(c, g);
    }
}

impl Gui {
    pub fn new(output_csv: String) -> Result<Self, Box<dyn Error>> {
        let (cities, paths) = helper::parse_output_csv(&output_csv)?;

        let window: PistonWindow =
            WindowSettings::new("TSP Visualization", [WINDOW_WIDTH, WINDOW_HEIGHT])
                .exit_on_esc(true)
                .build()?;

        let (scale, offset) = Self::calculate_scale_and_offset(&cities);

        Ok(Self {
            cities,
            paths,
            window,
            scale,
            offset,
            best_path: Vec::new(),
            best_fitness: std::f64::MIN,
            update_interval: Duration::from_millis(500),
            last_update: Instant::now(),
        })
    }

    fn calculate_scale_and_offset(cities: &[City]) -> (f64, [f64; 2]) {
        let min_x = cities.iter().map(|city| city.x).fold(f64::INFINITY, f64::min);
        let max_x = cities.iter().map(|city| city.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = cities.iter().map(|city| city.y).fold(f64::INFINITY, f64::min);
        let max_y = cities.iter().map(|city| city.y).fold(f64::NEG_INFINITY, f64::max);

        let scale_x = WINDOW_WIDTH * 0.8 / (max_x - min_x);
        let scale_y = WINDOW_HEIGHT * 0.8 / (max_y - min_y);
        let scale = scale_x.min(scale_y);
        let offset = [
            (WINDOW_WIDTH - (max_x + min_x) * scale) / 2.0,
            (WINDOW_HEIGHT - (max_y + min_y) * scale) / 2.0,
        ];

        (scale, offset)
    }

    pub fn run(&mut self) {
        let mut renderer = GuiRenderer {
            cities: &self.cities,
            scale: self.scale,
            offset: self.offset,
            iteration: 0,
        };
        for (_, fitness, path) in &self.paths {
            while let Some(event) = self.window.next() {
                if event.render_args().is_some() {
                    self.window.draw_2d(&event, |c, g, _| {
                        renderer.render(&c, g, path, &self.best_path);
                    });
                }
                
                if self.last_update.elapsed() >= self.update_interval {
                    self.last_update = Instant::now();
                    renderer.iteration += 1;
                    break;
                }
            }
            if *fitness > self.best_fitness {
                self.best_fitness = *fitness;
                self.best_path = path.clone();
            }
        }
    }
}

pub fn run_gui(output_csv: String) -> Result<(), Box<dyn Error>> {
    let mut gui = Gui::new(output_csv)?;
    gui.run();
    Ok(())
}