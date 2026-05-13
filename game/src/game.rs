use macroquad::prelude::*;

use crate::assets::GameAssets;
use crate::input::InputFrame;
use crate::layout;
use crate::menu::draw_menu;
use crate::render::{draw_dialog_overlay, draw_game};
use crate::state::{AppMode, CarryKind, DialogId, DialogReward, Facing, GameState, Spell};
use crate::world::{TRAINING_BOX, exit_locked_reason, scene_def};

pub struct Game {
    state: GameState,
    assets: GameAssets,
}

impl Game {
    pub fn new(assets: GameAssets) -> Self {
        Self {
            state: GameState::default(),
            assets,
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
            AppMode::Dialog(_) => {
                if input.confirm {
                    self.apply_dialog_reward();
                    self.state.pending_dialog_reward = None;
                    self.state.mode = AppMode::Playing;
                }
            }
            AppMode::Complete => {
                if input.confirm {
                    self.state.mode = AppMode::Playing;
                }
            }
        }
    }

    fn update_playing(&mut self, input: InputFrame) {
        let delta = get_frame_time();
        self.state.exit_cooldown = (self.state.exit_cooldown - delta).max(0.0);
        self.state.player_moving = input.direction.length_squared() > 0.01;
        if self.state.player_moving {
            self.state.player_walk_timer += delta;
        }

        if input.direction.x < -0.1 {
            self.state.player_facing = Facing::Left;
        } else if input.direction.x > 0.1 {
            self.state.player_facing = Facing::Right;
        } else if input.direction.y < -0.1 {
            self.state.player_facing = Facing::Up;
        } else if input.direction.y > 0.1 {
            self.state.player_facing = Facing::Down;
        }

        let scene = scene_def(self.state.scene);
        let proposed_pos =
            clamped_player_pos(self.state.player_pos + input.direction * 260.0 * delta);
        if let Some(reason) = blocked_exit_at(&self.state, proposed_pos) {
            self.state.player_moving = false;
            self.state.toast = Some(reason.to_string());
        } else {
            self.state.player_pos = proposed_pos;
        }

        if input.cat {
            if let Some(dialog) = self.find_cat_dialog() {
                self.state.pending_dialog_reward = match dialog {
                    DialogId::Palourde => Some(DialogReward::StarterSpells),
                    DialogId::Professor => Some(DialogReward::Spell(Spell::Mv)),
                    DialogId::Sign => None,
                };
                self.state.mode = AppMode::Dialog(dialog);
                return;
            }
        }

        if input.pwd && self.state.knows(Spell::Pwd) {
            self.state.show_pwd = !self.state.show_pwd;
        }

        if input.mv_pick && self.state.knows(Spell::Mv) && self.state.carried.is_none() {
            self.state.carried = self.find_movable();
            if self.state.carried.is_some() {
                self.state.toast = Some("Objet attrape avec mv".to_string());
            }
        }

        if input.mv_drop && self.state.knows(Spell::Mv) {
            self.drop_carried();
        }

        if self.state.exit_cooldown > 0.0 {
            return;
        }

        for exit in scene.exits {
            if exit.rect.contains(self.state.player_pos) {
                if let Some(reason) = exit_locked_reason(&self.state, exit) {
                    self.state.toast = Some(reason.to_string());
                    break;
                }

                self.state.scene = exit.target;
                self.state.player_pos = exit.spawn;
                self.state.exit_cooldown = 0.18;
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
                    "sign" if self.state.knows(Spell::Cat) => Some(DialogId::Sign),
                    _ => None,
                };
            }
        }

        if self.state.professor.scene == self.state.scene
            && !self.state.professor.boxed
            && self.state.knows(Spell::Cat)
            && self.state.player_pos.distance(self.state.professor.pos) <= 86.0
        {
            return Some(DialogId::Professor);
        }

        None
    }

    fn apply_dialog_reward(&mut self) {
        match self.state.pending_dialog_reward {
            Some(DialogReward::StarterSpells) => self.state.learn_starter_spells(),
            Some(DialogReward::Spell(spell)) => self.state.learn(spell),
            None => {}
        }
    }

    fn find_movable(&self) -> Option<CarryKind> {
        if self.state.professor.scene == self.state.scene
            && !self.state.professor.boxed
            && self.state.player_pos.distance(self.state.professor.pos) <= 86.0
        {
            return Some(CarryKind::Professor);
        }

        for (index, pillar) in self.state.pillars.iter().enumerate() {
            if pillar.scene == self.state.scene
                && !pillar.boxed
                && self.state.player_pos.distance(pillar.pos) <= 82.0
            {
                return Some(CarryKind::Pillar(index));
            }
        }

        None
    }

    fn drop_carried(&mut self) {
        let Some(kind) = self.state.carried.take() else {
            return;
        };

        let dropped_in_box = self.state.scene == crate::state::SceneId::SalleEntrainement
            && TRAINING_BOX.contains(self.state.player_pos);

        match kind {
            CarryKind::Professor => {
                self.state.professor.scene = self.state.scene;
                self.state.professor.pos = self.state.player_pos + vec2(70.0, 0.0);
                self.state.professor.boxed = false;
            }
            CarryKind::Pillar(index) => {
                self.state.pillars[index].scene = self.state.scene;
                self.state.pillars[index].pos = self.state.player_pos + vec2(62.0, 0.0);
                self.state.pillars[index].boxed = dropped_in_box;
            }
        }

        self.state.toast = Some(if dropped_in_box {
            "Objet range dans la boite".to_string()
        } else {
            "Objet depose".to_string()
        });

        self.check_training_room();
    }

    fn check_training_room(&mut self) {
        if !self.state.solved_training_room && self.state.pillars.iter().all(|pillar| pillar.boxed)
        {
            self.state.solved_training_room = true;
            self.state.mode = AppMode::Complete;
            self.state.toast = Some("Premiere partie terminee : mv maitrise".to_string());
        }
    }

    pub fn draw(&self) {
        match self.state.mode {
            AppMode::Menu => draw_menu(),
            AppMode::Intro => draw_intro(self.state.intro_step),
            AppMode::Playing => draw_game(&self.state, &self.assets),
            AppMode::Dialog(dialog) => {
                draw_game(&self.state, &self.assets);
                draw_dialog_overlay(dialog);
            }
            AppMode::Complete => {
                draw_game(&self.state, &self.assets);
                draw_complete();
            }
        }
    }
}

fn clamped_player_pos(pos: Vec2) -> Vec2 {
    let bounds = layout::VIRTUAL_PLAY;
    vec2(
        pos.x.clamp(bounds.x + 30.0, bounds.x + bounds.w - 30.0),
        pos.y.clamp(bounds.y + 30.0, bounds.y + bounds.h - 30.0),
    )
}

fn blocked_exit_at(state: &GameState, pos: Vec2) -> Option<&'static str> {
    let scene = scene_def(state.scene);
    scene.exits.iter().find_map(|exit| {
        exit.rect
            .contains(pos)
            .then(|| exit_locked_reason(state, exit))
            .flatten()
    })
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

fn draw_complete() {
    draw_rectangle(250.0, 220.0, 780.0, 190.0, Color::from_rgba(0, 0, 0, 230));
    draw_rectangle_lines(250.0, 220.0, 780.0, 190.0, 3.0, GREEN);
    draw_text("Premiere partie validee", 385.0, 292.0, 38.0, WHITE);
    draw_text(
        "Tu es au meme stade que le prototype Godot : pwd + mv + salle resolue.",
        305.0,
        345.0,
        24.0,
        GRAY,
    );
}
