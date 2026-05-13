use macroquad::prelude::*;

use crate::layout;
use crate::state::SceneId;

const TILE: f32 = 128.0;

#[derive(Clone, Copy)]
pub struct ScenePalette {
    pub void: Color,
    pub floor: Color,
    pub floor_alt: Color,
    pub detail: Color,
    pub line: Color,
    pub glow: Color,
}

pub fn palette(scene: SceneId) -> ScenePalette {
    match scene {
        SceneId::Depart => ScenePalette {
            void: Color::from_rgba(14, 26, 21, 255),
            floor: Color::from_rgba(47, 92, 64, 255),
            floor_alt: Color::from_rgba(38, 77, 55, 255),
            detail: Color::from_rgba(97, 148, 83, 255),
            line: Color::from_rgba(29, 53, 43, 255),
            glow: Color::from_rgba(160, 236, 146, 255),
        },
        SceneId::Prairie => ScenePalette {
            void: Color::from_rgba(22, 38, 20, 255),
            floor: Color::from_rgba(74, 119, 58, 255),
            floor_alt: Color::from_rgba(62, 103, 52, 255),
            detail: Color::from_rgba(151, 192, 89, 255),
            line: Color::from_rgba(39, 73, 39, 255),
            glow: Color::from_rgba(215, 242, 133, 255),
        },
        SceneId::BoisDesLutins => ScenePalette {
            void: Color::from_rgba(10, 22, 24, 255),
            floor: Color::from_rgba(33, 70, 58, 255),
            floor_alt: Color::from_rgba(27, 58, 50, 255),
            detail: Color::from_rgba(79, 121, 90, 255),
            line: Color::from_rgba(17, 39, 37, 255),
            glow: Color::from_rgba(109, 225, 185, 255),
        },
        SceneId::AcademieDesBots => ScenePalette {
            void: Color::from_rgba(18, 20, 34, 255),
            floor: Color::from_rgba(55, 66, 102, 255),
            floor_alt: Color::from_rgba(47, 57, 88, 255),
            detail: Color::from_rgba(107, 123, 170, 255),
            line: Color::from_rgba(31, 37, 58, 255),
            glow: Color::from_rgba(130, 192, 255, 255),
        },
        SceneId::Cours => ScenePalette {
            void: Color::from_rgba(25, 19, 33, 255),
            floor: Color::from_rgba(76, 56, 89, 255),
            floor_alt: Color::from_rgba(64, 48, 76, 255),
            detail: Color::from_rgba(138, 101, 147, 255),
            line: Color::from_rgba(42, 32, 54, 255),
            glow: Color::from_rgba(230, 159, 250, 255),
        },
        SceneId::SalleEntrainement => ScenePalette {
            void: Color::from_rgba(21, 22, 23, 255),
            floor: Color::from_rgba(72, 75, 77, 255),
            floor_alt: Color::from_rgba(59, 63, 66, 255),
            detail: Color::from_rgba(119, 126, 130, 255),
            line: Color::from_rgba(38, 41, 44, 255),
            glow: Color::from_rgba(238, 213, 143, 255),
        },
    }
}

pub fn draw_pixel_scene(scene: SceneId, terrain_tile: Option<&Texture2D>) {
    let colors = palette(scene);
    clear_background(colors.void);
    draw_play_area(colors);
    draw_scene_texture(scene, colors, terrain_tile);
    draw_tile_motion(scene, colors, get_time() as f32);
    draw_play_frame(colors);
    draw_vignette();
}

pub fn draw_ambient_pixels(scene: SceneId, time: f32) {
    let colors = palette(scene);
    match scene {
        SceneId::Depart | SceneId::Prairie | SceneId::BoisDesLutins => {
            draw_fireflies(colors, time);
        }
        SceneId::AcademieDesBots | SceneId::Cours => {
            draw_magic_dust(colors, time);
        }
        SceneId::SalleEntrainement => {
            draw_training_sparks(colors, time);
        }
    }
}

fn draw_play_area(colors: ScenePalette) {
    let play_area = layout::play_rect();
    draw_rectangle(
        play_area.x,
        play_area.y,
        play_area.w,
        play_area.h,
        colors.floor,
    );
}

