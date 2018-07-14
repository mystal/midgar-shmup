use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Clone, Debug)]
pub struct Projectile {
    pub lifetime: Option<f64>,
}

impl Component for Projectile {
    type Storage = specs::HashMapStorage<Self>;
}
