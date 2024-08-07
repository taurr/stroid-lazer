use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::assets::{GameSettings, SpriteSheetAsset};

use super::{
    ammonition_depot::AmmonitionDepot, asteroid_pool_collection::AsteroidPoolCollection,
    asteroid_texture_collection::AsteroidTextureCollection, input_key_settings::InputKeySettings,
    weapon_collection::WeaponCollection, AmmonitionTextureCollection, GameLevelSettingsCollection,
};

/// Loaded directly as a [Resource] by [bevy_asset_loader].
#[derive(AssetCollection, Resource, Debug)]
pub struct GameAssets {
    /// The individual fields of this asset can be found as individual [Resource]s.
    #[asset(key = "game-settings")]
    pub game_settings: Handle<GameSettings>,

    #[asset(key = "game-levels")]
    pub levels: Handle<GameLevelSettingsCollection>,

    /// The content of this asset can be found as a [Resource].
    #[asset(key = "input-key-settings")]
    pub input_keys: Handle<InputKeySettings>,

    /// The content of this asset can be found as a [Resource].
    #[asset(key = "weapon-collection")]
    pub weapon_settings: Handle<WeaponCollection>,

    /// The content of this asset can be found as a [Resource].
    #[asset(key = "ammonition-depot")]
    pub ammonition_settings: Handle<AmmonitionDepot>,

    /// The content of this asset can be found as a [Resource].
    #[asset(key = "asteroid-pool-collection")]
    pub asteroid_pool_settings: Handle<AsteroidPoolCollection>,

    /****************
     * spritesheets *
     ****************/
    #[asset(key = "player-sheet")]
    pub player_spritesheet_handle: Handle<SpriteSheetAsset>,

    #[asset(key = "flames-sheet")]
    pub flames_spritesheet_handle: Handle<SpriteSheetAsset>,

    /// The content of this asset can be found as a [Resource].
    #[asset(key = "ammonition-texture-collection")]
    pub ammonition_texture_collection_handle: Handle<AmmonitionTextureCollection>,

    /// The content of this asset can be found as a [Resource].
    #[asset(key = "asteroid-texture-collection")]
    pub asteroid_texture_collection_handle: Handle<AsteroidTextureCollection>,
}

#[derive(SystemParam)]
pub struct PlayerSpriteSheet<'w> {
    game_assets: Res<'w, GameAssets>,
    spritesheet_assets: Res<'w, Assets<SpriteSheetAsset>>,
}

impl<'w> PlayerSpriteSheet<'w> {
    pub fn spritesheet(&self) -> &SpriteSheetAsset {
        self.spritesheet_assets
            .get(self.game_assets.player_spritesheet_handle.id())
            .unwrap()
    }
}

#[derive(SystemParam)]
pub struct FlameSpriteSheet<'w> {
    game_assets: Res<'w, GameAssets>,
    spritesheet_assets: Res<'w, Assets<SpriteSheetAsset>>,
}

impl<'w> FlameSpriteSheet<'w> {
    pub fn spritesheet(&self) -> &SpriteSheetAsset {
        self.spritesheet_assets
            .get(self.game_assets.flames_spritesheet_handle.id())
            .unwrap()
    }
}
