use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Clone, Debug)]
pub struct Attacker {
    pub damage: u32,
}

impl Component for Attacker {
    type Storage = specs::VecStorage<Self>;
}
