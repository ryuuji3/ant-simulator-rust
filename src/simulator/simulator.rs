use super::{Colony, Food, Pheromone };

pub struct Simulator {
    colony: Colony,
    food: Vec<Food>,
    pheromones: Vec<Pheromone>,
}