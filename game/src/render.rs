use macroquad::prelude::*;

use crate::assets::GameAssets;
use crate::pixel_art::draw_player_sprite;
use crate::state::{CarryKind, DialogId, Facing, GameState, SceneId, Spell};
use crate::visual_style::{draw_ambient_pixels, draw_pixel_scene, palette, pulse_color};
use crate::world::{Exit, SceneDef, TRAINING_BOX, exit_locked_reason, scene_def};

pub fn draw_game(state: &GameState, assets: &GameAssets) {
    let scene = scene_def(state.scene);
    draw_scene_floor(&scene, assets);
    draw_ambient_pixels(scene.id, get_time() as f32);
    draw_exits(&scene, state);
    draw_static_actors(&scene, assets);
    draw_dynamic_actors(state, assets);
    draw_player(state, assets);
    draw_hud(state);
}

pub fn draw_dialog_overlay(dialog: DialogId) {
    let (title, body) = match dialog {
        DialogId::Palourde => (
            "Palourde",
            "Connais-tu le sort pwd ? Il revele le dossier ou tu te trouves.",
        ),
        DialogId::Sign => (
            "Panneau",
            "Au nord, l'Academie des Bots. On y enseigne les sorts dangereux.",
        ),
        DialogId::Professor => (
            "Professeur",
            "mv deplace ce qui bloque ton chemin. Essaie-le dans la salle d'entrainement.",
        ),
    };

    draw_terminal_panel(
        Rect::new(190.0, 470.0, 900.0, 150.0),
        Color::from_rgba(245, 235, 180, 255),
    );
    draw_text(&format!("> {title}"), 220.0, 515.0, 30.0, YELLOW);
    draw_text(body, 220.0, 560.0, 24.0, WHITE);
    draw_text("[Entree/Espace] fermer", 220.0, 596.0, 20.0, GRAY);
}

fn draw_scene_floor(scene: &SceneDef, assets: &GameAssets) {
    draw_pixel_scene(scene.id, assets.terrain_tiles.for_scene(scene.id));
    draw_text(scene.id.label(), 48.0, 70.0, 32.0, WHITE);
}

fn draw_exits(scene: &SceneDef, state: &GameState) {
    for exit in scene.exits {
        if let Some(reason) = exit_locked_reason(state, exit) {
            draw_locked_exit(exit, reason);
        } else {
            draw_open_exit(exit);
        }
    }
}

fn draw_open_exit(exit: &Exit) {
    let time = get_time() as f32;
    let glow = pulse_color(Color::from_rgba(243, 190, 84, 180), time * 3.0, 0.35, 0.55);

    draw_rectangle(exit.rect.x, exit.rect.y, exit.rect.w, exit.rect.h, glow);
    draw_rectangle_lines(
        exit.rect.x,
        exit.rect.y,
        exit.rect.w,
        exit.rect.h,
        2.0,
        Color::from_rgba(255, 229, 157, 220),
    );
    draw_text(exit.label, exit.rect.x, exit.rect.y - 10.0, 20.0, WHITE);

    let sweep = (time * 42.0) % (exit.rect.h + 40.0);
    draw_line(
        exit.rect.x + 5.0,
        exit.rect.y + sweep - 20.0,
        exit.rect.x + exit.rect.w - 5.0,
        exit.rect.y + sweep,
        3.0,
        Color::from_rgba(255, 250, 202, 120),
    );
}

fn draw_locked_exit(exit: &Exit, reason: &str) {
    draw_rectangle(
        exit.rect.x,
        exit.rect.y,
        exit.rect.w,
        exit.rect.h,
        Color::from_rgba(37, 42, 47, 210),
    );
    draw_rectangle_lines(
        exit.rect.x,
        exit.rect.y,
        exit.rect.w,
        exit.rect.h,
        3.0,
        Color::from_rgba(120, 129, 137, 230),
    );

    let mut stripe_x = exit.rect.x - exit.rect.h;
    while stripe_x < exit.rect.x + exit.rect.w {
        draw_line(
            stripe_x,
            exit.rect.y + exit.rect.h,
            stripe_x + exit.rect.h,
            exit.rect.y,
            2.0,
            Color::from_rgba(160, 168, 176, 90),
        );
        stripe_x += 18.0;
    }

    draw_rectangle(
        exit.rect.x - 12.0,
        exit.rect.y - 30.0,
        168.0,
        24.0,
        Color::from_rgba(0, 0, 0, 190),
    );
    draw_text("bloque", exit.rect.x, exit.rect.y - 12.0, 19.0, LIGHTGRAY);
    draw_text(
        reason,
        exit.rect.x - 16.0,
        exit.rect.y + exit.rect.h + 24.0,
        18.0,
        LIGHTGRAY,
    );
}

fn draw_static_actors(scene: &SceneDef, assets: &GameAssets) {
    for actor in scene.actors {
        if actor.id == "sign" {
            draw_texture_centered(assets.sign.as_ref(), actor.pos, vec2(86.0, 86.0));
        } else {
            draw_circle(
                actor.pos.x,
                actor.pos.y,
                actor.radius,
                Color::from_rgba(191, 116, 178, 255),
            );
        }
        draw_text(
            actor.label,
            actor.pos.x - 42.0,
            actor.pos.y - actor.radius - 14.0,
            20.0,
            WHITE,
        );
    }
}

