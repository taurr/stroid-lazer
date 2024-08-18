use std::path::Path;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_persistent::{Persistent, StorageFormat};

use crate::{
    assets::{
        game_assets::GameAssets, sprite_dynamic_asset_collection::SpriteDynamicAssetCollection,
        AmmonitionDepot, AmmonitionTextureCollection, AsteroidPoolCollection,
        AsteroidTextureCollection, DefaultLevelSettings, GameAreaSettings,
        GameLevelSettingsCollection, GameSettings, GameStartSettings, HighScoreBoard,
        InputKeySettings, SpriteSheetAsset, StateBackgrounds, TextureCount, WeaponCollection,
    },
    states::GameState,
};

pub struct GameAssetsPlugin;

//#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
//pub struct GameAssetsSet;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        let state_dir = dirs::state_dir()
            .map(|native_state_dir| native_state_dir.join(env!("CARGO_PKG_NAME")))
            .unwrap_or(Path::new("local").join("state"));
        info!(?state_dir, "State directory");

        app.insert_resource(
            Persistent::<HighScoreBoard>::builder()
                .name("highscore board")
                .format(StorageFormat::Ron)
                .path(state_dir.join("highscores.ron"))
                .default(HighScoreBoard::default())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to initialize high-scores"),
        );

        // register assets for debug
        app.register_type::<AsteroidPoolCollection>()
            .register_type::<GameSettings>()
            .register_type::<InputKeySettings>()
            .register_type::<WeaponCollection>()
            .register_type::<AmmonitionDepot>()
            .register_type::<TextureCount>();

        // register ron asset loaders
        app.add_plugins((
            RonAssetPlugin::<AmmonitionDepot>::new(&["ammonition-depot.ron"]),
            RonAssetPlugin::<AsteroidPoolCollection>::new(&["asteroid-pool-collection.ron"]),
            RonAssetPlugin::<GameSettings>::new(&["game-settings.ron"]),
            RonAssetPlugin::<InputKeySettings>::new(&["input-key-settings.ron"]),
            RonAssetPlugin::<WeaponCollection>::new(&["weapon-collection.ron"]),
            RonAssetPlugin::<SpriteDynamicAssetCollection>::new(&["sprite-assets.ron"]),
            RonAssetPlugin::<GameLevelSettingsCollection>::new(&["level-settings.ron"]),
        ));

        // register assets that can be dynamically loaded, but are NOT registered through the RonAssetPlugin
        app.init_asset::<SpriteSheetAsset>()
            .init_asset::<AmmonitionTextureCollection>()
            .init_asset::<AsteroidTextureCollection>();

        // setup loading of assets
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::LoadingAssets)
                .register_dynamic_asset_collection::<SpriteDynamicAssetCollection>()
                .with_dynamic_assets_file::<SpriteDynamicAssetCollection>(
                    "stroid.sprite-assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "stroid.misc-assets.ron",
                )
                // TODO: load dynamically found dynamic_assets_files after the above!
                .load_collection::<GameAssets>()
                .load_collection::<StateBackgrounds>()
                // These are loaded settings that are added as resources via their [FromWorld] implementations
                .init_resource::<GameAreaSettings>()
                .init_resource::<GameStartSettings>()
                .init_resource::<DefaultLevelSettings>()
                .init_resource::<InputKeySettings>()
                .init_resource::<WeaponCollection>()
                .init_resource::<AmmonitionDepot>()
                .init_resource::<AsteroidPoolCollection>()
                .init_resource::<AmmonitionTextureCollection>()
                .init_resource::<AsteroidTextureCollection>()
                .init_resource::<GameLevelSettingsCollection>(),
        );
    }
}
