use macroquad::prelude::*;

const GREEN: Color = Color::new(0.0, 0.86, 0.32, 1.0);
const DIM_GREEN: Color = Color::new(0.0, 0.42, 0.22, 1.0);
const PAPER: Color = Color::new(0.91, 0.96, 0.91, 1.0);
const PANEL: Color = Color::new(0.015, 0.018, 0.02, 1.0);

pub fn draw_menu() {
    let time = get_time() as f32;
    clear_background(Color::from_rgba(8, 10, 13, 255));
    draw_scanlines(time);
    draw_title_card(time);
    draw_start_preview(time);
    draw_prompt(time);
}

fn draw_scanlines(time: f32) {
    let w = screen_width();
    let h = screen_height();
    let sweep = (time * 70.0) % h;

    for row in (0..h as i32).step_by(6) {
        draw_rectangle(0.0, row as f32, w, 2.0, Color::from_rgba(255, 255, 255, 9));
    }

    draw_rectangle(0.0, sweep, w, 24.0, Color::from_rgba(0, 220, 90, 18));
}

fn draw_title_card(time: f32) {
    let card = menu_card_rect();
    draw_pixel_frame(card, time);
    draw_terminal_machine(card, time);
    draw_logo(card, time);
}

fn menu_card_rect() -> Rect {
    let card_w = (screen_width() * 0.58).clamp(640.0, 760.0);
    Rect::new(screen_width() / 2.0 - card_w / 2.0, 92.0, card_w, 370.0)
}

fn draw_pixel_frame(rect: Rect, time: f32) {
    draw_rectangle(
        rect.x + 10.0,
        rect.y + 12.0,
        rect.w,
        rect.h,
        Color::from_rgba(0, 0, 0, 120),
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, PANEL);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 5.0, PAPER);
    draw_rectangle_lines(
        rect.x + 12.0,
        rect.y + 12.0,
        rect.w - 24.0,
        rect.h - 24.0,
        2.0,
        DIM_GREEN,
    );

    for (x, y) in [
        (rect.x + 12.0, rect.y + 12.0),
        (rect.x + rect.w - 38.0, rect.y + 12.0),
        (rect.x + 12.0, rect.y + rect.h - 38.0),
        (rect.x + rect.w - 38.0, rect.y + rect.h - 38.0),
    ] {
        draw_rectangle(x, y, 26.0, 26.0, PAPER);
        draw_rectangle(x + 7.0, y + 7.0, 12.0, 12.0, PANEL);
    }

    let blink = if (time * 2.0).sin() > 0.0 {
        GREEN
    } else {
        DIM_GREEN
    };
    draw_rectangle(rect.x + rect.w - 88.0, rect.y + 92.0, 34.0, 16.0, blink);
    draw_rectangle_lines(
        rect.x + rect.w - 96.0,
        rect.y + 84.0,
        50.0,
        32.0,
        3.0,
        PAPER,
    );
}

fn draw_terminal_machine(rect: Rect, time: f32) {
    let ox = rect.x + 230.0;
    let oy = rect.y + 58.0;

    draw_rectangle(ox, oy, 240.0, 116.0, PAPER);
    draw_rectangle(ox + 14.0, oy + 14.0, 212.0, 80.0, PANEL);
    draw_rectangle_lines(ox + 28.0, oy + 24.0, 184.0, 58.0, 3.0, PAPER);

    let cursor_on = (time * 3.5).sin() > -0.15;
    draw_text(">", ox + 48.0, oy + 62.0, 34.0, GREEN);
    draw_rectangle(ox + 78.0, oy + 48.0, 48.0, 8.0, GREEN);
    if cursor_on {
        draw_rectangle(ox + 134.0, oy + 45.0, 10.0, 18.0, GREEN);
    }

    draw_keyboard(ox + 46.0, oy + 105.0, time);
    draw_cable(rect, time);
}

fn draw_keyboard(x: f32, y: f32, time: f32) {
    draw_rectangle(x, y, 180.0, 44.0, PAPER);
    draw_rectangle(x + 10.0, y + 8.0, 160.0, 28.0, PANEL);

    for row in 0..3 {
        for col in 0..12 {
            let glow = ((time * 4.0 + row as f32 * 0.7 + col as f32 * 0.21).sin() + 1.0) * 0.5;
            let color = Color::new(0.8 * glow, 0.95 * glow, 0.82 * glow, 1.0);
            draw_rectangle(
                x + 17.0 + col as f32 * 12.0,
                y + 12.0 + row as f32 * 8.0,
                7.0,
                4.0,
                color,
            );
        }
    }
}

