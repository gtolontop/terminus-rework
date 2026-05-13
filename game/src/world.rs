use macroquad::prelude::*;

use crate::state::{GameState, SceneId, Spell};

pub const TRAINING_BOX: Rect = Rect::new(555.0, 500.0, 170.0, 90.0);

#[derive(Clone, Copy)]
pub struct Exit {
    pub rect: Rect,
    pub target: SceneId,
    pub spawn: Vec2,
    pub label: &'static str,
}

#[derive(Clone, Copy)]
pub struct StaticActor {
    pub id: &'static str,
    pub pos: Vec2,
    pub radius: f32,
    pub label: &'static str,
}

pub struct SceneDef {
    pub id: SceneId,
    pub exits: &'static [Exit],
    pub actors: &'static [StaticActor],
}

const DEPART_EXITS: &[Exit] = &[
    Exit {
        rect: Rect::new(1180.0, 250.0, 60.0, 220.0),
        target: SceneId::Prairie,
        spawn: vec2(170.0, 360.0),
        label: "Prairie",
    },
    Exit {
        rect: Rect::new(40.0, 250.0, 60.0, 220.0),
        target: SceneId::BoisDesLutins,
        spawn: vec2(1060.0, 360.0),
        label: "Bois",
    },
];

const PRAIRIE_EXITS: &[Exit] = &[Exit {
    rect: Rect::new(40.0, 250.0, 60.0, 220.0),
    target: SceneId::Depart,
    spawn: vec2(1060.0, 360.0),
    label: "Depart",
}];

const BOIS_EXITS: &[Exit] = &[
    Exit {
        rect: Rect::new(1180.0, 250.0, 60.0, 220.0),
        target: SceneId::Depart,
        spawn: vec2(170.0, 360.0),
        label: "Depart",
    },
    Exit {
        rect: Rect::new(530.0, 30.0, 220.0, 60.0),
        target: SceneId::AcademieDesBots,
        spawn: vec2(640.0, 610.0),
        label: "Academie",
    },
];

const ACADEMIE_EXITS: &[Exit] = &[
    Exit {
        rect: Rect::new(530.0, 640.0, 220.0, 50.0),
        target: SceneId::BoisDesLutins,
        spawn: vec2(640.0, 120.0),
        label: "Bois",
    },
    Exit {
        rect: Rect::new(40.0, 250.0, 60.0, 220.0),
        target: SceneId::SalleEntrainement,
        spawn: vec2(1060.0, 360.0),
        label: "Entrainement",
    },
    Exit {
        rect: Rect::new(1180.0, 250.0, 60.0, 220.0),
        target: SceneId::Cours,
        spawn: vec2(170.0, 360.0),
        label: "Cours",
    },
];

const COURS_EXITS: &[Exit] = &[Exit {
    rect: Rect::new(40.0, 250.0, 60.0, 220.0),
    target: SceneId::AcademieDesBots,
    spawn: vec2(1060.0, 360.0),
    label: "Academie",
}];

const TRAINING_EXITS: &[Exit] = &[Exit {
    rect: Rect::new(1180.0, 250.0, 60.0, 220.0),
    target: SceneId::AcademieDesBots,
    spawn: vec2(170.0, 360.0),
    label: "Academie",
}];

const DEPART_ACTORS: &[StaticActor] = &[StaticActor {
    id: "palourde",
    pos: vec2(640.0, 360.0),
    radius: 58.0,
    label: "Palourde",
}];

const BOIS_ACTORS: &[StaticActor] = &[StaticActor {
    id: "sign",
    pos: vec2(640.0, 430.0),
    radius: 56.0,
    label: "Panneau",
}];

const EMPTY_ACTORS: &[StaticActor] = &[];

pub fn scene_def(id: SceneId) -> SceneDef {
    match id {
        SceneId::Depart => SceneDef {
            id,
            exits: DEPART_EXITS,
            actors: DEPART_ACTORS,
        },
        SceneId::Prairie => SceneDef {
            id,
            exits: PRAIRIE_EXITS,
            actors: EMPTY_ACTORS,
        },
        SceneId::BoisDesLutins => SceneDef {
            id,
            exits: BOIS_EXITS,
            actors: BOIS_ACTORS,
        },
        SceneId::AcademieDesBots => SceneDef {
            id,
            exits: ACADEMIE_EXITS,
            actors: EMPTY_ACTORS,
        },
        SceneId::Cours => SceneDef {
            id,
            exits: COURS_EXITS,
            actors: EMPTY_ACTORS,
        },
        SceneId::SalleEntrainement => SceneDef {
            id,
            exits: TRAINING_EXITS,
            actors: EMPTY_ACTORS,
        },
    }
}

pub fn exit_locked_reason(state: &GameState, exit: &Exit) -> Option<&'static str> {
    let _ = exit;

    if state.scene == SceneId::Depart && !state.knows(Spell::Cd) {
        return Some("Parle a la Palourde");
    }

    None
}
