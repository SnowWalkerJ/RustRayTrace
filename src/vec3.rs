use std::ops::{Add, Mul, Sub, Div, Neg};
use rand;


#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x, y, z}
    }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn unitary(self) -> Vec3 {
        self / self.norm()
    }
    pub fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn near_zero(self, threshold: f32) -> bool {
        self.x.abs() < threshold && self.y.abs() < threshold && self.z.abs() < threshold
    }
    pub fn dot(self, rhs: Vec3) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
    pub fn cross(self, rhs: Vec3) -> Self { Self {
        x: self.y * rhs.z - self.z * rhs.y,
        y: self.z * rhs.x - self.x * rhs.z,
        z: self.x * rhs.y - self.y * rhs.x,
    } }
    pub fn random_unit() -> Vec3 {
        let theta1: f32 = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let theta2: f32 = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let x = theta1.sin() * theta2.cos();
        let y = theta2.sin();
        let z = theta1.cos() * theta2.cos();
        Vec3::new(x, y, z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}


impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[test]
fn test_random_unit() {
    for _i in 0..10 {
        let vec = Vec3::random_unit();
        assert!((vec.norm() - 1.0).abs() < 1e-6)
    }
}

#[test]
fn test_cross() {
    let a = Vec3::new(2.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 0.0, 2.0);
    let c = a.cross(b);
    assert_eq!(c.x, 0.0);
    assert_eq!(c.y, -4.0);
    assert_eq!(c.z, 0.0);
}
