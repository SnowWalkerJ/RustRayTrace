use crate::{Camera, Canvas, Scene, Ray, Color, Interval};
pub struct Renderer {
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    background_color: Color,
    min_luminance: f32,
}

impl Renderer {
    pub fn new(width: usize, height: usize, samples_per_pixel: usize, max_depth: usize, background_color: Color, min_luminance: f32) -> Self {
        Renderer {width, height, samples_per_pixel, max_depth, background_color, min_luminance}
    }
    pub fn render(&self, camera: &Camera, world: &Scene) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height, Color::black());
        for u in 0..self.width {
            for v in 0..self.height {
                let mut result = Color::black();
                for _sample in 0..self.samples_per_pixel {
                    result = result + self.render_pixel(camera, world, u, v);
                }
                result = (result / self.samples_per_pixel as f32).truncate();
                canvas.put(u, v, result);
            }
        }
        canvas
    }
    fn render_pixel(&self, camera: &Camera, world: &Scene, u: usize, v: usize) -> Color {
        let fu = (u as f32 - (self.width - 1) as f32 * 0.5) / (self.width - 1) as f32;
        let fv = (v as f32 - (self.height - 1) as f32 * 0.5) / (self.width - 1) as f32;
        let du = 1.0 / (self.width - 1) as f32;
        let ray = camera.sample_ray(fu, fv, du, du);
        self.do_ray(world, &ray, 0, Color::white())
    }

    fn do_ray(&self, world: &Scene, ray: &Ray, depth: usize, albedo: Color) -> Color {
        if depth >= self.max_depth || albedo.luminance() < self.min_luminance {
            return self.background_color * albedo;
        }
        let result = world.hit(&ray, Interval::default());
        match result {
            Some(result) => {
                let texture_albedo = result.get_albedo();
                let light = result.emit() * albedo;
                match result.scatter(ray) {
                    Some(out_ray) => {light + self.do_ray(world, &out_ray, depth + 1, albedo * texture_albedo)}
                    None => light
                }
            }
            None => self.background_color
        }
    }
}