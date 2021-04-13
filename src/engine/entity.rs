use std::fmt::Debug;
use super::{Point, Container};

pub trait Entity<T>: Debug {
    fn get_position(&self) -> Point;

    fn tick(&mut self, tree: &dyn Container<T>);
}