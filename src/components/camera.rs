use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

pub struct Camera {
    pub follow_entity: Entity,
}

impl Component for Camera {
    type Storage = specs::HashMapStorage<Camera>;
}
