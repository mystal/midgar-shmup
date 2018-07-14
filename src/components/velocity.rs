use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component};

#[derive(Clone, Copy, Debug)]
pub struct Velocity(pub cgmath::Vector2<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(cgmath::vec2(x, y))
    }
}

impl Component for Velocity {
    type Storage = specs::VecStorage<Velocity>;
}

impl Deref for Velocity {
    type Target = Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
