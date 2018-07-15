use specs::{self, Component, Entity};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub struct Camera {
    //pub follow_entity: Entity,
}

impl Component for Camera {
    type Storage = specs::HashMapStorage<Camera>;
}

impl InitFromBlueprint for Camera {}
