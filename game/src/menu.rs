use macroquad::prelude::*;

const BG: Color = Color::new(0.027, 0.031, 0.035, 1.0);
const INK: Color = Color::new(0.89, 0.93, 0.89, 1.0);
const MUTED: Color = Color::new(0.42, 0.48, 0.45, 1.0);
const GREEN: Color = Color::new(0.0, 0.82, 0.36, 1.0);

pub fn draw_menu() {
    let time = get_time() as f32;
    clear_background(BG);

    draw_title();
    draw_command(time);
    draw_prompt(time);
}

fn draw_title() {
    let center_x = screen_width() / 2.0;
    let title = "TERMINUS";
    let title_size = 66;
    let title_width = measure_text(title, None, title_size, 1.0).width;
    let title_x = center_x - title_width / 2.0;
    let title_y = screen_height() * 0.39;

    draw_text(title, title_x, title_y, title_size as f32, INK);

    let subtitle = "rework";
    let subtitle_size = 34;
    let subtitle_width = measure_text(subtitle, None, subtitle_size, 1.0).width;
    draw_text(
        subtitle,
        center_x - subtitle_width / 2.0,
        title_y + 46.0,
        subtitle_size as f32,
        GREEN,
    );
}

fn draw_command(time: f32) {
    let center_x = screen_width() / 2.0;
    let y = screen_height() * 0.58;
    let command = "$ cargo run terminus";
    let size = 23;
    let width = measure_text(command, None, size, 1.0).width;

    draw_text(command, center_x - width / 2.0, y, size as f32, MUTED);

    if (time * 2.6).sin() > -0.2 {
        draw_rectangle(center_x + width / 2.0 + 8.0, y - 18.0, 9.0, 21.0, GREEN);
    }
}

fn draw_prompt(time: f32) {
    let label = "Entree / Espace";
    let size = 24;
    let pulse = 0.55 + (time * 2.0).sin().abs() * 0.28;
    let color = Color::new(INK.r, INK.g, INK.b, pulse);
    let text = format!("[ {label} ]");
    let width = measure_text(&text, None, size, 1.0).width;

    draw_text(
        &text,
        screen_width() / 2.0 - width / 2.0,
        screen_height() * 0.72,
        size as f32,
        color,
    );
}
