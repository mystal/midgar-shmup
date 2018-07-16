use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Projectile {
    pub distance: Option<f32>,
}

impl Component for Projectile {
    type Storage = specs::HashMapStorage<Self>;
}

impl InitFromBlueprint for Projectile {}
