use cgmath::{self, InnerSpace, Vector2, Zero};
use midgar::{Button, KeyCode, MouseButton};
use midgar::Input;

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

pub fn check_input(input: &Input) -> PlayerInput {
    let controller = input.controllers().first();
    let x = match
        (input.is_key_held(KeyCode::Left) || input.is_key_held(KeyCode::A) || controller.map(|c| c.is_button_held(Button::DPadLeft)).unwrap_or(false),
         input.is_key_held(KeyCode::Right) || input.is_key_held(KeyCode::D) || controller.map(|c| c.is_button_held(Button::DPadRight)).unwrap_or(false)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };
    let y = match
        (input.is_key_held(KeyCode::Up) || input.is_key_held(KeyCode::W) || controller.map(|c| c.is_button_held(Button::DPadUp)).unwrap_or(false),
         input.is_key_held(KeyCode::Down) || input.is_key_held(KeyCode::S) || controller.map(|c| c.is_button_held(Button::DPadDown)).unwrap_or(false)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };
    let move_dir = cgmath::vec2(x, y);
    let fire = if input.is_key_held(KeyCode::Space) || controller.map(|c| c.is_button_held(Button::RightShoulder)).unwrap_or(false) {
        FireInput::Fire(cgmath::vec2(0.0, -1.0))
    } else {
        FireInput::Idle
    };

    PlayerInput {
        move_dir: if !move_dir.is_zero() {move_dir.normalize()} else {Vector2::zero()},
        fire,
    }
}