fn draw_cable(rect: Rect, time: f32) {
    let y = rect.y + 178.0 + (time * 2.4).sin() * 3.0;
    draw_line(rect.x + 72.0, y, rect.x + 226.0, y, 5.0, PAPER);
    draw_line(rect.x + 70.0, y, rect.x + 226.0, y + 12.0, 2.0, GREEN);
    draw_line(rect.x + 470.0, y, rect.x + 642.0, y, 5.0, PAPER);
    draw_line(rect.x + 470.0, y + 12.0, rect.x + 642.0, y, 2.0, GREEN);
}

fn draw_logo(rect: Rect, time: f32) {
    let title_x = rect.x + 82.0;
    let title_y = rect.y + 224.0;
    let glitch = if (time * 6.0).sin() > 0.82 { 4.0 } else { 0.0 };

    draw_text("TERMINUS", title_x + glitch, title_y, 58.0, PAPER);
    draw_text(
        "TERMINUS",
        title_x - 2.0,
        title_y + 2.0,
        58.0,
        Color::from_rgba(0, 180, 90, 95),
    );
    draw_text("REWORK", title_x + 122.0, title_y + 62.0, 66.0, GREEN);

    let underline = 260.0 + (time * 3.0).sin() * 38.0;
    draw_rectangle(title_x + 126.0, title_y + 74.0, underline, 6.0, DIM_GREEN);
}

fn draw_prompt(time: f32) {
    let pulse = 0.55 + (time * 3.0).sin().abs() * 0.45;
    let label = "[ Entree / Espace ] lancer la session";
    let x = screen_width() / 2.0 - measure_text(label, None, 26, 1.0).width / 2.0;
    let y = menu_card_rect().y + menu_card_rect().h + 118.0;
    draw_text(label, x, y, 26.0, Color::new(0.64, 0.72, 0.68, pulse));

    let sub = "terminus://depart  -  build rust";
    let sx = screen_width() / 2.0 - measure_text(sub, None, 18, 1.0).width / 2.0;
    draw_text(sub, sx, y + 35.0, 18.0, Color::from_rgba(80, 110, 96, 255));
}

fn draw_start_preview(time: f32) {
    let card = menu_card_rect();
    let rect = Rect::new(card.x + 82.0, card.y + card.h + 18.0, card.w - 164.0, 64.0);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(2, 5, 6, 210),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, DIM_GREEN);

    draw_text("depart:", rect.x + 18.0, rect.y + 40.0, 22.0, GREEN);
    draw_mini_player(vec2(rect.x + 146.0, rect.y + 38.0), time);
    draw_mini_palourde(vec2(rect.x + 242.0, rect.y + 38.0), time);
    draw_text(
        "parler -> apprendre cd/cat/ls/pwd",
        rect.x + 285.0,
        rect.y + 39.0,
        18.0,
        PAPER,
    );
}

fn draw_mini_player(center: Vec2, time: f32) {
    let y = center.y + (time * 5.0).sin() * 2.0;
    draw_rectangle(
        center.x - 7.0,
        y - 20.0,
        14.0,
        12.0,
        Color::from_rgba(210, 148, 92, 255),
    );
    draw_rectangle(
        center.x - 9.0,
        y - 10.0,
        18.0,
        16.0,
        Color::from_rgba(38, 108, 225, 255),
    );
    draw_rectangle(
        center.x - 8.0,
        y + 5.0,
        6.0,
        14.0,
        Color::from_rgba(18, 23, 38, 255),
    );
    draw_rectangle(
        center.x + 2.0,
        y + 5.0,
        6.0,
        14.0,
        Color::from_rgba(18, 23, 38, 255),
    );
    draw_rectangle(
        center.x - 10.0,
        y - 25.0,
        20.0,
        6.0,
        Color::from_rgba(24, 13, 10, 255),
    );
}

fn draw_mini_palourde(center: Vec2, time: f32) {
    let open = 7.0 + (time * 3.0).sin().abs() * 5.0;
    draw_circle(
        center.x,
        center.y,
        18.0,
        Color::from_rgba(196, 113, 180, 255),
    );
    draw_rectangle(
        center.x - 18.0,
        center.y - open / 2.0,
        36.0,
        open,
        Color::from_rgba(35, 10, 34, 255),
    );
    draw_rectangle(center.x - 7.0, center.y - 3.0, 5.0, 5.0, PAPER);
    draw_rectangle(center.x + 3.0, center.y - 3.0, 5.0, 5.0, PAPER);
}
