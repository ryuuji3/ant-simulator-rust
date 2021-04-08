// game engine
mod engine;

// simulator
mod simulator;

// expose game engine to simulator
use engine::World;
use engine::Point;
use engine::Rectangle;
use engine::Boundary;

// public api
pub use simulator::Simulator;