use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

use components::*;
use resources::*;

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
        Write<'a, PlayerScore>,
        ReadStorage<'a, Faction>,
        WriteStorage<'a, Bomber>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut score, factions, mut bombers, entities) = data;

        for (faction, bomber) in (&factions, &mut bombers).join() {
            bomber.state = match bomber.state {
                BombState::Bomb => {
                    // Kill all enemies!
                    for (other_faction, entity) in (&factions, &*entities).join() {
                        if (faction != other_faction) || *faction == Faction::Neutral {
                            // TODO: Move this somewhere more central, in case things die for other
                            // reasons.
                            if *faction == Faction::Player {
                                score.0 += 10;
                            }

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
