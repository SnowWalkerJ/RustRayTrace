#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval {min, max}
    }
    pub fn begin(value: f32) -> Self {
        Self::new(value, f32::INFINITY)
    }
    pub fn end(value: f32) -> Self {
        Self::new(-f32::INFINITY, value)
    }
    pub fn universe() -> Self {
        Self::new(-f32::INFINITY, f32::INFINITY)
    }
    pub fn within(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }
    pub fn default() -> Self { Self {min: 1e-4, max: f32::INFINITY} }
    pub fn unit() -> Self { Self {min: 0.0, max: 1.0} }
    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
}