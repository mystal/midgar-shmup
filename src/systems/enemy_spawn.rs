use rand::{thread_rng, Rng};
use specs::{Read, System, Write};

use blueprints::BlueprintManager;
use components::*;
use config;
use resources::*;

pub struct EnemySpawnSystem {
    timer: f32,
    spawn_time: f32,
}

impl EnemySpawnSystem {
    pub fn new(spawn_time: f32) -> Self {
        Self {
            timer: spawn_time,
            spawn_time,
        }
    }
}

impl<'a> System<'a> for EnemySpawnSystem {
    type SystemData = (
        Read<'a, BlueprintManager>,
        Read<'a, DeltaTime>,
        Write<'a, SpawnQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (blueprints, dt, mut spawns) = data;

        if self.timer > 0.0 {
            self.timer -= dt.0;
        } else {
            let mut rng = thread_rng();

            // TODO: Spawn a wave of enemies.
            let mut blueprint = blueprints.get("Enemy")
                .expect("Could not find Enemy blueprint")
                .clone();
            blueprint.transform = Some(Transform::new(rng.gen_range(10.0, config::GAME_SIZE.x as f32 - 10.0),
                                                      rng.gen_range(30.0, 200.0), 0.0));
            blueprint.velocity = Some(Velocity::new(0.0, 100.0));
            spawns.push(blueprint);

            self.timer = self.spawn_time;
        }
    }
}
