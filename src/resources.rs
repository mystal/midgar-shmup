use std::ops::{Deref, DerefMut};

use components::Blueprint;

#[derive(Clone, Debug, Default)]
pub struct DeltaTime(pub f32);

#[derive(Debug, Default)]
pub struct SpawnQueue(pub Vec<Blueprint>);

impl Deref for SpawnQueue {
    type Target = Vec<Blueprint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpawnQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
