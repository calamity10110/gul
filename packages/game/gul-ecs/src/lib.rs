pub use hecs::*;

pub struct WorldManager {
    pub world: hecs::World,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            world: hecs::World::new(),
        }
    }

    pub fn spawn<C: hecs::Component>(&mut self, component: C) -> hecs::Entity {
        self.world.spawn((component,))
    }
}
