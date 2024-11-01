use crate::Direction;
use crate::interval::Interval;
use super::ray::{Point, Ray};

pub struct HitRecord<'a> {
    pub point: Point,
    pub t: f32,
    pub hittable: &'a dyn Shape,
    pub is_front: bool,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
    fn get_normal(&self, in_ray: &Ray, hit_record: &HitRecord) -> Direction;
    fn get_uv(&self, point: Point) -> (f32, f32);
}