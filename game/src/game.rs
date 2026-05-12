use macroquad::prelude::*;

use crate::input::InputFrame;
use crate::render::draw_game;
use crate::state::{AppMode, GameState};
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

    pub fn draw(&self) {
        match self.state.mode {
            AppMode::Menu => draw_menu(),
            AppMode::Intro => draw_intro(self.state.intro_step),
            AppMode::Playing => draw_game(&self.state),
            AppMode::Dialog(_) => draw_game(&self.state),
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
