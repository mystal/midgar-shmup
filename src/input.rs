use cgmath::{Vector2, Zero};

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub move_dir: Vector2<f32>,
    pub fire_dir: Vector2<f32>,
    pub fire: bool,
}

impl Default for PlayerInput {
    fn default() -> Self {
        PlayerInput {
            move_dir: Vector2::zero(),
            fire_dir: Vector2::zero(),
            fire: false,
        }
    }
}
