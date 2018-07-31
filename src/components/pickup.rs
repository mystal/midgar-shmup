use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Pickup {
    Bomb,
}

impl Component for Pickup {
    type Storage = specs::HashMapStorage<Pickup>;
}

impl InitFromBlueprint for Pickup {}
