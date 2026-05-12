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
    loop {
        clear_background(Color::from_rgba(12, 14, 18, 255));
        draw_text("Terminus Rework", 40.0, 72.0, 42.0, WHITE);
        draw_text("Rust prototype boot OK", 42.0, 116.0, 24.0, GRAY);
        next_frame().await;
    }
}
