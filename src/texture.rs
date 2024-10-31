use crate::Color;

pub trait Texture {
    fn color(&self, u: f32, v: f32) -> Color;
}