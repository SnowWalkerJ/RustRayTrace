use rand::random;
use crate::material::*;
use crate::{Color, Direction, HitRecord, Point, Ray};

pub struct MixtureMaterial {
    prob: f32,
    material1: Box<dyn Material>,
    material2: Box<dyn Material>,
}

impl MixtureMaterial {
    pub fn new(prob: f32, material1: impl Material + 'static, material2: impl Material + 'static) -> Self {
        Self {prob,material1:  Box::new(material1), material2: Box::new(material2)}
    }
}

impl Material for MixtureMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, normal: Direction) -> Option<Ray> {
        let realization = random::<f32>();
        if realization < self.prob {
            self.material1.scatter(ray, hit_record, normal)
        } else {
            self.material2.scatter(ray, hit_record, normal)
        }
    }

    fn emit(&self, point: Point) -> Color {
        let c1 = self.material1.emit(point);
        let c2 = self.material2.emit(point);
        (c1 * self.prob + c2 * (1.0 - self.prob)).truncate()
    }
}