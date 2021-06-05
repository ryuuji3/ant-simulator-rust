use uuid::Uuid;
use super::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Entity {
    pub id: Uuid,
    pub position: Point,
}

impl Entity {
    pub fn new(position: Point) -> Entity {
        Entity {
            id: Uuid::new_v4(),
            position
        }
    } 
}