use super::{Ant, Colony, Food, Pheromone, World };

pub struct Simulator {
    world: World<EntityType>,
    
    colony: Colony,
    food: Vec<Food>,
    pheromones: Vec<Pheromone>,
}

enum EntityType {
    Ant(Ant),
    Food(Food),
    Pheromone(Pheromone),
}