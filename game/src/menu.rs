use macroquad::prelude::*;

const BG: Color = Color::new(0.027, 0.031, 0.035, 1.0);
const INK: Color = Color::new(0.89, 0.93, 0.89, 1.0);
const MUTED: Color = Color::new(0.42, 0.48, 0.45, 1.0);
const GREEN: Color = Color::new(0.0, 0.82, 0.36, 1.0);
const GREEN_DIM: Color = Color::new(0.0, 0.35, 0.19, 1.0);

pub fn draw_menu() {
    let time = get_time() as f32;
    clear_background(BG);

    draw_quiet_background();
    draw_title(time);
    draw_command(time);
    draw_prompt(time);
    draw_butterfly(time);
}

fn draw_quiet_background() {
    let width = screen_width();
    let height = screen_height();
    let center_y = height * 0.52;

    draw_line(
        width * 0.22,
        center_y,
        width * 0.78,
        center_y,
        1.0,
        Color::from_rgba(0, 210, 95, 42),
    );

    for index in 0..7 {
        let alpha = 11 + index * 3;
        let y = center_y + 34.0 + index as f32 * 18.0;
        draw_line(
            width * 0.28,
            y,
            width * 0.72,
            y,
            1.0,
            Color::from_rgba(220, 236, 220, alpha as u8),
        );
    }
}

fn draw_title(time: f32) {
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

    let breathe = 0.72 + (time * 1.4).sin() * 0.08;
    draw_rectangle(
        center_x - 74.0,
        title_y + 67.0,
        148.0,
        2.0,
        Color::new(GREEN.r, GREEN.g, GREEN.b, breathe),
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

fn draw_butterfly(time: f32) {
    let center = vec2(screen_width() / 2.0 + 172.0, screen_height() * 0.38 - 18.0);
    let flap = (time * 5.0).sin().abs();
    let wing = 7.0 + flap * 4.0;
    let lift = (time * 1.7).sin() * 3.0;
    let body = vec2(center.x, center.y + lift);

    draw_line(
        body.x,
        body.y - 6.0,
        body.x,
        body.y + 8.0,
        1.0,
        Color::from_rgba(190, 226, 202, 180),
    );

    draw_triangle(
        vec2(body.x - 2.0, body.y - 1.0),
        vec2(body.x - wing, body.y - 9.0),
        vec2(body.x - 7.0, body.y + 5.0),
        Color::from_rgba(0, 210, 102, 150),
    );
    draw_triangle(
        vec2(body.x + 2.0, body.y - 1.0),
        vec2(body.x + wing, body.y - 9.0),
        vec2(body.x + 7.0, body.y + 5.0),
        Color::from_rgba(0, 210, 102, 150),
    );
    draw_circle(body.x, body.y + 1.0, 2.0, GREEN_DIM);
}
