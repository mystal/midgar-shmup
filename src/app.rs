use midgar::{self, KeyCode, Midgar};

use renderer::GameRenderer;
use world::GameWorld;

pub struct GameApp<'a, 'b, 'c> {
    world: GameWorld<'a, 'b>,
    renderer: GameRenderer<'c>,
}

impl<'a, 'b, 'c> midgar::App for GameApp<'a, 'b, 'c> {
    fn create(midgar: &Midgar) -> Self {
        let world = GameWorld::new(midgar);
        let renderer = GameRenderer::new(midgar);

        GameApp {
            world,
            renderer,
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time() as f32;

        self.world.update(midgar, dt);
        self.renderer.render(midgar, dt, &mut self.world);
    }
}
