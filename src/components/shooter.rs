use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Clone, Debug)]
pub enum FireState {
    Idle,
    Fire(Vector2<f64>),
}

impl Default for FireState {
    fn default() -> Self {
        FireState::Idle
    }
}

#[derive(Clone, Debug)]
pub struct Shooter {
    pub projectile: String,
    pub delay: u32,
    pub cooldown: f64,
    pub state: FireState,
}

impl Component for Shooter {
    type Storage = specs::HashMapStorage<Self>;
}
