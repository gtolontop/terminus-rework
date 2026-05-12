use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppMode {
    Menu,
    Intro,
    Playing,
    Dialog(DialogId),
    Complete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogId {
    Palourde,
    Sign,
    Professor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogReward {
    StarterSpells,
    Spell(Spell),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SceneId {
    Depart,
    Prairie,
    BoisDesLutins,
    AcademieDesBots,
    Cours,
    SalleEntrainement,
}

impl SceneId {
    pub fn label(self) -> &'static str {
        match self {
            SceneId::Depart => "DEPART",
            SceneId::Prairie => "PRAIRIE",
            SceneId::BoisDesLutins => "BOIS DES LUTINS",
            SceneId::AcademieDesBots => "ACADEMIE DES BOTS",
            SceneId::Cours => "COURS",
            SceneId::SalleEntrainement => "SALLE D'ENTRAINEMENT",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spell {
    Cd,
    Cat,
    Ls,
    Pwd,
    Mv,
}

impl Spell {
    pub fn label(self) -> &'static str {
        match self {
            Spell::Cd => "cd",
            Spell::Cat => "cat",
            Spell::Ls => "ls",
            Spell::Pwd => "pwd",
            Spell::Mv => "mv",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CarryKind {
    Professor,
    Pillar(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Facing {
    Down,
    Up,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct ActorState {
    pub scene: SceneId,
    pub pos: Vec2,
    pub boxed: bool,
}

#[derive(Debug)]
pub struct GameState {
    pub mode: AppMode,
    pub scene: SceneId,
    pub player_pos: Vec2,
    pub player_facing: Facing,
    pub exit_cooldown: f32,
    pub spells: Vec<Spell>,
    pub show_pwd: bool,
    pub carried: Option<CarryKind>,
    pub professor: ActorState,
    pub pillars: [ActorState; 3],
    pub intro_step: usize,
    pub solved_training_room: bool,
    pub pending_dialog_reward: Option<DialogReward>,
    pub toast: Option<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            mode: AppMode::Menu,
            scene: SceneId::Depart,
            player_pos: vec2(640.0, 360.0),
            player_facing: Facing::Down,
            exit_cooldown: 0.0,
            spells: vec![],
            show_pwd: false,
            carried: None,
            professor: ActorState {
                scene: SceneId::Cours,
                pos: vec2(760.0, 350.0),
                boxed: false,
            },
            pillars: [
                ActorState {
                    scene: SceneId::SalleEntrainement,
                    pos: vec2(470.0, 320.0),
                    boxed: false,
                },
                ActorState {
                    scene: SceneId::SalleEntrainement,
                    pos: vec2(620.0, 320.0),
                    boxed: false,
                },
                ActorState {
                    scene: SceneId::SalleEntrainement,
                    pos: vec2(770.0, 320.0),
                    boxed: false,
                },
            ],
            intro_step: 0,
            solved_training_room: false,
            pending_dialog_reward: None,
            toast: None,
        }
    }
}

impl GameState {
    pub fn knows(&self, spell: Spell) -> bool {
        self.spells.contains(&spell)
    }

    pub fn learn(&mut self, spell: Spell) {
        if !self.knows(spell) {
            self.spells.push(spell);
            self.toast = Some(format!("Nouveau sort appris : {}", spell.label()));
        }
    }

    pub fn learn_starter_spells(&mut self) {
        for spell in [Spell::Cd, Spell::Cat, Spell::Ls, Spell::Pwd] {
            if !self.knows(spell) {
                self.spells.push(spell);
            }
        }
        self.toast = Some("Sorts appris : cd, cat, ls, pwd".to_string());
    }
}
