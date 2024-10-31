use crate::Color;
use crate::texture::Texture;
use crate::canvas::Canvas;
use image::ImageReader;

pub struct ImageTexture {
    image: Canvas,
}

impl ImageTexture {
    pub fn open(filename: &str) -> Self {
        let image = ImageReader::open(filename).unwrap().decode().unwrap();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let data = image.as_rgb8().unwrap().as_raw();
        let mut canvas = Canvas::new(width, height, Color::black());
        for y in 0..height {
            for x in 0..width {
                let r = data[(width * y + x) * 3 + 0];
                let g = data[(width * y + x) * 3 + 1];
                let b = data[(width * y + x) * 3 + 2];
                canvas.put(x, y, Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0));
            }
        }
        ImageTexture { image: canvas }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f32, v: f32) -> Color {
        let x = (u * self.image.width() as f32) as usize;
        let y = (v * self.image.height() as f32) as usize;
        self.image.get(x, y)
    }
}