use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        PI * self.radius.powf(2.0)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) < self.radius + other.radius
    }

    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point(x,y),
            radius: r,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);


impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((self.1 -other.1).powf(2.0) + (self.0 -other.0).powf(2.0)).sqrt()
    }
}