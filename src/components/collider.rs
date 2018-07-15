use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Collider {
    pub radius: u32,
}

impl Collider {
    pub fn new(radius: u32) -> Self {
        Collider {
            radius,
        }
    }
}

impl Component for Collider {
    type Storage = specs::VecStorage<Collider>;
}

impl InitFromBlueprint for Collider {}
