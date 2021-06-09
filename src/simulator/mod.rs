use rand::{ Rng, thread_rng };

use crate::engine::{Entity, Point, Rotation, World};

pub fn create_simulation() -> World {
    let mut world = World::new();

    setup(&mut world);

    world
}

fn setup(world: &mut World) {
    create_colony(world);
}

fn create_colony(world: &mut World) -> Entity {
    let position = Point { x: 0.0, y: 0.0 };
    let size: usize = 100;

    for _ in 0..size {
        create_ant(world, position);
    }

    world.insert((
        position,
        size,
    ))
}

fn create_ant(world: &mut World, starting_point: Point) -> Entity {
    let random_rotation: f32 = thread_rng().gen_range(0.0..360.0);

    world.insert((
        starting_point,
        random_rotation,
    ))
}