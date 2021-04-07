use super::Boundary;
use crate::{ Point };

#[derive(Clone, Copy)]
pub struct Rectangle {
    // center of rectangle
    pub origin: Point,

    pub width: f32,
    pub height: f32,

    pub rotation: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, rotation: f32) -> Rectangle {
        Rectangle {
            origin: Point::create_point(x, y),
            width,
            height,
            rotation,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.origin.x
    }

    pub fn get_y(&self) -> f32 {
        self.origin.y
    }
}

impl Boundary<Rectangle> for Rectangle {
    fn contains(&self, point: &Point) -> bool {
        // factor out the rotation by applying same rotation in reverse to point 
        let rotated_point = point.rotate_around_origin(-self.rotation);

        // can now apply normal logic to rotated point
        let left_x = self.get_x() - self.width / 2.0;
        let right_x = self.get_x() + self.width / 2.0;
        let top_y = self.get_y() - self.height / 2.0;
        let bottom_y = self.get_y() + self.height / 2.0;

        left_x <= rotated_point.x && rotated_point.x <= right_x
        && top_y <= rotated_point.y && rotated_point.y <= bottom_y
    }

    fn intersects(range: &Rectangle) -> bool {
        true // TODO: Does this rectangle intersect another rectangle?
    }
}