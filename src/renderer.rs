use std::rc::Rc;

use cgmath::{self, EuclideanSpace};
use midgar::{Midgar, Surface};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::TextureRegion;
use specs::{Join, ReadStorage};

use components::{Camera, Position, Renderable};
use config;
use world::GameWorld;

pub struct GameRenderer {
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
    projection: cgmath::Matrix4<f32>,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        let viewport_width = config::GAME_SIZE.x as f32;
        let viewport_height = config::GAME_SIZE.y as f32;
        let projection = cgmath::ortho(-viewport_width / 2.0, viewport_width / 2.0,
                                       viewport_height / 2.0, -viewport_height / 2.0,
                                       -1.0, 1.0);

        GameRenderer {
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
            projection,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, _dt: f32, world: &mut GameWorld) {
        world.world.exec(|(renderables, positions, cameras): (ReadStorage<Renderable>, ReadStorage<Position>, ReadStorage<Camera>)| {
            let camera_pos = (&positions, &cameras).join()
                .next()
                .expect("Lost the camera when trying to render!").0;

            // Compute the combined projection * view matrix.
            let camera_pos = cgmath::vec3(camera_pos.x.round(), camera_pos.y.round(), 0.0);
            let view = cgmath::Matrix4::look_at(cgmath::Point3::from_vec(camera_pos),
                                                cgmath::Point3::new(0.0, 0.0, -1.0) + camera_pos,
                                                cgmath::vec3(0.0, 1.0, 0.0));
            let combined = self.projection * view;
            self.sprite.set_projection_matrix(combined);
            self.shape.set_projection_matrix(combined);

            let mut target = midgar.graphics().display().draw();

            // Clear the screen.
            let color = [0, 0, 0];
            target.clear_color(color[0] as f32 / 0.0, color[1] as f32 / 0.0, color[2] as f32 / 0.0, 1.0);

            // Draw each renderable.
            for (renderable, pos) in (&renderables, &positions).join() {
                match *renderable {
                    Renderable::Shape(ref shape) => {
                        // Round the positions so that shapes are drawn at pixel boundaries.
                        // Also draw with the origin in the center of the shape.
                        self.shape.draw_filled_rect(pos.0.x.round(), pos.0.y.round(),
                                                    shape.width as f32, shape.height as f32,
                                                    shape.color, &mut target);
                    },
                }
            }

            // Finish this frame.
            target.finish()
                .unwrap();
        });
    }
}
