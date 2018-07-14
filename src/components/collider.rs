use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Clone, Debug)]
pub struct Collider {
    pub radius: u32,
}

impl Collider {
    pub fn new(radius: u32) -> Self {
        Collider {
            radius,
        }
    }
}

impl Component for Collider {
    type Storage = specs::VecStorage<Collider>;
}
