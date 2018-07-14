use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

#[derive(Clone, Debug)]
pub enum Renderable {
    Shape(Shape),
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub width: u32,
    pub height: u32,
    pub color: [f32; 3],
}

impl Renderable {
    pub fn new_shape(width: u32, height: u32, color: [f32; 3]) -> Self {
        Renderable::Shape(Shape {
            width: width,
            height: height,
            color: color,
        })
    }
}

impl Component for Renderable {
    type Storage = specs::VecStorage<Renderable>;
}
