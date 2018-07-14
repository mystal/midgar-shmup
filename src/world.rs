use cgmath::Vector2;
use cgmath::prelude::*;
use midgar::{KeyCode, Midgar, MouseButton};
use specs::{self, Builder};

use components::*;
use config;
use input::PlayerInput;
use systems::*;

pub struct GameWorld<'a, 'b> {
    pub world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameWorld<'a, 'b> {
    pub fn new(_midgar: &Midgar) -> Self {
        let mut world = specs::World::new();

        // Register all components before trying to use them.
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Collider>();
        world.register::<Renderable>();
        world.register::<Player>();
        world.register::<Camera>();
        //world.register::<Ai>();
        world.register::<Faction>();
        world.register::<Shooter>();
        world.register::<Projectile>();
        world.register::<Attacker>();
        world.register::<Health>();

        // Add the player entity.
        let player_entity = world.create_entity()
            .with(Position::new(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0))
            .with(Velocity::new(0.0, 0.0))
            .with(Collider::new(15))
            .with(Renderable::new_shape(30, 30, [0.2, 0.8, 0.0]))
            .with(Player {})
            .build();

        // Add the camera entity.
        let camera_entity = world.create_entity()
            .with(Position::new(50.0, 50.0))
            .with(Camera {
                follow_entity: player_entity,
            })
            .build();

        // Add an enemy that follows the player.
        // TODO: Remove this when we have an enemy spawning system in place.
        world.create_entity()
            .with(Position::new(100.0, 100.0))
            .with(Velocity::new(0.0, 0.0))
            .with(Collider::new(10))
            .with(Renderable::new_shape(20, 20, [0.8, 0.2, 0.0]))
            .build();

        world.add_resource(DeltaTime(0.0));
        world.add_resource(PlayerInput {
            move_dir: Vector2::zero(),
            fire_dir: Vector2::zero(),
            fire: false,
        });

        // Create a dispatcher to run systems.
        let dispatcher = specs::DispatcherBuilder::new()
            .with(PlayerSystem::new(), "player", &[])
            .with(PhysicsSystem::new(), "physics", &["player"])
            .with(CameraSystem::new(player_entity), "camera", &["physics"])
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
            let input = midgar.input();
            let mut player_input = self.world.write_resource::<PlayerInput>();
            player_input.move_dir.x = match
                (input.is_key_held(KeyCode::Left) || input.is_key_held(KeyCode::A),
                 input.is_key_held(KeyCode::Right) || input.is_key_held(KeyCode::D)) {
                (true, false) => -1.0,
                (false, true) => 1.0,
                _ => 0.0,
            };
            player_input.move_dir.y = match
                (input.is_key_held(KeyCode::Up) || input.is_key_held(KeyCode::W),
                 input.is_key_held(KeyCode::Down) || input.is_key_held(KeyCode::S)) {
                (true, false) => -1.0,
                (false, true) => 1.0,
                _ => 0.0,
            };
            if !player_input.move_dir.is_zero() {
                player_input.move_dir.normalize();
            }
            player_input.fire_dir = Vector2::zero();
            player_input.fire = midgar.input().is_button_held(MouseButton::Left);
        }
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
    }
}
