use crate::*;


pub struct Quard {
    point: Point,
    u_norm: f32,
    v_norm: f32,
    unitary_u: Direction,
    unitary_v: Direction,
    unitary_n: Direction,
    u_orthogonal_v: Direction,
    d: f32,
}

impl Quard {
    pub fn new(point: Point, u: Direction, v: Direction) -> Self {
        let norm = u.cross(v);
        let unitary_n = norm.unitary();
        let d = (point - Point::origin()).dot(unitary_n);
        let unitary_u = u.unitary();
        let unitary_v = v.unitary();
        let uv = unitary_u.dot(unitary_v);
        let u_parallel_v = unitary_v * uv;
        let u_orthogonal_v = unitary_u - u_parallel_v;
        assert!(u_parallel_v.dot(u_orthogonal_v).abs() < 1e-5);
        Self { point, u_norm: u.norm(), v_norm: v.norm(), unitary_u, unitary_v, unitary_n, u_orthogonal_v, d }
    }
}

impl Shape for Quard {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let np = self.unitary_n.dot(ray.origin - Point::origin());
        let nd = self.unitary_n.dot(ray.direction);
        if nd.abs() < 1e-4 {
            return None;
        }
        let t = (self.d - np) / nd;
        if !interval.within(t) {
            return None;
        }
        let point = ray.at(t);
        let (u, v) = self.get_uv(point);
        let unit_interval = Interval::unit();
        if unit_interval.within(u) && unit_interval.within(v) {
            Some(HitRecord {
                point,
                t,
                hittable: self
            })
        } else {
            None
        }
    }

    fn get_normal(&self, in_ray: &Ray, _point: Point) -> Direction {
        if in_ray.direction.dot(self.unitary_n) > 0.0 {
            -self.unitary_n
        } else {
            self.unitary_n
        }
    }

    fn get_uv(&self, point: Point) -> (f32, f32) {
        let relative = point - self.point;
        let relative_orthogonal_v = relative.dot(self.u_orthogonal_v);
        let amount_u = relative_orthogonal_v / self.unitary_u.dot(self.u_orthogonal_v) / self.u_norm;
        let relative_parallel_v = relative - self.unitary_u * (amount_u * self.u_norm);
        let amount_v = relative_parallel_v.dot(self.unitary_v) / self.v_norm;
        // assert!((self.unitary_u * (self.u_norm * amount_u) + self.unitary_v * (self.v_norm * amount_v) - relative).near_zero(1e-5));
        (amount_u, amount_v)
    }
}