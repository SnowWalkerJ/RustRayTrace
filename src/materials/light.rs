use crate::{Color, Direction, HitRecord, Point, Ray};
use crate::material::Material;


pub struct Light {
    color: Color,
}


impl From<Color> for Light {
    fn from(value: Color) -> Self {
        Self { color: value }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _normal: Direction) -> Option<Ray> {
        None
    }

    fn emit(&self, _point: Point) -> Color {
        self.color
    }
}
