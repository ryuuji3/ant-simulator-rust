use super::{Entity, Shape};

pub trait Container<T> {
    fn query(&self, bounds: &Shape) -> Vec<&Box<dyn Entity<T>>>;
    fn insert(&mut self, entity: Box<dyn Entity<T>>);
}