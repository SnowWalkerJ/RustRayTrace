use rand::random;
use crate::{Direction, HitRecord, Interval, Point, Ray, Shape};

struct Medium {
    shape: Box<dyn Shape>,
    density: f32,
}

impl Medium {
    pub fn new(shape: impl Shape + 'static, density: f32) -> Self {
        Self { shape: Box::new(shape), density }
    }

    fn test_hit(&self, ray: &Ray, hit_record: &HitRecord) -> Option<HitRecord> {
        let length = hit_record.t;
        let prob_of_hitting = 1.0 - (-length * self.density).exp();
        let realization = random::<f32>();
        if realization < prob_of_hitting {
            let t = length * realization / prob_of_hitting;
            Some(HitRecord { point: ray.at(t), hittable: self, t, is_front: false})
        } else {
            None
        }
    }
}

impl Shape for Medium {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        match self.shape.hit(ray, interval) {
            None => None,
            Some(hit_record1) => {
                if hit_record1.is_front {
                    let ray2 = Ray::new(hit_record1.point, ray.direction);
                    match self.shape.hit(&ray2, Interval::default()) {
                        None => None,
                        Some(hit_record2) => {
                            self.test_hit(ray, &hit_record2)
                        }
                    }
                } else {
                    self.test_hit(ray, &hit_record1)
                }
            }
        }
    }

    fn get_normal(&self, _in_ray: &Ray, _hit_record: &HitRecord) -> Direction {
        // This normal is never used
        Direction::new(1.0, 0.0, 0.0)
    }

    fn get_uv(&self, _point: Point) -> (f32, f32) {
        // This uv is never used
        (0.0, 0.0)
    }
}
