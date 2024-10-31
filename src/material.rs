use crate::ray::*;
use crate::{Color, HitRecord};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, normal: Direction) -> Option<Ray>;
    fn emit(&self, _point: Point) -> Color {
        Color::black()
    }
}