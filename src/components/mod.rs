use specs::{self, Builder};

pub use self::attacker::Attacker;
pub use self::bomber::{Bomber, BombState};
pub use self::camera::Camera;
pub use self::collider::Collider;
pub use self::faction::Faction;
pub use self::health::Health;
pub use self::pickup::Pickup;
pub use self::player::Player;
pub use self::projectile::Projectile;
pub use self::renderable::{Renderable, Shape};
pub use self::shooter::{FireState, Shooter};
pub use self::transform::Transform;
pub use self::velocity::Velocity;

pub mod attacker;
pub mod bomber;
pub mod camera;
pub mod collider;
pub mod faction;
pub mod health;
pub mod pickup;
pub mod player;
pub mod projectile;
pub mod renderable;
pub mod shooter;
pub mod transform;
pub mod velocity;

pub trait InitFromBlueprint: Clone {
    fn init_from_blueprint(from: &Self) -> Self {
        from.clone()
    }
}

macro_rules! blueprint {
    ( $( $name:ident: $typ:ty ),* , ) => {
        #[derive(Clone, Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct Blueprint {
            $(
                pub $name: Option<$typ>,
            )*
        }

        impl Blueprint {
            pub fn create_entity<'a, 'b>(&'a self, world: &'b mut specs::World) -> specs::EntityBuilder<'b> {
                self.init_components(world.create_entity())
            }

            fn init_components<'a, 'b>(&'a self, mut builder: specs::EntityBuilder<'b>) -> specs::EntityBuilder<'b> {
                $(
                    if let Some(ref $name) = self.$name {
                        builder = builder.with(InitFromBlueprint::init_from_blueprint($name));
                    }
                )*

                builder
            }
        }
    };
}

blueprint! {
    attacker: Attacker,
    bomber: Bomber,
    camera: Camera,
    collider: Collider,
    faction: Faction,
    health: Health,
    pickup: Pickup,
    player: Player,
    projectile: Projectile,
    renderable: Renderable,
    shooter: Shooter,
    transform: Transform,
    velocity: Velocity,
}
