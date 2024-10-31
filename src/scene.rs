use crate::interval::Interval;
use crate::object::{Object, ObjectHitRecord};
use crate::shape::Shape;
use crate::texture::Texture;
use crate::material::Material;
use crate::ray::*;

pub struct Scene {
    objects: Vec<Object>
}
impl Scene {
    pub fn new() -> Self { Self {objects: Vec::new() } }
    pub fn add_object(&mut self, shape: impl Shape + 'static, texture: impl Texture + 'static, material: impl Material + 'static) {
        self.objects.push(Object::new(shape, texture, material));
    }
    pub fn hit(&self, ray: &Ray, mut interval: Interval) -> Option<ObjectHitRecord> {
        let mut result = None;
        for object in self.objects.iter() {
            let local_hit = object.hit(ray, interval);
            if local_hit.is_none() {
                continue;
            }
            interval = Interval::new(interval.min, local_hit.as_ref().unwrap().hit_record.t);
            result = local_hit;
        }
        result
    }
}