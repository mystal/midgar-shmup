use specs::{self, Component};

use components::InitFromBlueprint;

#[derive(Clone, Debug, Deserialize)]
pub enum Renderable {
    Shape(Shape),
}

#[derive(Clone, Debug, Deserialize)]
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

impl InitFromBlueprint for Renderable {}
