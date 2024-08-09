use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

use super::{game_assets::GameAssets, optional, LevelStartupSettings, PlayerSettingOptions};

/// Resource is initialized after loading assets, through its [FromWorld] implementation.
#[derive(Asset, Resource, Deserialize, Reflect, Debug, Clone, Deref)]
pub struct GameLevelSettingsCollection(HashMap<String, GameLevelSettings>);

impl FromWorld for GameLevelSettingsCollection {
    fn from_world(world: &mut World) -> Self {
        let game_settings = {
            let assets = world.resource::<GameAssets>();
            world
                .resource::<Assets<GameLevelSettingsCollection>>()
                .get(assets.levels.id())
                .unwrap()
        };
        game_settings.clone()
    }
}

/// Resource is initialized during [crate::states::init_level_settings].
#[derive(Resource, Deserialize, Reflect, Debug, Clone)]
pub struct GameLevelSettings {
    pub background: String,
    pub startup: LevelStartupSettings,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub next_level: Option<String>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub player: Option<PlayerSettingOptions>,
}
