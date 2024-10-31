use crate::ray::{Direction, Point, Ray};

pub struct Camera {
    fov_tan: f32,
    position: Point,
    focus_dist: f32,
    lens_radius: f32,
    dw: Direction,
    du: Direction,
    dv: Direction,
}

impl Camera {
    pub fn new(
        vertical_fov_indegrees: f32,
        position: Point,
        focus_position: Point,
        right: Direction,
        defocus_radian: f32,
    ) -> Camera {
        let vertical_fov = vertical_fov_indegrees / 180.0 * std::f32::consts::PI;
        let fov_tan = (vertical_fov / 2.0).tan();
        let direction = focus_position - position;
        let focus_dist = direction.norm();
        let dw = direction.unitary();
        let du = right.unitary();
        let dv = dw.cross(du);
        Camera {
            fov_tan,
            position,
            focus_dist,
            lens_radius: defocus_radian * focus_dist,
            dw,
            du,
            dv,
        }
    }

    pub fn sample_ray(&self, u: f32, v: f32, du: f32, dv: f32) -> Ray {
        let target_u = u + (rand::random::<f32>() - 0.5) * du;
        let target_v = v + (rand::random::<f32>() - 0.5) * dv;
        let target_relative_point = (self.dw + (self.du * target_u + self.dv * target_v) * self.fov_tan) * self.focus_dist;
        let (defocus_du, defocus_dv) = Self::sample_in_unit_disk();
        let source_relative_point = (self.du * defocus_du + self.dv * defocus_dv) * self.lens_radius;
        Ray::new(self.position + source_relative_point, (target_relative_point - source_relative_point).unitary())
    }
    fn sample_in_unit_disk() -> (f32, f32) {
        let radius = rand::random::<f32>();
        let theta = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let u = radius * theta.cos();
        let v = radius * theta.sin();
        (u, v)
    }
}