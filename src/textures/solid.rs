use crate::Color;
use crate::texture::Texture;


pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(color: Color) -> Self { Self {color} }
}

impl Texture for SolidTexture {
    fn color(&self, _u: f32, _v: f32) -> Color {
        self.color
    }
}


impl From<Color> for SolidTexture {
    fn from(value: Color) -> Self {
        Self::new(value)
    }
}