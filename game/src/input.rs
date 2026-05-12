use macroquad::prelude::*;

#[derive(Default)]
pub struct InputFrame {
    pub direction: Vec2,
    pub confirm: bool,
    pub cat: bool,
    pub pwd: bool,
    pub mv_pick: bool,
    pub mv_drop: bool,
}

impl InputFrame {
    pub fn read() -> Self {
        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        Self {
            direction,
            confirm: is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space),
            cat: is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Space),
            pwd: is_key_pressed(KeyCode::P),
            mv_pick: is_key_pressed(KeyCode::M),
            mv_drop: is_key_pressed(KeyCode::V),
        }
    }
}
