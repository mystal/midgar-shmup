use specs::{Join, Read, ReadStorage, System, WriteStorage};

use components::*;
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
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, colliders, mut transforms, mut velocities) = data;

        // TODO: Try to move entities, clamp to level bounds.
        for (collider, transform, velocity) in (&colliders, &mut transforms, &mut velocities).join() {
            transform.position += **velocity * dt.0;
        }

        // TODO: Check overlap with any entities and emit events?
        for (collider, pos) in (&colliders, &transforms).join() {
            // TODO: If player hits enemy, die
            // TODO: If bullet hits enemy, enemy dies
        }
    }
}
