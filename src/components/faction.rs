use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
