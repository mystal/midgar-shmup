use specs::{Entities, Join, ReadStorage, System};

use components::*;
use config;

pub struct DespawnSystem {
}

impl DespawnSystem {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'a> System<'a> for DespawnSystem {
    type SystemData = (
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (projectiles, transforms, entities) = data;

        // Kill anything past a certain y-value.
        for (transform, entity) in (&transforms, &*entities).join() {
            if transform.position.y > config::GAME_SIZE.y as f32 + 100.0 {
                entities.delete(entity)
                    .unwrap();
            }
        }
        // Kill projectiles past certain y-value.
        // TODO: Kill projectiles after they travel a certain distance?
        for (_, transform, entity) in (&projectiles, &transforms, &*entities).join() {
            if transform.position.y < -100.0 {
                entities.delete(entity)
                    .unwrap();
            }
        }
    }
}
