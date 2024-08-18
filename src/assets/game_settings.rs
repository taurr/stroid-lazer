use bevy::prelude::*;
use derive_more::Constructor;
use serde::Deserialize;

use crate::assets::{DefaultLevelSettings, GameAreaSettings, GameStartSettings};

/// Loaded as part of the [crate::assets::GameAssets] collection.
#[derive(Asset, Reflect, Deserialize, Debug, Clone, Constructor)]
pub struct GameSettings {
    pub game_area: GameAreaSettings,
    pub game_start: GameStartSettings,
    pub level_defaults: DefaultLevelSettings,
}
