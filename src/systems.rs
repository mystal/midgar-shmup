use specs::{Entity, Join, Read, ReadStorage, System, WriteStorage};

use components::*;
use input::PlayerInput;

#[derive(Clone, Debug, Default)]
pub struct DeltaTime(pub f32);

pub struct PhysicsSystem {
}

impl PhysicsSystem {
    pub fn new() -> Self {
        PhysicsSystem {
        }
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, colliders, mut positions, mut velocities) = data;

        // TODO: Try to move entities, clamp to level bounds.
        for (collider, pos, velocity) in (&colliders, &mut positions, &mut velocities).join() {
            pos.0 += velocity.0 * dt.0;
        }

        // TODO: Check overlap with any entities and emit events?
        for (collider, pos) in (&colliders, &positions).join() {
            // TODO: If player hits enemy, die
            // TODO: If bullet hits enemy, enemy dies
        }
    }
}

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
        Read<'a, DeltaTime>,
        Read<'a, PlayerInput>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, input, players, mut velocities) = data;

        let move_speed = 500.0;

        for (_, velocity) in (&players, &mut velocities).join() {
            velocity.0 = input.move_dir * move_speed;

            // TODO: Try to fire a projectile.
        }
    }
}

pub struct CameraSystem {
    player_entity: Entity,
}

impl CameraSystem {
    pub fn new(player_entity: Entity) -> Self {
        CameraSystem {
            player_entity: player_entity,
        }
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cameras, mut positions) = data;

        let player_position = positions.get(self.player_entity)
            .expect("Lost player entity in CameraSystem!").0;

        // TODO: Iterate over cameras and get their associated entity positions.
        for (_, position) in (&cameras, &mut positions).join() {
            position.0 = player_position;
        }
    }
}
