use cgmath::{self, Vector2};
use specs::{self, Component};

use components::InitFromBlueprint;

fn default_position() -> Vector2<f32> {
    cgmath::vec2(0.0, 0.0)
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Transform {
    #[serde(default = "default_position")]
    pub position: Vector2<f32>,
    #[serde(default)]
    pub rotation: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, rotation: f32) -> Self {
        Transform {
            position: cgmath::vec2(x, y),
            rotation,
        }
    }
}

impl Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

impl InitFromBlueprint for Transform {}
