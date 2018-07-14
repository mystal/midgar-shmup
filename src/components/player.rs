use std::ops::{Deref, DerefMut};

use cgmath::{self, Vector2};
use specs::{self, Component, Entity};

pub struct Player {
}

impl Component for Player {
    type Storage = specs::HashMapStorage<Player>;
}
