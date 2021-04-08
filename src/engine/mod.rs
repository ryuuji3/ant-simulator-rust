// structs
mod world;
mod rectangle;
mod quadtree;
mod entity;
mod point;

pub use world::World;
pub use entity::Entity;
// shapes
pub use boundary::Boundary; // allow for custom implementations

pub use point::Point;
pub use rectangle::Rectangle;

// traits
mod boundary;