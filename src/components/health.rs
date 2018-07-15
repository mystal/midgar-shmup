use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Health {
    #[serde(default)]
    pub current: u32,
    pub max: u32,
    #[serde(default)]
    pub dead: bool,
    #[serde(default)]
    pub invulnerable: bool,
}

impl Health {
    pub fn take_damage(&mut self, amount: u32) {
        if self.dead || self.invulnerable {
            return;
        }

        self.current = self.current.saturating_sub(amount);

        if self.current == 0 {
            self.dead = true;
        }
    }

    pub fn current_percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

impl Component for Health {
    type Storage = specs::VecStorage<Self>;
}

impl InitFromBlueprint for Health {
    fn init_from_blueprint(from: &Self) -> Self {
        let mut health = from.clone();
        if health.current == 0 {
            health.current = health.max;
        }
        health
    }
}
