use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Collider {
    pub radius: f32,
    #[serde(default)]
    pub die: bool,
}

impl Collider {
    pub fn new(radius: f32, die: bool) -> Self {
        Collider {
            radius,
            die,
        }
    }
}

impl Component for Collider {
    type Storage = specs::VecStorage<Collider>;
}

impl InitFromBlueprint for Collider {}
