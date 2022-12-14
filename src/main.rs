use crate::{
    canvas::Canvas,
    shapes::{shape::Shape, sphere::SpherePoints, torus::TorusPoints},
};
use std::time::{Duration, Instant};

mod canvas;
mod shapes;

fn main() {
    let fps = 30;
    let duration = 2.5;
    let height = 30;
    let width = 30;
    let canvas = Canvas::new(height, width);
    canvas.create();
    let mut shape = Shape::new(height, width, -10.0);
    shape.shift((height / 2) as f32, (width / 2) as f32, 10.0);
    shape.shift_light_source(1.0, 0.0, 1.0);
    let start_time = Instant::now();
    while start_time.elapsed().as_millis() < (duration * 1000.0) as u128 {
        shape.rotate(
            std::f32::consts::PI / (fps as f32),
            0.0,
            std::f32::consts::PI / 2.0 / (fps as f32),
        );
        shape.shift_light_source(0.03, 0.0, -0.03);
        #[allow(unused_variables)]
        let points = TorusPoints::new(5.0, 10.0, &shape);
        let points = SpherePoints::new(15.0, &shape);
        let buf = &shape.generate_lumi(points);
        canvas.draw(&buf);
        std::thread::sleep(Duration::from_millis(1000 / fps));
    }
    canvas.destroy();
}
