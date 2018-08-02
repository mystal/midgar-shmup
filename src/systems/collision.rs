use cgmath::MetricSpace;
use specs::{Entities, Entity, Join, ReadStorage, System, Write};

use components::*;

pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        CollisionSystem
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, Vec<CollisionEvent>>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Faction>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut collisions, colliders, factions, transforms) = data;

        collisions.clear();

        // FIXME: This is ugly, but it works. Clean it up someday.
        for (i, entity) in entities.join().enumerate() {
            if let (Some(collider), Some(transform)) = (colliders.get(entity), transforms.get(entity)) {
                let faction = factions.get(entity);

                for other_entity in entities.join().skip(i + 1) {
                    if let (Some(other_collider), Some(other_transform)) = (colliders.get(other_entity), transforms.get(other_entity)) {
                        let other_faction = factions.get(other_entity);

                        let overlap = transform.position.distance(other_transform.position) <
                            (collider.radius + other_collider.radius);
                        let faction_check = faction != other_faction ||
                            (faction.is_none() && other_faction.is_none()) /*||
                            (collider.collide_with_faction && other_collider.collide_with_faction)*/;

                        if overlap && faction_check {
                            //println!("Collision: {:?}, {:?}", entity, other_entity);

                            // Store collision events.
                            // NOTE: We always send two events right now, but eventually it may
                            // be that only one entity hits the other due to ignore flags.
                            collisions.push(CollisionEvent {
                                entity_a: entity,
                                entity_b: other_entity,
                            });
                            collisions.push(CollisionEvent {
                                entity_a: other_entity,
                                entity_b: entity,
                            });

                            // Kill entities that die on collision.
                            if collider.die {
                                entities.delete(entity)
                                    .unwrap();
                            }
                            if other_collider.die {
                                entities.delete(other_entity)
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
