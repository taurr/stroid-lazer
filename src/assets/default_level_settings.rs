use bevy::prelude::*;
use serde::Deserialize;

use crate::assets::{game_assets::GameAssets, GameSettings, PlayerSettings};

/// Resource is initialized after loading assets, through its [FromWorld] implementation.
#[derive(Resource, Reflect, Deserialize, Debug, Clone)]
pub struct DefaultLevelSettings {
    pub player: PlayerSettings,
}

impl FromWorld for DefaultLevelSettings {
    fn from_world(world: &mut World) -> Self {
        let game_settings = {
            let assets = world.resource::<GameAssets>();
            world
                .resource::<Assets<GameSettings>>()
                .get(assets.game_settings.id())
                .unwrap()
        };

        game_settings.level_defaults.clone()
    }
}
