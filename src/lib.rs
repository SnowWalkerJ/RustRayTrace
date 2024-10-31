pub mod vec3;
mod ray;
mod color;
mod shapes;
mod canvas;
mod camera;
mod interval;
mod material;
mod materials;
mod texture;
mod textures;
mod object;
mod shape;
mod scene;
mod renderer;
mod misc;

pub use camera::Camera;
pub use canvas::Canvas;
pub use color::Color;
pub use shape::{HitRecord, Shape};
pub use ray::{Direction, Point, Ray};
pub use scene::Scene;
pub use interval::Interval;
pub use shapes::*;
pub use textures::*;
pub use materials::*;
pub use renderer::Renderer;
pub use misc::write_ppm_to_file;


#[cfg(test)]
mod test {
    use std::io::Write;
    use crate::{vec3, Canvas, Color};
    #[test]
    fn test_example() {
        let x = vec3::Vec3::new(1.0, 2.0, 3.0);
        println!("{:?}", x + x * 2.0);
    }
    #[test]
    fn test_ppm() {
        const WIDTH: usize = 120;
        const HEIGHT: usize = 120;
        let mut canvas = Canvas::new(WIDTH, HEIGHT, Color::black());
        for w in 0..WIDTH {
            for h in 0..HEIGHT {
                canvas.put(w, h, Color::new(w as f32 / WIDTH as f32, h as f32 / HEIGHT as f32, 0.0));
            }
        }
        let ppm = canvas.to_ppm();
        use std::fs;
        let mut file = fs::File::create("test.ppm").expect("Can't create file");
        file.write_all(ppm.as_bytes()).expect("error writing");
    }
}