use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

/// Loaded directly as a [Resource] by [bevy_asset_loader].
#[derive(AssetCollection, Resource, Debug)]
pub struct StateBackgrounds {
    /// The individual fields of this asset can be found as individual [Resource]s.
    #[asset(key = "background-main-menu")]
    pub main_menu: Handle<Image>,

    #[asset(key = "background-game-over")]
    pub game_over: Handle<Image>,

    #[asset(key = "background-game-won")]
    pub game_won: Handle<Image>,
}
