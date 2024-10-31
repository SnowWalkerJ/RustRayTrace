use crate::material::*;
use crate::{Direction, HitRecord, Ray};


pub struct LambertianDiffuse {
}

impl LambertianDiffuse {
    pub fn new() -> Self { Self {} }
}


impl Material for LambertianDiffuse {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, normal: Direction) -> Option<Ray> {
        let mut random_hemisphere = Direction::random_unit();
        if random_hemisphere.dot(normal) > 0.0 {
            random_hemisphere = -random_hemisphere;
        }
        let mut direction = normal + random_hemisphere;
        if direction.x().abs() < 1e-6 && direction.y().abs() < 1e-6 && direction.z().abs() < 1e-6 {
            direction = normal;
        }
        direction = direction.unitary();
        Some(Ray { origin: hit_record.point, direction})
    }
}