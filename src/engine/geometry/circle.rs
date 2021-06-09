use super::{Point, Rotation, Shape};

pub struct Circle {
    pub origin: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        Rectangle {
            origin: Point::create_point(x, y),
            radius,
        }
    }
}

impl Shape<Circle> for Circle {
    fn contains(&self, point: &Point) -> bool {
        unimplemented!();
    }

    fn intersects(&self, range: &Circle) -> bool {
        unimplemented!();
    }
}