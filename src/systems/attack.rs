use specs::{Entities, Read, ReadStorage, System, WriteStorage};

use components::*;
use systems::CollisionEvent;

pub struct AttackSystem;

impl AttackSystem {
    pub fn new() -> Self {
        AttackSystem
    }
}

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Vec<CollisionEvent>>,
        ReadStorage<'a, Attacker>,
        ReadStorage<'a, Faction>,
        WriteStorage<'a, Health>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, collisions, attackers, factions, mut healths) = data;

        for event in &*collisions {
            // TODO: Check for friendly fire.
            //let faction_a = factions.get(event.entity_a);
            //let faction_b = factions.get(event.entity_b);
            //let faction_check = faction_a != faction_b ||
            //    (faction_a.is_none() && faction_b.is_none());

            // Deal damage.
            if let (Some(attacker), Some(health)) = (attackers.get(event.entity_a), healths.get_mut(event.entity_b)) {
                let was_alive = !health.dead;
                health.take_damage(attacker.damage);
                println!("Ouch!");

                // TODO: Can we send an event on death? To perform certain logic?
                if was_alive && health.dead {
                    // Delete the attacked entity.
                    entities.delete(event.entity_b)
                        .unwrap();
                }
            }
        }
    }
}
