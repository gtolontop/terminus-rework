use macroquad::prelude::*;

use crate::state::{CarryKind, DialogId, GameState, SceneId};
use crate::world::{SceneDef, scene_def};

pub fn draw_game(state: &GameState) {
    let scene = scene_def(state.scene);
    draw_scene_floor(&scene);
    draw_exits(&scene);
    draw_static_actors(&scene);
    draw_dynamic_actors(state);
    draw_player(state);
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

    draw_rectangle(190.0, 470.0, 900.0, 150.0, Color::from_rgba(0, 0, 0, 220));
    draw_rectangle_lines(
        190.0,
        470.0,
        900.0,
        150.0,
        3.0,
        Color::from_rgba(245, 235, 180, 255),
    );
    draw_text(title, 220.0, 515.0, 30.0, YELLOW);
    draw_text(body, 220.0, 560.0, 24.0, WHITE);
    draw_text("Entree/Espace pour fermer", 220.0, 596.0, 20.0, GRAY);
}

fn draw_scene_floor(scene: &SceneDef) {
    clear_background(scene.background);
    draw_rectangle(40.0, 90.0, 1200.0, 560.0, Color::from_rgba(12, 15, 20, 95));
    draw_rectangle_lines(
        40.0,
        90.0,
        1200.0,
        560.0,
        4.0,
        Color::from_rgba(230, 235, 225, 130),
    );
    draw_text(scene.id.label(), 48.0, 70.0, 32.0, WHITE);
}

fn draw_exits(scene: &SceneDef) {
    for exit in scene.exits {
        draw_rectangle(
            exit.rect.x,
            exit.rect.y,
            exit.rect.w,
            exit.rect.h,
            Color::from_rgba(243, 190, 84, 145),
        );
        draw_text(exit.label, exit.rect.x, exit.rect.y - 10.0, 20.0, WHITE);
    }
}

fn draw_static_actors(scene: &SceneDef) {
    for actor in scene.actors {
        draw_circle(
            actor.pos.x,
            actor.pos.y,
            actor.radius,
            Color::from_rgba(191, 116, 178, 255),
        );
        draw_text(
            actor.label,
            actor.pos.x - 42.0,
            actor.pos.y - actor.radius - 14.0,
            20.0,
            WHITE,
        );
    }
}

fn draw_dynamic_actors(state: &GameState) {
    if state.professor.scene == state.scene
        && !state.professor.boxed
        && state.carried != Some(CarryKind::Professor)
    {
        draw_circle(
            state.professor.pos.x,
            state.professor.pos.y,
            42.0,
            Color::from_rgba(142, 190, 245, 255),
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
            draw_rectangle(
                pillar.pos.x - 28.0,
                pillar.pos.y - 50.0,
                56.0,
                100.0,
                Color::from_rgba(220, 197, 126, 255),
            );
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
        draw_rectangle(
            555.0,
            500.0,
            170.0,
            90.0,
            Color::from_rgba(100, 62, 45, 255),
        );
        draw_rectangle_lines(
            555.0,
            500.0,
            170.0,
            90.0,
            3.0,
            Color::from_rgba(235, 210, 155, 255),
        );
        draw_text("Boite", 610.0, 554.0, 24.0, WHITE);
    }
}

fn draw_player(state: &GameState) {
    draw_circle(
        state.player_pos.x,
        state.player_pos.y,
        30.0,
        Color::from_rgba(101, 225, 169, 255),
    );
    draw_circle_lines(
        state.player_pos.x,
        state.player_pos.y,
        34.0,
        3.0,
        Color::from_rgba(240, 255, 245, 210),
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

fn draw_hud(state: &GameState) {
    let spells = state
        .spells
        .iter()
        .map(|spell| spell.label())
        .collect::<Vec<_>>()
        .join("  ");
    draw_text(&format!("sorts: {spells}"), 48.0, 690.0, 22.0, WHITE);
    draw_text(
        "Fleches/WASD: bouger   C/Espace: cat   P: pwd   M: mv prendre   V: mv poser",
        430.0,
        690.0,
        20.0,
        GRAY,
    );

    if state.show_pwd {
        draw_rectangle(48.0, 96.0, 300.0, 48.0, Color::from_rgba(0, 0, 0, 175));
        draw_text(
            &format!("pwd -> {}", state.scene.label()),
            62.0,
            128.0,
            24.0,
            YELLOW,
        );
    }

    if let Some(toast) = &state.toast {
        draw_rectangle(430.0, 24.0, 420.0, 46.0, Color::from_rgba(0, 0, 0, 190));
        draw_text(
            toast,
            448.0,
            55.0,
            22.0,
            Color::from_rgba(150, 240, 180, 255),
        );
    }
}
