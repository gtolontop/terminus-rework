use macroquad::prelude::*;

pub struct GameAssets {
    pub professor: Option<Texture2D>,
    pub pillar: Option<Texture2D>,
    pub sign: Option<Texture2D>,
}

impl GameAssets {
    pub async fn load() -> Self {
        Self {
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

async fn load_first(paths: &[&str]) -> Option<Texture2D> {
    for path in paths {
        if let Ok(texture) = load_texture(path).await {
            texture.set_filter(FilterMode::Nearest);
            return Some(texture);
        }
    }
    None
}
