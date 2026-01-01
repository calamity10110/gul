use gul_ecs::WorldManager;

struct Position {
    x: f32,
    y: f32,
}

fn main() {
    let mut game = WorldManager::new();
    let entity = game.spawn(Position { x: 0.0, y: 0.0 });
    println!("Spawned entity: {:?}", entity);
}
