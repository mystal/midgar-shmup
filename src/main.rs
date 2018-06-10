extern crate cgmath;
extern crate midgar;
extern crate specs;

mod app;
mod components;
mod config;
mod input;
mod renderer;
mod systems;
mod world;

fn main() {
    let app_config = midgar::MidgarAppConfig::new()
        .with_title("shmups")
        .with_screen_size((config::SCREEN_SIZE.x, config::SCREEN_SIZE.y));
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(app_config);
    app.run();
}
