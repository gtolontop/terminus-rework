use macroquad::prelude::*;

use crate::state::SceneId;

const PLAY_AREA: Rect = Rect::new(40.0, 90.0, 1200.0, 560.0);
const TILE: f32 = 32.0;

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

pub fn draw_pixel_scene(scene: SceneId) {
    let colors = palette(scene);
    clear_background(colors.void);
    draw_play_area(colors);
    draw_scene_texture(scene, colors);
    draw_vignette();
}

fn draw_play_area(colors: ScenePalette) {
    draw_rectangle(
        PLAY_AREA.x,
        PLAY_AREA.y,
        PLAY_AREA.w,
        PLAY_AREA.h,
        colors.floor,
    );
    draw_rectangle_lines(
        PLAY_AREA.x,
        PLAY_AREA.y,
        PLAY_AREA.w,
        PLAY_AREA.h,
        4.0,
        Color::from_rgba(236, 238, 222, 150),
    );
    draw_rectangle_lines(
        PLAY_AREA.x + 8.0,
        PLAY_AREA.y + 8.0,
        PLAY_AREA.w - 16.0,
        PLAY_AREA.h - 16.0,
        2.0,
        Color::from_rgba(10, 12, 16, 120),
    );
}

fn draw_scene_texture(scene: SceneId, colors: ScenePalette) {
    let cols = (PLAY_AREA.w / TILE).ceil() as i32;
    let rows = (PLAY_AREA.h / TILE).ceil() as i32;

    for row in 0..rows {
        for col in 0..cols {
            let x = PLAY_AREA.x + col as f32 * TILE;
            let y = PLAY_AREA.y + row as f32 * TILE;
            let alt = (row + col) % 2 == 0;
            draw_rectangle(
                x,
                y,
                TILE,
                TILE,
                if alt { colors.floor } else { colors.floor_alt },
            );

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

fn hash2(x: i32, y: i32) -> i32 {
    let mut n = x * 374_761 + y * 668_265;
    n = (n ^ (n >> 13)) * 1_274_126_177;
    (n ^ (n >> 16)).abs()
}
