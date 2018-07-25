use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use components::*;
use config;
use resources::DeltaTime;

pub struct MotionSystem {
}

impl MotionSystem {
    pub fn new() -> Self {
        MotionSystem {
        }
    }
}

impl<'a> System<'a> for MotionSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, colliders, players, mut transforms, mut velocities, entities) = data;

        for (collider, transform, velocity, entity) in (&colliders, &mut transforms, &mut velocities, &*entities).join() {
            transform.position += **velocity * dt.0;

            // Clamp the player to the level bounds.
            if players.contains(entity) {
                let (min_x, max_x) = (collider.radius, config::GAME_SIZE.x as f32 - collider.radius);
                let (min_y, max_y) = (collider.radius, config::GAME_SIZE.y as f32 - collider.radius);
                if transform.position.x < min_x {
                    transform.position.x = min_x;
                } else if transform.position.x > max_x {
                    transform.position.x = max_x;
                }
                if transform.position.y < min_y {
                    transform.position.y = min_y;
                } else if transform.position.y > max_y {
                    transform.position.y = max_y;
                }
            }
        }

        // TODO: Check overlap with any entities and emit events?
        for (collider, pos) in (&colliders, &transforms).join() {
            // TODO: If player hits enemy, die
            // TODO: If bullet hits enemy, enemy dies
        }
    }
}