fn draw_scene_texture(scene: SceneId, colors: ScenePalette, terrain_tile: Option<&Texture2D>) {
    let play_area = layout::play_rect();
    let cols = (play_area.w / TILE).ceil() as i32;
    let rows = (play_area.h / TILE).ceil() as i32;

    for row in 0..rows {
        for col in 0..cols {
            let x = play_area.x + col as f32 * TILE;
            let y = play_area.y + row as f32 * TILE;
            if let Some(tile) = terrain_tile {
                draw_texture_ex(
                    tile,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TILE, TILE)),
                        ..Default::default()
                    },
                );
            } else {
                let alt = (row + col) % 2 == 0;
                draw_rectangle(
                    x,
                    y,
                    TILE,
                    TILE,
                    if alt { colors.floor } else { colors.floor_alt },
                );
            }

            if terrain_tile.is_none() {
                match scene {
                    SceneId::Depart | SceneId::Prairie | SceneId::BoisDesLutins => {
                        draw_grass_detail(x, y, col, row, colors);
                    }
                    SceneId::AcademieDesBots | SceneId::Cours => {
                        draw_tile_glyph(x, y, col, row, colors)
                    }
                    SceneId::SalleEntrainement => draw_stone_detail(x, y, col, row, colors),
                }
            }
        }
    }
}

fn draw_play_frame(colors: ScenePalette) {
    let play = layout::play_rect();

    draw_rectangle(0.0, 0.0, screen_width(), play.y, colors.void);
    draw_rectangle(
        0.0,
        play.y + play.h,
        screen_width(),
        screen_height() - play.y - play.h,
        colors.void,
    );
    draw_rectangle(0.0, play.y, play.x, play.h, colors.void);
    draw_rectangle(
        play.x + play.w,
        play.y,
        screen_width() - play.x - play.w,
        play.h,
        colors.void,
    );

    draw_rectangle_lines(
        play.x - 3.0,
        play.y - 3.0,
        play.w + 6.0,
        play.h + 6.0,
        3.0,
        Color::from_rgba(8, 14, 15, 240),
    );
    draw_rectangle_lines(
        play.x,
        play.y,
        play.w,
        play.h,
        2.0,
        Color::from_rgba(126, 220, 174, 210),
    );
    draw_rectangle_lines(
        play.x + 5.0,
        play.y + 5.0,
        play.w - 10.0,
        play.h - 10.0,
        1.0,
        Color::from_rgba(222, 238, 224, 70),
    );
}

fn draw_grass_detail(x: f32, y: f32, col: i32, row: i32, colors: ScenePalette) {
    let seed = hash2(col, row);
    for blade in 0..3 {
        let bx = x + 5.0 + ((seed + blade * 7) % 22) as f32;
        let by = y + 7.0 + ((seed / 3 + blade * 9) % 20) as f32;
        draw_rectangle(bx, by, 3.0, 9.0, colors.detail);
    }
    if seed % 17 == 0 {
        draw_rectangle(x + 14.0, y + 14.0, 4.0, 4.0, colors.glow);
    }
}

fn draw_tile_glyph(x: f32, y: f32, col: i32, row: i32, colors: ScenePalette) {
    draw_rectangle_lines(x, y, TILE, TILE, 1.0, colors.line);
    if hash2(col, row) % 5 == 0 {
        draw_rectangle(
            x + 11.0,
            y + 11.0,
            10.0,
            10.0,
            Color::from_rgba(255, 255, 255, 32),
        );
    }
}

fn draw_stone_detail(x: f32, y: f32, col: i32, row: i32, colors: ScenePalette) {
    draw_rectangle_lines(x, y, TILE, TILE, 1.0, colors.line);
    if hash2(col, row) % 3 == 0 {
        draw_line(x + 6.0, y + 20.0, x + 26.0, y + 14.0, 2.0, colors.detail);
    }
}

fn draw_tile_motion(scene: SceneId, colors: ScenePalette, time: f32) {
    match scene {
        SceneId::Depart | SceneId::Prairie | SceneId::BoisDesLutins => {
            draw_waving_grass(scene, colors, time);
        }
        _ => {}
    }
}

