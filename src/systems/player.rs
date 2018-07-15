use specs::{Join, Read, ReadStorage, System, WriteStorage};

use components::*;
use input::{FireInput, PlayerInput};

pub struct PlayerSystem {
}

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {
        }
    }
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Read<'a, PlayerInput>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Shooter>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, players, mut shooters, mut velocities) = data;

        let move_speed = 500.0;

        for (_, shooter, velocity) in (&players, &mut shooters, &mut velocities).join() {
            **velocity = input.move_dir * move_speed;

            // Try to fire a projectile.
            if let FireInput::Fire(dir) = input.fire {
                shooter.try_firing(dir);
            }
        }
    }
}
