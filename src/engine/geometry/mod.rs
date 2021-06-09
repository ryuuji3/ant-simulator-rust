mod point;
mod rectangle;
mod rotation;

pub use point::Point;
pub use rectangle::Rectangle;
pub use rotation::Rotation;

pub trait Shape<T: Shape<T>> {
    fn contains(&self, point: &Point) -> bool;
    fn intersects(&self, range: &T) -> bool;
}