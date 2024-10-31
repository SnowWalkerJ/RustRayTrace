use std::ops::{Add, Sub, Mul, Neg};
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Point(Vec3);
#[derive(Debug, Copy, Clone)]
pub struct Direction(Vec3);

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}


impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point { Point {0: Vec3::new(x, y, z)}}
    pub fn x(self) -> f32 { self.0.x }
    pub fn y(self) -> f32 { self.0.y }
    pub fn z(self) -> f32 { self.0.z }
    pub fn origin() -> Self { Self::new(0.0, 0.0, 0.0) }
}

impl Direction {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self {0: Vec3::new(x, y, z)}}
    pub fn x(self) -> f32 { self.0.x }
    pub fn y(self) -> f32 { self.0.y }
    pub fn z(self) -> f32 { self.0.z }
    pub fn dot(self, rhs: Self) -> f32 { self.0.dot(rhs.0) }
    pub fn norm(self) -> f32 { self.0.norm() }
    pub fn unitary(self) -> Self { Self { 0: self.0.unitary() }}
    pub fn random_unit() -> Self { Self { 0: Vec3::random_unit() }}
    pub fn cross(self, rhs: Self) -> Self { Self {0: self.0.cross(rhs.0)} }
    pub fn near_zero(self, threshold: f32) -> bool { self.0.near_zero(threshold) }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        Point { 0: self.0 + rhs.0 }
    }
}

impl Sub<Point> for Point {
    type Output = Direction;

    fn sub(self, rhs: Point) -> Self::Output {
        Direction { 0: self.0 - rhs.0 }
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        Direction { 0: self.0 + rhs.0 }
    }
}

impl Sub<Direction> for Direction {
    type Output = Direction;

    fn sub(self, rhs: Direction) -> Self::Output {
        Direction { 0: self.0 - rhs.0 }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction { 0: -self.0 }
    }
}

impl Mul<f32> for Direction {
    type Output = Direction;

    fn mul(self, rhs: f32) -> Self::Output {
        Direction { 0: self.0 * rhs }
    }
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Ray {
        Ray {origin, direction}
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}