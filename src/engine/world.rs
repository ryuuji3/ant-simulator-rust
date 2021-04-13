use super::{ Entity, Rectangle };

pub struct World<T> {
    bounds: Rectangle,
    entities: Vec<Box<dyn Entity<T>>>,
}

impl<T> World<T> {
    pub fn new(bounds: Rectangle) -> World<T> {
        World {
            bounds,
            entities: vec![],
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }
}