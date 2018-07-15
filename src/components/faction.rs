use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Faction {
    Neutral,
    Player,
    Enemy,
}

impl Default for Faction {
    fn default() -> Self {
        Faction::Neutral
    }
}

impl Component for Faction {
    type Storage = specs::VecStorage<Self>;
}

impl InitFromBlueprint for Faction {}
