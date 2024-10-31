use crate::Color;
use crate::texture::Texture;

pub struct CheckerTexture {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
    scale_u: f32,
    scale_v: f32,
    offset_u: f32,
    offset_v: f32,
}

impl CheckerTexture {
    pub fn new(even: impl Texture + 'static,
               odd: impl Texture + 'static,
               scale_u: f32,
               scale_v: f32,
               offset_u: f32,
               offset_v: f32) -> Self {
        CheckerTexture {even: Box::new(even), odd: Box::new(odd), scale_u: scale_u, scale_v: scale_v, offset_u, offset_v}
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f32, v: f32) -> Color {
        assert!(0.0 <= u && u <= 1.0);
        assert!(0.0 <= v && v <= 1.0);
        let transformed_u = (u / self.scale_u) + self.offset_u;
        let transformed_v = (v / self.scale_v) + self.offset_v;
        let integer_u = transformed_u.floor() as u32;
        let integer_v = transformed_v.floor() as u32;
        let float_u = transformed_u - transformed_u.floor();
        let float_v = transformed_v - transformed_v.floor();
        let is_even = (integer_u + integer_v) % 2 == 0;
        if is_even {
            self.even.color(float_u, float_v)
        } else {
            self.odd.color(float_u, float_v)
        }
    }
}