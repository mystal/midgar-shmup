use midgar::Midgar;
use specs::{self, Builder};

use blueprints::BlueprintManager;
use components::*;
use config;
use input::{check_input, FireInput, PlayerInput};
use resources::*;
use systems::*;

pub struct GameWorld<'a, 'b> {
    pub world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameWorld<'a, 'b> {
    pub fn new(_midgar: &Midgar) -> Self {
        // Load blueprints data.
        let blueprints = BlueprintManager::load("data/blueprints.json")
            .expect("Failed to load blueprints");

        let mut world = specs::World::new();

        // Register all components before trying to use them.
        world.register::<Attacker>();
        world.register::<Bomber>();
        world.register::<Camera>();
        world.register::<Collider>();
        world.register::<Faction>();
        world.register::<Health>();
        world.register::<Pickup>();
        world.register::<Player>();
        world.register::<Projectile>();
        world.register::<Renderable>();
        world.register::<Shooter>();
        world.register::<Transform>();
        world.register::<Velocity>();

        // Add the player entity.
        let player_entity = {
            let blueprint = blueprints.get("Player")
                .expect("Could not find Player blueprint");
            blueprint.create_entity(&mut world)
                .with(Transform::new(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0, 0.0))
                .build()
        };

        // Add the camera entity.
        let camera_entity = {
            let blueprint = blueprints.get("WorldCamera")
                .expect("Could not find WorldCamera blueprint");
            blueprint.create_entity(&mut world)
                .with(Transform::new(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0, 0.0))
                .build()
        };

        // Add an enemy that follows the player.
        // TODO: Remove this when we have an enemy spawning system in place.
        let enemy_entity = {
            let blueprint = blueprints.get("Enemy")
                .expect("Could not find Enemy blueprint");
            blueprint.create_entity(&mut world)
                .with(Transform::new(100.0, 100.0, 0.0))
                .build()
        };

        // Add a bomb pickup.
        // TODO: Remove this when we have a pickup spawning system in place.
        let bomb_entity = {
            let blueprint = blueprints.get("BombPickup")
                .expect("Could not find BombPickup blueprint");
            blueprint.create_entity(&mut world)
                .with(Transform::new(200.0, 200.0, 0.0))
                .build()
        };

        world.add_resource(blueprints);
        world.add_resource(DeltaTime::default());
        world.add_resource(PlayerInput::default());
        world.add_resource(PlayerScore::default());
        world.add_resource(SpawnQueue::default());
        world.add_resource::<Vec<CollisionEvent>>(Vec::default());

        // Create a dispatcher to run systems.
        let dispatcher = specs::DispatcherBuilder::new()
            .with(PlayerSystem::new(), "player", &[])
            .with(BomberSystem::new(), "bomber", &["player"])
            .with(ShooterSystem::new(), "shooter", &["player"])
            .with(MotionSystem::new(), "motion", &["bomber", "shooter"])
            .with(CollisionSystem::new(), "collision", &["motion"])
            .with(AttackSystem::new(), "attack", &["collision"])
            .with(PickupSystem::new(), "pickup", &["collision"])
            .with(PickupSpawnSystem::new(2.0), "pickup_spawn", &["attack", "pickup"])
            //.with(CameraSystem::new(player_entity), "camera", &["attack"])
            .build();

        GameWorld {
            world,
            dispatcher,
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            delta.0 = dt;
        }
        {
            // Check player input.
            let mut player_input = self.world.write_resource::<PlayerInput>();
            *player_input = check_input(midgar.input());
        }
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        // Spawn new entities.
        let spawns = {
            // Clone and clear queued spawns.
            let mut queued_spawns = self.world.write_resource::<SpawnQueue>();
            // FIXME: Can we avoid this clone?
            let cloned_spawns = queued_spawns.0.clone();
            queued_spawns.0.clear();
            cloned_spawns
        };
        // Spawn new entities.
        for blueprint in spawns {
            blueprint.create_entity(&mut self.world)
                .build();
        }
    }
}
