use core::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;

use super::{game_assets::GameAssets, GameSettings};

/// Resource is initialized after loading assets, through its [FromWorld] implementation.
#[derive(Resource, Reflect, Deserialize, Debug, Clone)]
pub struct GameStartSettings {
    pub lives: usize,
    pub level: String,
    pub weapon_key: String,
    pub countdown_duration: Duration,
    pub minimum_countdown_duration: Duration,
    pub new_life_every: usize,
}

impl FromWorld for GameStartSettings {
    fn from_world(world: &mut World) -> Self {
        let game_settings = {
            let assets = world.resource::<GameAssets>();
            world
                .resource::<Assets<GameSettings>>()
                .get(assets.game_settings.id())
                .unwrap()
        };
        game_settings.game_start.clone()
    }
}
