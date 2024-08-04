use bevy::prelude::*;
use serde::Deserialize;

use crate::movement::GameArea;

use super::{game_assets::GameAssets, GameSettings};

/// Resource is initialized after loading assets, through its [FromWorld] implementation.
#[derive(Resource, Deserialize, Reflect, Debug, Clone)]
pub struct GameAreaSettings {
    pub border_area: Rect,
    pub game_area: GameArea,
}

impl FromWorld for GameAreaSettings {
    fn from_world(world: &mut World) -> Self {
        let game_settings = {
            let assets = world.resource::<GameAssets>();
            world
                .resource::<Assets<GameSettings>>()
                .get(assets.game_settings.id())
                .unwrap()
        };
        game_settings.game_area.clone()
    }
}
