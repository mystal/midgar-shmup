use cgmath::Vector2;
use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub enum FireState {
    Idle,
    Fire(Vector2<f32>),
    Cooldown(f32)
}

impl Default for FireState {
    fn default() -> Self {
        FireState::Idle
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Shooter {
    pub projectile: String,
    pub velocity: f32,
    pub delay: f32,
    #[serde(default)]
    pub state: FireState,
}

impl Shooter {
    pub fn try_firing(&mut self, dir: Vector2<f32>) -> bool {
        if let FireState::Idle = self.state {
            self.state = FireState::Fire(dir);
            return true;
        }
        false
    }
}

impl Component for Shooter {
    type Storage = specs::HashMapStorage<Self>;
}

impl InitFromBlueprint for Shooter {}
