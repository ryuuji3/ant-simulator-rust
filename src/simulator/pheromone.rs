use super::Point;

#[derive(Copy, Clone)]
pub struct Pheromone {
    pub position: Point,
    pub pheromone_type: PheromoneType,
}

#[derive(Copy, Clone)]
pub enum PheromoneType {
    Home,
    Food,
}