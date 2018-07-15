use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Player {
}

impl Component for Player {
    type Storage = specs::HashMapStorage<Player>;
}

impl InitFromBlueprint for Player {}
