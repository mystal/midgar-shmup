use specs::{Entity, Join, ReadStorage, System, WriteStorage};

use components::*;

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
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cameras, mut transforms) = data;

        let player_position = transforms.get(self.player_entity)
            .expect("Lost player entity in CameraSystem!").position;

        // TODO: Iterate over cameras and get their associated entity positions.
        for (_, transform) in (&cameras, &mut transforms).join() {
            transform.position = player_position;
        }
    }
}
