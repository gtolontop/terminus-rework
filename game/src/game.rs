use macroquad::prelude::*;

use crate::input::InputFrame;
use crate::render::{draw_dialog_overlay, draw_game};
use crate::state::{AppMode, DialogId, GameState, Spell};
use crate::world::scene_def;

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::default(),
        }
    }

    pub fn update(&mut self) {
        let input = InputFrame::read();

        match self.state.mode {
            AppMode::Menu => {
                if input.confirm {
                    self.state.mode = AppMode::Intro;
                }
            }
            AppMode::Intro => {
                if input.confirm {
                    self.state.intro_step += 1;
                    if self.state.intro_step >= 4 {
                        self.state.mode = AppMode::Playing;
                    }
                }
            }
            AppMode::Playing => {
                self.update_playing(input);
            }
            AppMode::Dialog(_) | AppMode::Complete => {
                if input.confirm {
                    self.state.mode = AppMode::Playing;
                }
            }
        }
    }

    fn update_playing(&mut self, input: InputFrame) {
        self.state.player_pos += input.direction * 260.0 * get_frame_time();
        self.state.player_pos.x = self.state.player_pos.x.clamp(70.0, 1210.0);
        self.state.player_pos.y = self.state.player_pos.y.clamp(120.0, 620.0);

        let scene = scene_def(self.state.scene);
        if input.cat {
            if let Some(dialog) = self.find_cat_dialog() {
                match dialog {
                    DialogId::Palourde => self.state.learn(Spell::Pwd),
                    DialogId::Professor => self.state.learn(Spell::Mv),
                    DialogId::Sign => {}
                }
                self.state.mode = AppMode::Dialog(dialog);
                return;
            }
        }

        if input.pwd && self.state.knows(Spell::Pwd) {
            self.state.show_pwd = !self.state.show_pwd;
        }

        for exit in scene.exits {
            if exit.rect.contains(self.state.player_pos) {
                self.state.scene = exit.target;
                self.state.player_pos = exit.spawn;
                self.state.show_pwd = false;
                self.state.toast = None;
                break;
            }
        }
    }

    fn find_cat_dialog(&self) -> Option<DialogId> {
        let scene = scene_def(self.state.scene);
        for actor in scene.actors {
            if self.state.player_pos.distance(actor.pos) <= actor.radius + 38.0 {
                return match actor.id {
                    "palourde" => Some(DialogId::Palourde),
                    "sign" => Some(DialogId::Sign),
                    _ => None,
                };
            }
        }

        if self.state.professor.scene == self.state.scene
            && !self.state.professor.boxed
            && self.state.player_pos.distance(self.state.professor.pos) <= 86.0
        {
            return Some(DialogId::Professor);
        }

        None
    }

    pub fn draw(&self) {
        match self.state.mode {
            AppMode::Menu => draw_menu(),
            AppMode::Intro => draw_intro(self.state.intro_step),
            AppMode::Playing => draw_game(&self.state),
            AppMode::Dialog(dialog) => {
                draw_game(&self.state);
                draw_dialog_overlay(dialog);
            }
            AppMode::Complete => draw_game(&self.state),
        }
    }
}

fn draw_menu() {
    clear_background(Color::from_rgba(12, 14, 18, 255));
    draw_text("TERMINUS REWORK", 340.0, 280.0, 56.0, WHITE);
    draw_text("appuie sur Entree ou Espace", 446.0, 345.0, 26.0, GRAY);
}

fn draw_intro(step: usize) {
    clear_background(Color::from_rgba(18, 20, 28, 255));
    let lines = [
        "Bienvenue dans Terminus.",
        "Ici les commandes sont des sorts.",
        "Observe, parle, apprends, puis casse les regles.",
        "Premiere mission : retrouver pwd puis mv.",
    ];
    draw_text(lines[step.min(lines.len() - 1)], 250.0, 320.0, 34.0, WHITE);
    draw_text("Entree/Espace pour continuer", 420.0, 390.0, 24.0, GRAY);
}
