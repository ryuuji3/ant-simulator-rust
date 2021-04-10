use super::{Colony, Food, Pheromone, World };

pub struct Simulator {
    world: World,
    
    colony: Colony,
    food: Vec<Food>,
    pheromones: Vec<Pheromone>,
}