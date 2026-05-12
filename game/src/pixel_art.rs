use macroquad::prelude::*;

use crate::state::Facing;

const SKIN: Color = Color::new(0.82, 0.58, 0.38, 1.0);
const SKIN_LIGHT: Color = Color::new(0.96, 0.72, 0.48, 1.0);
const HAIR: Color = Color::new(0.12, 0.08, 0.07, 1.0);
const SHIRT: Color = Color::new(0.12, 0.36, 0.82, 1.0);
const SHIRT_DARK: Color = Color::new(0.06, 0.18, 0.48, 1.0);
const PANTS: Color = Color::new(0.08, 0.11, 0.18, 1.0);
const BOOTS: Color = Color::new(0.05, 0.04, 0.04, 1.0);
const OUTLINE: Color = Color::new(0.03, 0.035, 0.04, 1.0);
const EYE: Color = Color::new(0.02, 0.02, 0.02, 1.0);

pub fn draw_player_sprite(center: Vec2, facing: Facing) {
    draw_ellipse(
        center.x,
        center.y + 27.0,
        24.0,
        8.0,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.22),
    );

    let origin = center - vec2(24.0, 42.0);
    let scale = 3.0;

    match facing {
        Facing::Down => draw_front(origin, scale),
        Facing::Up => draw_back(origin, scale),
        Facing::Left => draw_side(origin, scale, true),
        Facing::Right => draw_side(origin, scale, false),
    }
}

fn draw_front(origin: Vec2, scale: f32) {
    rect(origin, scale, 5, 0, 6, 2, HAIR);
    rect(origin, scale, 4, 2, 8, 5, SKIN);
    rect(origin, scale, 3, 3, 1, 3, HAIR);
    rect(origin, scale, 12, 3, 1, 3, HAIR);
    rect(origin, scale, 6, 4, 1, 1, EYE);
    rect(origin, scale, 9, 4, 1, 1, EYE);
    rect(origin, scale, 6, 7, 4, 1, SKIN_LIGHT);

    rect(origin, scale, 3, 8, 10, 7, SHIRT);
    rect(origin, scale, 2, 9, 2, 5, SKIN);
    rect(origin, scale, 12, 9, 2, 5, SKIN);
    rect(origin, scale, 5, 9, 6, 1, SHIRT_DARK);
    rect(origin, scale, 5, 15, 2, 5, PANTS);
    rect(origin, scale, 9, 15, 2, 5, PANTS);
    rect(origin, scale, 4, 20, 3, 2, BOOTS);
    rect(origin, scale, 9, 20, 3, 2, BOOTS);

    outline(origin, scale);
}

fn draw_back(origin: Vec2, scale: f32) {
    rect(origin, scale, 4, 0, 8, 6, HAIR);
    rect(origin, scale, 4, 5, 8, 2, SKIN);
    rect(origin, scale, 3, 8, 10, 7, SHIRT);
    rect(origin, scale, 2, 9, 2, 5, SKIN);
    rect(origin, scale, 12, 9, 2, 5, SKIN);
    rect(origin, scale, 5, 9, 6, 2, SHIRT_DARK);
    rect(origin, scale, 5, 15, 2, 5, PANTS);
    rect(origin, scale, 9, 15, 2, 5, PANTS);
    rect(origin, scale, 4, 20, 3, 2, BOOTS);
    rect(origin, scale, 9, 20, 3, 2, BOOTS);

    outline(origin, scale);
}

fn draw_side(origin: Vec2, scale: f32, flip: bool) {
    let x = |value| if flip { 15 - value } else { value };

    rect(origin, scale, x(5), 0, 6, 2, HAIR);
    rect(origin, scale, x(4), 2, 8, 5, SKIN);
    rect(origin, scale, x(4), 3, 2, 3, HAIR);
    rect(origin, scale, x(10), 4, 1, 1, EYE);
    rect(origin, scale, x(10), 6, 2, 1, SKIN_LIGHT);
    rect(origin, scale, x(3), 8, 10, 7, SHIRT);
    rect(origin, scale, x(11), 9, 2, 5, SKIN);
    rect(origin, scale, x(5), 9, 5, 1, SHIRT_DARK);
    rect(origin, scale, x(5), 15, 2, 5, PANTS);
    rect(origin, scale, x(9), 15, 2, 5, PANTS);
    rect(origin, scale, x(4), 20, 3, 2, BOOTS);
    rect(origin, scale, x(9), 20, 3, 2, BOOTS);

    outline(origin, scale);
}

fn outline(origin: Vec2, scale: f32) {
    rect(origin, scale, 4, -1, 8, 1, OUTLINE);
    rect(origin, scale, 3, 1, 1, 6, OUTLINE);
    rect(origin, scale, 12, 1, 1, 6, OUTLINE);
    rect(origin, scale, 2, 8, 1, 7, OUTLINE);
    rect(origin, scale, 13, 8, 1, 7, OUTLINE);
    rect(origin, scale, 4, 22, 8, 1, OUTLINE);
}

fn rect(origin: Vec2, scale: f32, x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw_rectangle(
        origin.x + x as f32 * scale,
        origin.y + y as f32 * scale,
        w as f32 * scale,
        h as f32 * scale,
        color,
    );
}