fn draw_waving_grass(scene: SceneId, colors: ScenePalette, time: f32) {
    let play_area = layout::play_rect();
    let density = match scene {
        SceneId::Prairie => 46,
        SceneId::BoisDesLutins => 26,
        _ => 30,
    };

    for index in 0..density {
        let seed = hash2(index * 11, index * 29);
        let x = play_area.x + 34.0 + (seed % (play_area.w as i32 - 68)) as f32;
        let y = play_area.y + 34.0 + ((seed / 13) % (play_area.h as i32 - 68)) as f32;
        let height = 9.0 + (seed % 7) as f32;
        let sway = (time * 1.8 + index as f32 * 0.61).sin() * 3.0;
        let alpha = match scene {
            SceneId::Prairie => 0.68,
            SceneId::BoisDesLutins => 0.42,
            _ => 0.48,
        };

        draw_line(
            x,
            y + height,
            x + sway,
            y,
            2.0,
            Color::new(colors.detail.r, colors.detail.g, colors.detail.b, alpha),
        );
        draw_rectangle(
            x + sway - 1.0,
            y - 1.0,
            2.0,
            2.0,
            Color::new(colors.glow.r, colors.glow.g, colors.glow.b, alpha * 0.55),
        );
    }
}

fn draw_vignette() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        34.0,
        Color::from_rgba(0, 0, 0, 65),
    );
    draw_rectangle(
        0.0,
        screen_height() - 34.0,
        screen_width(),
        34.0,
        Color::from_rgba(0, 0, 0, 75),
    );
}

fn draw_fireflies(colors: ScenePalette, time: f32) {
    let play_area = layout::play_rect();
    for index in 0..18 {
        let seed = hash2(index, index * 7) as f32;
        let x = play_area.x + 80.0 + (seed % (play_area.w - 160.0).max(1.0));
        let y = play_area.y + 70.0 + ((seed / 11.0) % (play_area.h - 140.0).max(1.0));
        let bob = (time * 1.8 + index as f32).sin() * 5.0;
        let alpha = 90 + ((time * 2.4 + index as f32).sin().abs() * 120.0) as u8;
        draw_rectangle(
            x,
            y + bob,
            5.0,
            5.0,
            Color::from_rgba(
                (colors.glow.r * 255.0) as u8,
                (colors.glow.g * 255.0) as u8,
                (colors.glow.b * 255.0) as u8,
                alpha,
            ),
        );
    }
}

fn draw_magic_dust(colors: ScenePalette, time: f32) {
    let play_area = layout::play_rect();
    for index in 0..24 {
        let seed = hash2(index * 5, index * 13) as f32;
        let x = play_area.x + 70.0 + (seed % (play_area.w - 140.0).max(1.0));
        let y = play_area.y + 80.0 + ((seed / 9.0 + time * 18.0) % (play_area.h - 160.0).max(1.0));
        draw_rectangle(
            x,
            y,
            4.0,
            4.0,
            Color::new(colors.glow.r, colors.glow.g, colors.glow.b, 0.35),
        );
    }
}

fn draw_training_sparks(colors: ScenePalette, time: f32) {
    let play_area = layout::play_rect();
    for index in 0..12 {
        let seed = hash2(index * 3, index * 19) as f32;
        let x = play_area.x + 130.0 + (seed % (play_area.w - 260.0).max(1.0));
        let y = play_area.y + 120.0 + ((seed / 17.0) % (play_area.h - 240.0).max(1.0));
        let blink = (time * 4.0 + index as f32).sin().max(0.0);
        draw_rectangle(
            x,
            y,
            6.0,
            2.0 + blink * 5.0,
            Color::new(colors.glow.r, colors.glow.g, colors.glow.b, 0.45),
        );
    }
}

fn hash2(x: i32, y: i32) -> i32 {
    let mut n = x
        .wrapping_mul(374_761)
        .wrapping_add(y.wrapping_mul(668_265));
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    (n ^ (n >> 16)) & i32::MAX
}

#[cfg(test)]
mod tests {
    use super::hash2;

    #[test]
    fn hash2_stays_inside_positive_i32_range() {
        for (x, y) in [(0, 0), (18, 126), (i32::MAX, i32::MIN)] {
            assert!(hash2(x, y) >= 0);
        }
    }
}
