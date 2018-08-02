use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use components::*;
use resources::DeltaTime;

pub struct BomberSystem {
}

impl BomberSystem {
    pub fn new() -> Self {
        BomberSystem {
        }
    }
}

impl<'a> System<'a> for BomberSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Faction>,
        WriteStorage<'a, Bomber>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, factions, mut bombers, entities) = data;

        for (faction, bomber) in (&factions, &mut bombers).join() {
            bomber.state = match bomber.state {
                BombState::Bomb => {
                    // Kill all enemies!
                    for (other_faction, entity) in (&factions, &*entities).join() {
                        if (faction != other_faction) || *faction == Faction::Neutral {
                            entities.delete(entity)
                                .unwrap();
                        }
                    }

                    BombState::Cooldown(bomber.delay)
                }
                BombState::Cooldown(cooldown) if cooldown > 0.0 =>
                    BombState::Cooldown(cooldown - dt.0),
                _ => BombState::Idle
            };
        }
    }
}
