use super::{ Rectangle, Point };

pub struct QuadTree {
    bounds: Rectangle,
    capacity: i32,
    points: Vec<Point>,
}

impl QuadTree {
    pub fn new(bounds: Rectangle, capacity: i32) -> QuadTree {
        QuadTree {
            bounds,
            capacity,
            points: vec![],
        }
    }
}