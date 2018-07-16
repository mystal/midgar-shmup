use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use blueprints::BlueprintManager;
use components::*;
use resources::{DeltaTime, SpawnQueue};

pub struct ShooterSystem {
}

impl ShooterSystem {
    pub fn new() -> Self {
        ShooterSystem {
        }
    }
}

impl<'a> System<'a> for ShooterSystem {
    type SystemData = (
        Read<'a, BlueprintManager>,
        Read<'a, DeltaTime>,
        Write<'a, SpawnQueue>,
        ReadStorage<'a, Faction>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Shooter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (blueprints, dt, mut spawns, factions, transforms, mut shooters) = data;

        for (faction, shooter, transform) in (&factions, &mut shooters, &transforms).join() {
            shooter.state = match shooter.state {
                FireState::Fire(dir) => {
                    // Spawn projectile.
                    let mut blueprint = blueprints.get(&shooter.projectile)
                        .expect("Could not find blueprint for shooter projectile")
                        .clone();
                    blueprint.transform = Some(*transform);
                    blueprint.velocity = Some((dir * shooter.velocity).into());
                    blueprint.faction = Some(*faction);
                    spawns.push(blueprint);

                    FireState::Cooldown(shooter.delay)
                }
                FireState::Cooldown(cooldown) if cooldown > 0.0 =>
                    FireState::Cooldown(cooldown - dt.0),
                _ => FireState::Idle
            };
        }
    }
}
