
use super::Point;

pub trait Boundary<T> {
    fn contains(&self, point: &Point) -> bool;

    fn intersects(&self, range: &T) -> bool;
}