fn draw_dynamic_actors(state: &GameState, assets: &GameAssets) {
    if state.professor.scene == state.scene
        && !state.professor.boxed
        && state.carried != Some(CarryKind::Professor)
    {
        draw_actor_shadow(state.professor.pos, 32.0, 9.0);
        draw_texture_centered(
            assets.professor.as_ref(),
            state.professor.pos,
            vec2(92.0, 92.0),
        );
        draw_text(
            "Professeur",
            state.professor.pos.x - 48.0,
            state.professor.pos.y - 55.0,
            20.0,
            WHITE,
        );
    }

    for (index, pillar) in state.pillars.iter().enumerate() {
        let kind = CarryKind::Pillar(index);
        if pillar.scene == state.scene && !pillar.boxed && state.carried != Some(kind) {
            draw_actor_shadow(pillar.pos + vec2(0.0, 48.0), 30.0, 8.0);
            draw_texture_centered(assets.pillar.as_ref(), pillar.pos, vec2(72.0, 118.0));
            draw_text(
                &format!("Pilier {}", index + 1),
                pillar.pos.x - 40.0,
                pillar.pos.y - 64.0,
                18.0,
                WHITE,
            );
        }
    }

    if state.scene == SceneId::SalleEntrainement {
        let colors = palette(state.scene);
        draw_training_box(colors);
        draw_text("Boite", 610.0, 554.0, 24.0, WHITE);
    }
}

fn draw_player(state: &GameState, assets: &GameAssets) {
    draw_actor_shadow(state.player_pos + vec2(0.0, 36.0), 26.0, 8.0);

    if let Some(sheet) = assets.player_sheet.as_ref() {
        let frame = player_frame(state);
        draw_texture_ex(
            sheet,
            state.player_pos.x - 32.0,
            state.player_pos.y - 72.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 96.0)),
                source: Some(frame),
                ..Default::default()
            },
        );
    } else {
        draw_player_sprite(state.player_pos, state.player_facing);
    }

    draw_circle(
        state.player_pos.x,
        state.player_pos.y + 27.0,
        3.0,
        Color::from_rgba(95, 255, 154, 190),
    );

    if let Some(kind) = state.carried {
        let label = match kind {
            CarryKind::Professor => "mv Professeur",
            CarryKind::Pillar(_) => "mv Pilier",
        };
        draw_text(
            label,
            state.player_pos.x - 52.0,
            state.player_pos.y - 48.0,
            18.0,
            YELLOW,
        );
    }
}

fn player_frame(state: &GameState) -> Rect {
    const FRAME_W: f32 = 32.0;
    const FRAME_H: f32 = 48.0;

    let row = match state.player_facing {
        Facing::Down => 0.0,
        Facing::Left => 1.0,
        Facing::Right => 2.0,
        Facing::Up => 3.0,
    };
    let column = if state.player_moving {
        ((state.player_walk_timer * 9.5) as i32).rem_euclid(4) as f32
    } else {
        0.0
    };

    Rect::new(column * FRAME_W, row * FRAME_H, FRAME_W, FRAME_H)
}

fn draw_actor_shadow(center: Vec2, radius_x: f32, radius_y: f32) {
    draw_ellipse(
        center.x,
        center.y,
        radius_x,
        radius_y,
        0.0,
        Color::from_rgba(0, 0, 0, 75),
    );
}

fn draw_training_box(colors: crate::visual_style::ScenePalette) {
    draw_actor_shadow(
        vec2(
            TRAINING_BOX.x + TRAINING_BOX.w / 2.0,
            TRAINING_BOX.y + TRAINING_BOX.h,
        ),
        72.0,
        11.0,
    );
    draw_rectangle(
        TRAINING_BOX.x,
        TRAINING_BOX.y,
        TRAINING_BOX.w,
        TRAINING_BOX.h,
        Color::from_rgba(94, 55, 37, 255),
    );
    draw_rectangle(
        TRAINING_BOX.x + 12.0,
        TRAINING_BOX.y + 14.0,
        TRAINING_BOX.w - 24.0,
        14.0,
        Color::from_rgba(132, 78, 49, 255),
    );
    draw_rectangle(
        TRAINING_BOX.x + 20.0,
        TRAINING_BOX.y + 50.0,
        TRAINING_BOX.w - 40.0,
        10.0,
        Color::from_rgba(65, 38, 30, 255),
    );
    draw_rectangle_lines(
        TRAINING_BOX.x,
        TRAINING_BOX.y,
        TRAINING_BOX.w,
        TRAINING_BOX.h,
        4.0,
        Color::from_rgba(235, 210, 155, 255),
    );
    draw_rectangle_lines(
        TRAINING_BOX.x + 9.0,
        TRAINING_BOX.y + 9.0,
        TRAINING_BOX.w - 18.0,
        TRAINING_BOX.h - 18.0,
        2.0,
        Color::from_rgba(45, 28, 24, 220),
    );
    draw_rectangle(
        TRAINING_BOX.x + 10.0,
        TRAINING_BOX.y + 12.0,
        TRAINING_BOX.w - 20.0,
        12.0,
        Color::new(colors.glow.r, colors.glow.g, colors.glow.b, 0.18),
    );
}

