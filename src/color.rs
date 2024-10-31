use crate::interval::Interval;
use super::vec3::Vec3;
use std::ops::{Add, Mul, Div};
#[derive(Debug, Copy, Clone)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { 0: Vec3::new(r, g, b) }
    }
    
    pub fn r(self) -> f32 { self.0.x }
    pub fn g(self) -> f32 { self.0.y }
    pub fn b(self) -> f32 { self.0.z }
    pub fn luminance(self) -> f32 { 0.2126 * self.r() + 0.7152 * self.g() + 0.0722 * self.b() }
    pub fn truncate(self) -> Self {
        let interval = Interval::new(0.0, 1.0);
        Self::new(
            interval.clamp(self.r()),
            interval.clamp(self.g()),
            interval.clamp(self.b()),
        )
    }
    pub fn linear_to_gamma(&self) -> Color {
        Color::new(self.r().sqrt(), self.g().sqrt(), self.b().sqrt())
    }

    pub fn green() -> Self { Self::new(0.0, 1.0, 0.0) }
    pub fn red() -> Self { Self::new(1.0, 0.0, 0.0) }
    pub fn blue() -> Self { Self::new(0.0, 0.0, 1.0) }
    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
    pub fn black() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn yellow() -> Self { Self::new(1.0, 1.0, 0.0) }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r() * rhs.r(),
            self.g() * rhs.g(),
            self.b() * rhs.b(),
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            self.r() * rhs,
            self.g() * rhs,
            self.b() * rhs,
        )
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r() + rhs.r(),
            self.g() + rhs.g(),
            self.b() + rhs.b(),
        )
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(
            self.r() / rhs,
            self.g() / rhs,
            self.b() / rhs,
        )
    }
}