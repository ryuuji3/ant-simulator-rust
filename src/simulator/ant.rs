use rand::{ thread_rng, Rng };

use super::{ Food, Point };

#[derive(Clone)]
pub struct Ant {
    position: Point,
    rotation: f32,
    food: Option<Food>,
}

impl Ant {
    pub fn new(starting_point: Point) -> Ant {
        Ant {
            position: starting_point,
            rotation: generate_random_angle(),
            food: None,
        }
    }
}

fn generate_random_angle() -> f32 {
    thread_rng().gen_range(0.0..360.0)
}
