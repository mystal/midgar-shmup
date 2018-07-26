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
        WriteStorage<'a, Bomber>,
        WriteStorage<'a, Shooter>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, players, mut bombers, mut shooters, mut velocities) = data;

        let move_speed = 500.0;

        for (_, bomber, shooter, velocity) in (&players, &mut bombers, &mut shooters, &mut velocities).join() {
            **velocity = input.move_dir * move_speed;

            // Try to fire a projectile.
            if let FireInput::Fire(dir) = input.fire {
                shooter.try_firing(dir);
            }
            if input.bomb {
                bomber.try_bombing();
            }
        }
    }
}
