use macroquad::prelude::*;

pub const VIRTUAL_PLAY: Rect = Rect::new(40.0, 90.0, 1200.0, 560.0);

pub fn play_rect() -> Rect {
    let margin = (screen_width() * 0.032).clamp(28.0, 56.0);
    let top = (screen_height() * 0.16).clamp(96.0, 128.0);
    let hud_height = 62.0;
    let gap = 10.0;
    let bottom = 10.0;
    let height = (screen_height() - top - hud_height - gap - bottom).max(300.0);

    Rect::new(margin, top, screen_width() - margin * 2.0, height)
}

pub fn hud_rect() -> Rect {
    let play = play_rect();
    Rect::new(play.x, play.y + play.h + 10.0, play.w, 62.0)
}

pub fn world_to_screen(point: Vec2) -> Vec2 {
    let play = play_rect();
    vec2(
        play.x + (point.x - VIRTUAL_PLAY.x) / VIRTUAL_PLAY.w * play.w,
        play.y + (point.y - VIRTUAL_PLAY.y) / VIRTUAL_PLAY.h * play.h,
    )
}

pub fn rect_to_screen(rect: Rect) -> Rect {
    let top_left = world_to_screen(vec2(rect.x, rect.y));
    let bottom_right = world_to_screen(vec2(rect.x + rect.w, rect.y + rect.h));
    Rect::new(
        top_left.x,
        top_left.y,
        bottom_right.x - top_left.x,
        bottom_right.y - top_left.y,
    )
}

pub fn world_scale() -> f32 {
    let play = play_rect();
    (play.w / VIRTUAL_PLAY.w).min(play.h / VIRTUAL_PLAY.h)
}

pub fn scale_vec(size: Vec2) -> Vec2 {
    let scale = world_scale();
    vec2(size.x * scale, size.y * scale)
}
