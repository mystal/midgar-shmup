use cgmath::{Vector2, Zero};

#[derive(Debug, Clone)]
pub enum FireInput {
    Idle,
    Fire(Vector2<f32>),
}

#[derive(Clone, Debug)]
pub struct PlayerInput {
    pub move_dir: Vector2<f32>,
    pub fire: FireInput,
}

impl Default for PlayerInput {
    fn default() -> Self {
        PlayerInput {
            move_dir: Vector2::zero(),
            fire: FireInput::Idle,
        }
    }
}
