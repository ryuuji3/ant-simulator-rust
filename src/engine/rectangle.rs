use crate::{ Point };

#[derive(Clone, Copy)]
pub struct Rectangle {
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

    pub fn contains_point(&self, point: Point) -> bool {
        true
    }
}