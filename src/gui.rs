extern crate piston_window;
extern crate plotters_piston;

use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters_piston::draw_piston_window;

fn plot_champion_path(&self, path: &Path, window: &mut Window<()>)
        -> Result<(), Box<dyn std::error::Error>> {
    window.draw_2d(|c, g, _| {
        let (x_min, x_max) = self.city_list.iter().map(|c| c.x).minmax().into_option().unwrap();
        let (y_min, y_max) = self.city_list.iter().map(|c| c.y).minmax().into_option().unwrap();

        let mut chart = ChartBuilder::on(&c.draw_state, g)
            .caption("Traveling Salesman Problem", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.configure_mesh().draw()?;

        let path_points: Vec<(f64, f64)> = path.order.iter().map(|&index| {
            let city = &self.city_list[index];
            (city.x, city.y)
        }).collect();

        chart.draw_series(LineSeries::new(path_points.into_iter().cycle().take(path.order.len() + 1), &RED))?;

        Ok(())
    });
}