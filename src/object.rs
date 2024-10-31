use crate::material::Material;
use crate::{Color, HitRecord, Ray};
use crate::interval::Interval;
use crate::shape::Shape;
use crate::texture::Texture;

pub struct Object {
    shape: Box<dyn Shape>,
    material: Box<dyn Material>,
    texture: Box<dyn Texture>,
}

pub struct ObjectHitRecord<'a> {
    pub hit_record: HitRecord<'a>,
    pub object: &'a Object,
}

impl<'a> ObjectHitRecord<'a> {
    pub fn get_albedo(&self) -> Color {
        let (u, v) = self.object.shape.get_uv(self.hit_record.point);
        self.object.texture.color(u, v)
    }
    pub fn scatter(&self, in_ray: &Ray) -> Option<Ray> {
        let normal = self.object.shape.get_normal(in_ray, self.hit_record.point);
        self.object.material.scatter(&in_ray, &self.hit_record, normal)
    }
    pub fn emit(&self) -> Color {
        self.object.material.emit(self.hit_record.point)
    }
}

impl Object {
    pub fn new(shape: impl Shape + 'static, texture: impl Texture + 'static, material: impl Material + 'static) -> Self {
        Self {
            shape: Box::new(shape),
            texture: Box::new(texture),
            material: Box::new(material),
        }
    }
    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<ObjectHitRecord> {
        match self.shape.hit(ray, interval) {
            Some(hit_record) => Some(ObjectHitRecord {hit_record, object: self}),
            None => None
        }
    }
    pub fn material(&self) -> &Box<dyn Material> { &self.material }
    pub fn texture(&self) -> &Box<dyn Texture> { &self.texture }
}