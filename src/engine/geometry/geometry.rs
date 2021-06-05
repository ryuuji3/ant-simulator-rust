use super::{ Point };

pub trait Geometry<T: Geometry<T>> {
    fn contains(&self, point: &Point) -> bool;
    fn intersects(&self, range: &T) -> bool;
}