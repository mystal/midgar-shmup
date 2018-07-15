use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component};

use components::InitFromBlueprint;

fn default_velocity() -> Vector2<f32> {
    cgmath::vec2(0.0, 0.0)
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Velocity {
    #[serde(default = "default_velocity")]
    inner: cgmath::Vector2<f32>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            inner: cgmath::vec2(x, y),
        }
    }
}

impl Component for Velocity {
    type Storage = specs::VecStorage<Velocity>;
}

impl Deref for Velocity {
    type Target = Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl InitFromBlueprint for Velocity {}
