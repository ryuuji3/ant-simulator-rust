use legion::storage::IntoComponentSource;

pub struct World(legion::World);
pub struct Entity(legion::Entity);

impl World {
    pub fn new() -> World {
        World(legion::World::default())
    }

    pub fn insert<T>(&mut self, components: T) -> Entity 
        where Option<T>: IntoComponentSource
    {
        let entity = self.0.push(components);

        Entity(entity)
    }
}