// models
mod ant;
mod colony;
mod food;
mod pheromone;
mod simulator;

use ant::Ant;
use colony::Colony;
use food::Food;
use pheromone::Pheromone;

use super::engine::{ World };

// expose engine constructs to simulator
use crate::Point;

// actual simulation
pub use simulator::Simulator;