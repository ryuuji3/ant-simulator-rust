
use super::Point;

pub trait Boundary<T> {
    fn contains(&self, point: &Point) -> bool;

    fn intersects(range: &T) -> bool;
}