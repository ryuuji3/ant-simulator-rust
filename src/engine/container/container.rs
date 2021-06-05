use super::{Entity, Rectangle};

pub trait Container {
    fn query(&self, bounds: &Rectangle) -> Vec<&Entity>;
    fn insert(&mut self, entity: Entity);
}