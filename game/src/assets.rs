use macroquad::prelude::*;

use crate::state::SceneId;

pub struct GameAssets {
    pub player_sheet: Option<Texture2D>,
    pub palourde_sheet: Option<Texture2D>,
    pub terrain_tiles: TerrainTiles,
    pub professor: Option<Texture2D>,
    pub pillar: Option<Texture2D>,
    pub sign: Option<Texture2D>,
}

pub struct TerrainTiles {
    depart: Option<Texture2D>,
    prairie: Option<Texture2D>,
    forest: Option<Texture2D>,
    academy: Option<Texture2D>,
    course: Option<Texture2D>,
    training: Option<Texture2D>,
}

impl GameAssets {
    pub async fn load() -> Self {
        Self {
            player_sheet: load_first(&[
                "assets/sprites/player_sheet.png",
                "game/assets/sprites/player_sheet.png",
            ])
            .await,
            palourde_sheet: load_first(&[
                "assets/sprites/palourde_sheet.png",
                "game/assets/sprites/palourde_sheet.png",
            ])
            .await,
            terrain_tiles: TerrainTiles::load().await,
            professor: load_first(&[
                "assets/terminus-rpg/sprites/npcs/professeursprites.png",
                "../assets/terminus-rpg/sprites/npcs/professeursprites.png",
            ])
            .await,
            pillar: load_first(&[
                "assets/terminus-rpg/sprites/sm sht/pilierterminus.png",
                "../assets/terminus-rpg/sprites/sm sht/pilierterminus.png",
            ])
            .await,
            sign: load_first(&[
                "assets/terminus-rpg/sprites/sm sht/panneau1.png",
                "../assets/terminus-rpg/sprites/sm sht/panneau1.png",
            ])
            .await,
        }
    }
}

impl TerrainTiles {
    async fn load() -> Self {
        Self {
            depart: load_tile("depart_grass_64.png").await,
            prairie: load_tile("prairie_grass_64.png").await,
            forest: load_tile("forest_moss_64.png").await,
            academy: load_tile("academy_tile_64.png").await,
            course: load_tile("course_tile_64.png").await,
            training: load_tile("training_stone_64.png").await,
        }
    }

    pub fn for_scene(&self, scene: SceneId) -> Option<&Texture2D> {
        match scene {
            SceneId::Depart => self.depart.as_ref(),
            SceneId::Prairie => self.prairie.as_ref(),
            SceneId::BoisDesLutins => self.forest.as_ref(),
            SceneId::AcademieDesBots => self.academy.as_ref(),
            SceneId::Cours => self.course.as_ref(),
            SceneId::SalleEntrainement => self.training.as_ref(),
        }
    }
}

async fn load_first(paths: &[&str]) -> Option<Texture2D> {
    for path in paths {
        if let Ok(texture) = load_texture(path).await {
            texture.set_filter(FilterMode::Nearest);
            return Some(texture);
        }
    }
    None
}

async fn load_tile(file_name: &str) -> Option<Texture2D> {
    let game_path = format!("assets/tiles/{file_name}");
    let root_path = format!("game/assets/tiles/{file_name}");
    load_first(&[&game_path, &root_path]).await
}
