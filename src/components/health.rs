use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Clone, Debug)]
pub struct Health {
    pub current: u32,
    pub max: u32,
    pub dead: bool,
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
