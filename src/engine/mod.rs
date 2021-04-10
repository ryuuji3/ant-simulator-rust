// structs
mod world;
mod rectangle;
mod quadtree;
mod point;

use quadtree::QuadTree;

pub use world::World;
// shapes
pub use boundary::Boundary; // allow for custom implementations

pub use point::Point;
pub use rectangle::Rectangle;

// traits
mod boundary;