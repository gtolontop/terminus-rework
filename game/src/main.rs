mod assets;
mod game;
mod input;
mod menu;
mod pixel_art;
mod render;
mod state;
mod visual_style;
mod world;

use assets::GameAssets;
use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Terminus Rework".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = GameAssets::load().await;
    let mut game = Game::new(assets);

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
