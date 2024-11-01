use crate::ray::{Point, Direction, Ray};
use crate::shape::*;
use crate::interval::Interval;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    v: Direction,
    u: Direction,
    w: Direction,
}


impl Sphere {
    pub fn new(center: Point, radius: f32, top: Direction, facing: Direction) -> Self {
        Self {center, radius, v: top, w: facing, u: facing.cross(top)}
    }
    // pub fn center(self) -> Point { self.center }
    // pub fn radius(self) -> f32 { self.radius }
    fn _compute_hitrecord(&self, ray: &Ray, t: f32, is_front: bool) -> HitRecord{
        let point = ray.at(t);
        HitRecord {
            point,
            t,
            hittable: self,
            is_front,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let r = ray.origin - self.center;
        let d = ray.direction;
        let a = d.dot(d);
        let b = r.dot(d) * 2.0;
        let c = r.dot(r) - self.radius.powf(2.0);
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }
        let root1 = (-b - delta.sqrt()) / 2.0 / a;
        let root2 = (-b + delta.sqrt()) / 2.0 / a;
        if interval.within(root1) {
            Some(self._compute_hitrecord(ray, root1, true))
        } else if interval.within(root2) {
            Some(self._compute_hitrecord(ray, root2, false))
        } else {
            None
        }

    }
    fn get_normal(&self, _in_ray: &Ray, hit_record: &HitRecord) -> Direction {
        let normal = (hit_record.point - self.center).unitary();
        if hit_record.is_front {
            -normal
        } else {
            normal
        }
    }
    fn get_uv(&self, point: Point) -> (f32, f32) {
        let direction = (point - self.center).unitary();
        let theta = direction.dot(-self.v).acos();
        let direction_in_plane = direction - self.v * direction.dot(self.v);
        let phi = direction_in_plane.dot(self.u).atan2(direction_in_plane.dot(self.w));
        let u = phi / 2.0 / std::f32::consts::PI;
        let v = theta / std::f32::consts::PI;
        (u, v)
    }
}

#[cfg(test)]
mod test {
    

    #[test]
    fn test_hit() {
        use crate::*;
        let sphere = Sphere::new(
            Point::new(0.0, 1.0, 0.0),
            0.8,
            Direction::new(0.0, 1.0, 0.0),
            Direction::new(-1.0, 0.0, 0.0),
        );
        for i in 1..100 {
            let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(i as f32 / 100.0, 1.3, 0.0).unitary());
            let result1 = sphere.hit(&ray, Interval::default());
            let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(-i as f32 / 100.0, 1.3, 0.0).unitary());
            let result2 = sphere.hit(&ray, Interval::default());
            assert_eq!(result1.is_none(), result2.is_none());
        }
    }
}