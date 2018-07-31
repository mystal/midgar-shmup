use specs::{Entities, Read, ReadStorage, System, WriteStorage};

use components::*;
use systems::CollisionEvent;

pub struct PickupSystem {
}

impl PickupSystem {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'a> System<'a> for PickupSystem {
    type SystemData = (
        Read<'a, Vec<CollisionEvent>>,
        ReadStorage<'a, Pickup>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Bomber>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (collisions, pickups, players, mut bombers, entities) = data;

        for event in &*collisions {
            // If the player collided with a pickup.
            if let (Some(_), Some(pickup)) = (players.get(event.entity_a), pickups.get(event.entity_b)) {
                match pickup {
                    Pickup::Bomb => {
                        if let Some(bomber) = bombers.get_mut(event.entity_a) {
                            bomber.count += 1;
                            entities.delete(event.entity_b)
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}
