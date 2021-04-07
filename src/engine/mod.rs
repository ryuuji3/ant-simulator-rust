// structs
mod world;
mod rectangle;
mod quadtree;
mod entity;
mod point;
// traits
mod boundary;

use boundary::Boundary;

pub use world::World;
pub use entity::Entity;
// shapes
pub use point::Point;
pub use rectangle::Rectangle;
