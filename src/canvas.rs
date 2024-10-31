use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    points: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Canvas {
        let mut canvas = Canvas {
            width,
            height,
            points: Vec::with_capacity(width * height)
        };
        for _w in 0..width {
            for _h in 0..height {
                canvas.points.push(color);
            }
        }
        canvas
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.points[x * self.height + y]
    }

    pub fn put(&mut self, x: usize, y: usize, c: Color) {
        self.points[x * self.height + y] = c;
    }
    pub fn to_ppm(&self) -> String {
        let mut result = format!("P3\n{} {}\n255\n", self.width, self.height);
        for h in (0..self.height).rev() {
            for w in 0..self.width {
                let c = self.get(w, h).linear_to_gamma();
                let r = (c.r() * 255.999) as u32;
                let g = (c.g() * 255.999) as u32;
                let b = (c.b() * 255.999) as u32;
                result += format!("{} {} {}\n", r, g, b).as_str();
            }
        }
        result
    }
}