fn draw_texture_centered(texture: Option<&Texture2D>, center: Vec2, size: Vec2) {
    if let Some(texture) = texture {
        draw_texture_ex(
            texture,
            center.x - size.x / 2.0,
            center.y - size.y / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    } else {
        draw_circle(center.x, center.y, size.x.min(size.y) * 0.35, WHITE);
    }
}

fn draw_hud(state: &GameState) {
    let spells = if state.spells.is_empty() {
        "aucun".to_string()
    } else {
        state
            .spells
            .iter()
            .map(|spell| spell.label())
            .collect::<Vec<_>>()
            .join("  ")
    };
    draw_terminal_panel(
        Rect::new(34.0, 650.0, 1212.0, 62.0),
        Color::from_rgba(128, 239, 190, 210),
    );
    draw_text(
        &format!("Sorts appris: {spells}"),
        52.0,
        676.0,
        20.0,
        Color::from_rgba(206, 255, 224, 255),
    );

    let actions = available_actions(state);
    draw_text(
        &format!("Possible ici > {}", actions.join("   |   ")),
        52.0,
        702.0,
        20.0,
        Color::from_rgba(190, 206, 214, 255),
    );

    if state.show_pwd {
        draw_terminal_panel(Rect::new(48.0, 96.0, 318.0, 48.0), YELLOW);
        draw_text(
            &format!("pwd -> {}", state.scene.label()),
            62.0,
            128.0,
            24.0,
            YELLOW,
        );
    }

    if let Some(toast) = &state.toast {
        draw_terminal_panel(
            Rect::new(430.0, 24.0, 420.0, 46.0),
            Color::from_rgba(150, 240, 180, 255),
        );
        draw_text(
            toast,
            448.0,
            55.0,
            22.0,
            Color::from_rgba(150, 240, 180, 255),
        );
    }
}

fn draw_terminal_panel(rect: Rect, accent: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(4, 8, 10, 224),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, accent);
    draw_rectangle(
        rect.x + 5.0,
        rect.y + 5.0,
        rect.w - 10.0,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.22),
    );
    draw_rectangle(
        rect.x + 5.0,
        rect.y + rect.h - 7.0,
        rect.w - 10.0,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.12),
    );
}

fn available_actions(state: &GameState) -> Vec<String> {
    let mut actions = vec!["Fleches/WASD: bouger".to_string()];

    if let Some(kind) = state.carried {
        let label = match kind {
            CarryKind::Professor => "professeur",
            CarryKind::Pillar(_) => "pilier",
        };

        if state.scene == SceneId::SalleEntrainement && TRAINING_BOX.contains(state.player_pos) {
            actions.push(format!("V: poser {label} dans la boite"));
        } else {
            actions.push(format!("V: poser {label}"));
        }
        return actions;
    }

    let scene = scene_def(state.scene);
    for actor in scene.actors {
        if state.player_pos.distance(actor.pos) <= actor.radius + 38.0 {
            match actor.id {
                "palourde" if state.knows(Spell::Cat) => {
                    actions.push("C/Espace: cat Palourde".to_string())
                }
                "palourde" => actions.push("Espace: parler a Palourde".to_string()),
                "sign" if state.knows(Spell::Cat) => {
                    actions.push("C/Espace: cat Panneau".to_string())
                }
                _ => {}
            }
        }
    }

    if state.professor.scene == state.scene
        && !state.professor.boxed
        && state.player_pos.distance(state.professor.pos) <= 86.0
    {
        if state.knows(Spell::Cat) {
            actions.push("C/Espace: cat Professeur".to_string());
        }
        if state.knows(Spell::Mv) {
            actions.push("M: mv Professeur".to_string());
        }
    }

    if state.knows(Spell::Mv) {
        for (index, pillar) in state.pillars.iter().enumerate() {
            if pillar.scene == state.scene
                && !pillar.boxed
                && state.player_pos.distance(pillar.pos) <= 82.0
            {
                actions.push(format!("M: mv Pilier {}", index + 1));
            }
        }
    }

    if state.knows(Spell::Pwd) {
        actions.push("P: pwd".to_string());
    }

    for exit in scene.exits {
        if point_near_rect(state.player_pos, exit.rect, 12.0) {
            if let Some(reason) = exit_locked_reason(state, exit) {
                actions.push(format!("Sortie bloquee: {reason}"));
            } else if state.knows(Spell::Cd) {
                actions.push(format!("cd: aller vers {}", exit.label));
            }
        }
    }

    actions
}

fn point_near_rect(point: Vec2, rect: Rect, margin: f32) -> bool {
    point.x >= rect.x - margin
        && point.x <= rect.x + rect.w + margin
        && point.y >= rect.y - margin
        && point.y <= rect.y + rect.h + margin
}
