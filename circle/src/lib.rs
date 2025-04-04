#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub fn diameter(&self) -> f32 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64).powi(2)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance < (self.radius + other.radius) as f64
    }

    pub fn new(x: f64, y: f64, radius: f32) -> Self {
        Circle {
            center: Point(x as f32, y as f32),
            radius,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        (((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)) as f64).sqrt()
    }
}
