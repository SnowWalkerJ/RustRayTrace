use crate::material::Material;
use crate::{Direction, HitRecord, Ray};

pub struct MetalMaterial {
}

impl MetalMaterial {
    pub fn new() -> Self { Self{} }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, normal: Direction) -> Option<Ray> {
        let out_direction = ray.direction + normal * 2.0;
        Some(Ray::new(hit_record.point, out_direction.unitary()))
    }
}