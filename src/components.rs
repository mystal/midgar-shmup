use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

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

pub struct Player {
}

impl Component for Player {
    type Storage = specs::HashMapStorage<Player>;
}

pub struct Camera {
    pub follow_entity: Entity,
}

impl Component for Camera {
    type Storage = specs::HashMapStorage<Camera>;
}
