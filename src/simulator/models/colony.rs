use super::{ Ant, Point };

pub struct Colony {
    position: Point,
    ants: Vec<Ant>,
}

impl Colony {
    pub fn new(position: Point, number: usize) -> Colony {
        Colony {
            position,
            ants: vec![
                Ant::new(position);
                number
            ],
        }
    }
}