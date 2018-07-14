use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component};

#[derive(Clone, Copy, Debug)]
pub struct Position(pub Vector2<f32>);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(cgmath::vec2(x, y))
    }
}

impl Deref for Position {
    type Target = Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Component for Position {
    type Storage = specs::VecStorage<Position>;
}
