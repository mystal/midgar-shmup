use std::rc::Rc;

use cgmath::{self, EuclideanSpace};
use midgar::{Midgar, Surface};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::texture::TextureRegion;
use specs::{Entities, Join, Read, ReadStorage};

use components::*;
use config;
use resources::PlayerScore;
use world::GameWorld;

type RenderData<'a> = (
    Read<'a, PlayerScore>,
    ReadStorage<'a, Bomber>,
    ReadStorage<'a, Camera>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, Renderable>,
    ReadStorage<'a, Transform>,
);

pub struct GameRenderer<'a> {
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
    text: TextRenderer<'a>,
    projection: cgmath::Matrix4<f32>,

    font: Font<'a>,
}

impl<'a> GameRenderer<'a> {
    pub fn new(midgar: &Midgar) -> Self {
        let viewport_width = config::GAME_SIZE.x as f32;
        let viewport_height = config::GAME_SIZE.y as f32;
        let projection = cgmath::ortho(-viewport_width / 2.0, viewport_width / 2.0,
                                       viewport_height / 2.0, -viewport_height / 2.0,
                                       -1.0, 1.0);

        GameRenderer {
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
            text: TextRenderer::new(midgar.graphics().display()),
            projection,

            font: text::load_font_from_path("assets/fonts/Kenney Pixel.ttf"),
        }
    }

    pub fn render(&mut self, midgar: &Midgar, _dt: f32, world: &mut GameWorld) {
        world.world.exec(|(score, bombers, cameras, players, renderables, transforms): RenderData| {
            let camera_pos = (&transforms, &cameras).join()
                .next()
                .expect("Lost the camera when trying to render!").0.position;

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
            for (renderable, transform) in (&renderables, &transforms).join() {
                match *renderable {
                    Renderable::Shape(ref shape) => {
                        let pos = transform.position;
                        // Round the positions so that shapes are drawn at pixel boundaries.
                        // Also draw with the origin in the center of the shape.
                        self.shape.draw_filled_rect(pos.x.round(), pos.y.round(),
                                                    shape.width as f32, shape.height as f32,
                                                    shape.color, &mut target);
                    },
                }
            }

            // Draw UI.
            let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                           config::SCREEN_SIZE.y as f32, 0.0,
                                           -1.0, 1.0);
            // Get the player entity's number of bombs.
            let bomb_count = if let Some((bomber, _)) = (&bombers, &players).join().next() {
                bomber.count
            } else {
                0
            };
            // Draw number of bombs in stock.
            let bomb_count_text = format!("Bomb: {:02}", bomb_count);
            self.text.draw_text(&bomb_count_text, self.font.clone(), [1.0, 1.0, 1.0],
                                40, 40.0, 30.0, 800, &projection, &mut target);

            // Draw player score.
            let player_score_text = format!("Score: {}", score.0);
            self.text.draw_text(&player_score_text, self.font.clone(), [1.0, 1.0, 1.0],
                                40, 800.0, 30.0, 800, &projection, &mut target);

            // Finish this frame.
            target.finish()
                .unwrap();
        });
    }
}
