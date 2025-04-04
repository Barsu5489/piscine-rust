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
fn main() {
    let circle = Circle::new(500.0, 500.0, 150.0);
    let circle1 = Circle {
        center: Point(80.0, 115.0),
        radius: 30.0,
    };
    let point_a = Point(1.0, 1.0);
    let point_b = Point(0.0, 0.0);
    println!("circle = {:?} area = {}", circle, circle.area());
    println!("circle = {:?} diameter = {}", circle, circle.diameter());
    println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
    println!(
        "circle and circle1 intersect = {}",
        circle.intersect(circle1)
    );

    println!(
        "distance between {:?} and {:?} is {}",
        point_a,
        point_b,
        point_a.distance(point_b)
    );
}