use crate::material::Material;
use crate::{Direction, HitRecord, Ray};

struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, normal: Direction) -> Option<Ray> {
        let cos_theta = ray.direction.dot(normal);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();
        let relative_refraction_index = if hit_record.is_front {self.refraction_index} else {1.0 / self.refraction_index};
        let target_sin = sin_theta * relative_refraction_index;
        if target_sin > 1.0 {
            // reflection
            let out_direction = ray.direction + normal * 2.0;
            Some(Ray::new(hit_record.point, out_direction.unitary()))
        } else {
            // refraction
            let r_out_perp = (ray.direction + normal * cos_theta) * (1.0 / relative_refraction_index);
            let r_out_parallel = normal * (1.0 - r_out_perp.dot(r_out_perp)).sqrt();
            let out_direction = r_out_perp + r_out_parallel;
            Some(Ray::new(hit_record.point, out_direction.unitary()))
        }
    }
}