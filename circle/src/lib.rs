#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle {
            center: center,
            radius: radius,
        }
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, second_point: &Point) -> f64 {
        ((second_point.x - self.x).powf(2.0) + (second_point.y - self.y).powf(2.0)).sqrt()
    }
}