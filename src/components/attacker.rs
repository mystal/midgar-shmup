use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Attacker {
    pub damage: u32,
}

impl Component for Attacker {
    type Storage = specs::VecStorage<Self>;
}

impl InitFromBlueprint for Attacker {}
