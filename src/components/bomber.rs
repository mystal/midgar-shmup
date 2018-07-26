use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub enum BombState {
    Idle,
    Bomb,
    Cooldown(f32)
}

impl Default for BombState {
    fn default() -> Self {
        BombState::Idle
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bomber {
    #[serde(default)]
    pub count: u8,
    pub delay: f32,
    #[serde(default)]
    pub state: BombState,
}

impl Bomber {
    pub fn try_bombing(&mut self) -> bool {
        if let BombState::Idle = self.state {
            if self.count > 0 {
                self.count -= 1;
                self.state = BombState::Bomb;
                return true;
            }
        }
        false
    }
}

impl Component for Bomber {
    type Storage = specs::HashMapStorage<Bomber>;
}

impl InitFromBlueprint for Bomber {}
